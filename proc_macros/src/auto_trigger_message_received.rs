use proc_macro::TokenStream;
use syn::{
    braced,
    parse::{Parse, ParseStream},
    parse_macro_input, Error, Ident, ItemEnum, ItemStruct, Result, Token,
};

/// Structure to parse the attribute input with two enums: target and message.
struct AutoTriggerArgs {
    target_enum: ItemEnum,
    message_enum: ItemEnum,
}

impl Parse for AutoTriggerArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        // Expect: target = { ... }
        let target_ident: Ident = input.parse()?;
        if target_ident != "target" {
            return Err(Error::new(target_ident.span(), "Expected 'target'"));
        }
        input.parse::<Token![=]>()?;
        let target_content;
        braced!(target_content in input);
        let target_enum: ItemEnum = target_content.parse()?;
        input.parse::<Token![,]>()?;

        // Expect: message = { ... }
        let message_ident: Ident = input.parse()?;
        if message_ident != "message" {
            return Err(Error::new(message_ident.span(), "Expected 'message'"));
        }
        input.parse::<Token![=]>()?;
        let message_content;
        braced!(message_content in input);
        let message_enum: ItemEnum = message_content.parse()?;

        Ok(AutoTriggerArgs {
            target_enum,
            message_enum,
        })
    }
}

pub fn generate(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the attribute input into our two enums.
    let args = parse_macro_input!(attr as AutoTriggerArgs);
    let mut target_enum = args.target_enum;
    let message_enum = args.message_enum;
    let container = parse_macro_input!(item as ItemStruct);
    let container_ident = &container.ident;
    let target_enum_ident = &target_enum.ident;
    let message_enum_ident = &message_enum.ident;

    // Collect valid target names from the target enum.
    let valid_targets: Vec<Ident> = target_enum
        .variants
        .iter()
        .map(|v| v.ident.clone())
        .collect();
    let valid_target_names: Vec<String> = valid_targets.iter().map(|id| id.to_string()).collect();

    // Create a mapping from each variant to an optional function name.
    // For each variant, remove any attribute whose simple identifier starts with "get_".
    let target_mapping: Vec<(Ident, Option<String>)> = target_enum
        .variants
        .iter_mut()
        .map(|variant| {
            let variant_ident = variant.ident.clone();
            let mut func_name: Option<String> = None;
            // Retain only attributes that are not function-name markers.
            variant.attrs.retain(|attr| {
                if let syn::Meta::Path(path) = &attr.meta {
                    if let Some(ident) = path.get_ident() {
                        let ident_str = ident.to_string();
                        if ident_str.starts_with("get_") {
                            func_name = Some(ident_str);
                            return false; // remove this attribute from the variant
                        }
                    }
                }
                true
            });
            (variant_ident, func_name)
        })
        .collect();

    // 2. Build the match expression for target function calls.
    // For each target variant, if a function name was specified (e.g. "get_players_in_lobby"),
    // generate a match arm that calls: lobby_management.get_players_in_lobby(lobby_management_arg);
    let target_function_match = {
        let target_function_arms = target_mapping.iter().map(|(variant_ident, func_opt)| {
            if let Some(func_name) = func_opt {
                // Convert the function name string into an identifier.
                let func_ident = syn::Ident::new(&func_name, variant_ident.span());
                quote::quote! {
                    #target_enum_ident::#variant_ident => {
                        // The following call is generated at compile time.
                        // Adjust "lobby_management" and "lobby_management_arg" as needed.
                        lobby_management.#func_ident(lobby_management_arg);
                    }
                }
            } else {
                quote::quote! {
                    #target_enum_ident::#variant_ident => { },
                }
            }
        });
        quote::quote! {
            match self.target {
                #(#target_function_arms)*
            }
        }
    };

    // 3. Process the message enum variants as before, generating match arms for each variant.
    let arms = message_enum.variants.iter().map(|variant| {
        let variant_ident = &variant.ident;
        let trigger_ident = syn::Ident::new(&format!("{}Trigger", variant_ident), variant_ident.span());
        // Extract allowed target attributes from the variant (already done earlier)
        let mut allowed_targets: Vec<Ident> = variant
            .attrs
            .iter()
            .filter_map(|attr| {
                if let syn::Meta::Path(path) = &attr.meta {
                    if let Some(ident) = path.get_ident() {
                        if valid_target_names.contains(&ident.to_string()) {
                            return Some(ident.clone());
                        }
                    }
                }
                None
            })
            .collect();
        if allowed_targets.is_empty() {
            allowed_targets = valid_targets.clone();
        }
        let allowed_array = quote::quote! {
            [ #( #target_enum_ident::#allowed_targets ),* ]
        };
        match &variant.fields {
            syn::Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                quote::quote! {
                    #message_enum_ident::#variant_ident(data) => {
                        if !#allowed_array.contains(&self.target) {
                            eprintln!(
                                "Error: Message variant {} cannot be sent to target {:?}",
                                stringify!(#variant_ident),
                                self.target
                            );
                            return Err(format!(
                                "Invalid target for message variant {}",
                                stringify!(#variant_ident)
                            ));
                        }
                        // Insert the target-specific function call.
                        #target_function_match
                        /* if targets.is_empty() {
                            commands.trigger(#trigger_ident { message: data.clone(), sender: self.sender.clone().unwrap() });
                        } else {
                            commands.trigger_targets(#trigger_ident { message: data.clone(), sender: self.sender.clone().unwrap() }, targets);
                        } */
                        Ok(())
                    }
                }
            },
            _ => {
                return Error::new_spanned(
                    variant,
                    "Each enum variant must be a tuple with exactly one field"
                )
                .to_compile_error();
            }
        }
    });

    // 4. Generate the final impl block including our new target function call.
    let impl_block = quote::quote! {
        impl #container_ident {
            pub fn trigger_message_received(&self, commands: &mut Commands, lobby_management: &LobbyManagementSystemParam, lobby_management_arg: LobbyManagementArgument) -> Result<(), String> {
                match &self.message {
                    #(#arms),*
                }
            }
        }
    };

    // 5. Re-emit everything.
    let expanded = quote::quote! {
        #container

        #target_enum

        #message_enum

        #impl_block
    };

    expanded.into()
}
