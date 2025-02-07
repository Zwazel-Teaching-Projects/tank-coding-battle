use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    braced,
    parse::{Parse, ParseStream, Parser},
    parse_macro_input,
    punctuated::Punctuated,
    Attribute, DeriveInput, Ident, ItemEnum, Token,
};

// Structure to capture our unholy macro arguments.
struct AutoTriggerArgs {
    target_enum: ItemEnum,
    message_enum: ItemEnum,
}

impl Parse for AutoTriggerArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // Parse: target = { ... }
        let target_ident: Ident = input.parse()?;
        if target_ident != "target" {
            return Err(syn::Error::new(target_ident.span(), "Expected 'target'"));
        }
        input.parse::<Token![=]>()?;
        let content;
        braced!(content in input);
        let target_enum: ItemEnum = content.parse()?;
        input.parse::<Token![,]>()?;
        // Parse: message = { ... }
        let message_ident: Ident = input.parse()?;
        if message_ident != "message" {
            return Err(syn::Error::new(message_ident.span(), "Expected 'message'"));
        }
        input.parse::<Token![=]>()?;
        let content_msg;
        braced!(content_msg in input);
        let message_enum: ItemEnum = content_msg.parse()?;
        Ok(AutoTriggerArgs {
            target_enum,
            message_enum,
        })
    }
}

// Helper function to extract the get_targets function name from an attribute.
// Example: #[get_targets(get_players_in_lobby_team)]
fn get_get_targets_fn(attrs: &[Attribute]) -> Option<Ident> {
    for attr in attrs {
        if attr.path().is_ident("get_targets") {
            // In syn 2.0, `meta` is a field, not a method.
            if let syn::Meta::List(meta_list) = &attr.meta {
                // Parse the tokens into a punctuated list of Meta items using parse_terminated.
                let nested = Punctuated::<syn::Meta, Token![,]>::parse_terminated
                    .parse2(meta_list.tokens.clone())
                    .ok()?;
                // Extract the first meta item if it is a simple path.
                if let Some(syn::Meta::Path(path)) = nested.into_iter().next() {
                    return path.get_ident().cloned();
                }
            }
        }
    }
    None
}

// Helper function to extract allowed targets from a message enum variant attribute.
// Example: #[target(ServerOnly)] or #[target(Client, Team)]
fn get_allowed_targets(attrs: &[Attribute]) -> Option<Vec<Ident>> {
    for attr in attrs {
        if attr.path().is_ident("target") {
            // Use parse_terminated to extract the inner tokens as a list of Meta items.
            if let syn::Meta::List(meta_list) = &attr.meta {
                let nested = Punctuated::<syn::Meta, Token![,]>::parse_terminated
                    .parse2(meta_list.tokens.clone())
                    .ok()?;
                let mut targets = Vec::new();
                for meta in nested {
                    if let syn::Meta::Path(path) = meta {
                        if let Some(ident) = path.get_ident() {
                            targets.push(ident.clone());
                        }
                    }
                }
                return Some(targets);
            }
        }
    }
    None
}

pub fn generate(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the vile macro arguments and the pitiful structure to be enhanced.
    let args = parse_macro_input!(attr as AutoTriggerArgs);
    let input_ast = parse_macro_input!(item as DeriveInput);

    // Capture the two cursed enums of our dark design.
    let target_enum = args.target_enum;
    let message_enum = args.message_enum;

    let target_enum_ident = &target_enum.ident;
    let message_enum_ident = &message_enum.ident;

    // Create cleaned copies by stripping our unrecognized attributes.
    // Remove #[get_targets(...)] from target enum variants.
    let mut cleaned_target_enum = target_enum.clone();
    for variant in cleaned_target_enum.variants.iter_mut() {
        variant
            .attrs
            .retain(|attr| !attr.path().is_ident("get_targets"));
    }

    // Remove #[target(...)] from message enum variants.
    let mut cleaned_message_enum = message_enum.clone();
    for variant in cleaned_message_enum.variants.iter_mut() {
        variant.attrs.retain(|attr| !attr.path().is_ident("target"));
    }

    // Forge match arms for each target variant to invoke its get_targets function.
    // For each variant, extract the get_targets function from the original attributes.
    let target_match_arms = target_enum.variants.iter().map(|variant| {
    let variant_ident = &variant.ident;
        // Extract the get_targets function using the original attributes.
        let get_targets_fn = get_get_targets_fn(&variant.attrs)
            .unwrap_or_else(|| format_ident!("undefined_get_targets"));
        quote! {
            #target_enum_ident::#variant_ident => lobby_management.#get_targets_fn(lobby_management_arg)
        }
    }).collect::<Vec<_>>();

    // Now, create a cleaned copy for emitting the enum without the get_targets attributes.
    let mut cleaned_target_enum = target_enum.clone();
    for variant in cleaned_target_enum.variants.iter_mut() {
        variant
            .attrs
            .retain(|attr| !attr.path().is_ident("get_targets"));
    }

    // For each message variant, craft its dreadful trigger arm.
    let mut message_match_arms = Vec::new();
    for variant in cleaned_message_enum.variants.iter() {
        let variant_ident = &variant.ident;
        // Name the trigger struct for this variant.
        let trigger_struct_ident = format_ident!("{}Trigger", variant_ident);
        // Retrieve allowed targets from the original variant attributes.
        let allowed_targets = get_allowed_targets(&variant.attrs);
        let target_check = if let Some(targets) = allowed_targets {
            // Build a pattern like: TargetEnum::A | TargetEnum::B | ...
            let allowed_patterns = targets.iter().map(|t| {
                quote! { #target_enum_ident::#t }
            });
            quote! {
                if !matches!(self.target, #( #allowed_patterns )|* ) {
                    return Err(concat!("Invalid target for ", stringify!(#variant_ident)).to_string());
                }
            }
        } else {
            quote! {}
        };

        message_match_arms.push(quote! {
            #message_enum_ident::#variant_ident(data) => {
                // Validate that the target is proper.
                #target_check
                // Determine recipients using the designated get_targets function.
                let targets = match self.target {
                    #( #target_match_arms , )*
                }?;
                // If no targets are found, trigger without specific recipients.
                if targets.is_empty() {
                    commands.trigger(#trigger_struct_ident {
                        message: data.clone(),
                        sender: self.sender.clone().unwrap()
                    });
                } else {
                    // Otherwise, trigger for the designated targets.
                    commands.trigger_targets(#trigger_struct_ident {
                        message: data.clone(),
                        sender: self.sender.clone().unwrap()
                    }, targets);
                }
            }
        });
    }

    // Extract the identifier of the pitiful structure.
    let struct_ident = &input_ast.ident;
    // Generate the infernal implementation of trigger_message_received.
    let generated_impl = quote! {
        impl #struct_ident {
            // Invoke this function to trigger the message upon receiving.
            pub fn trigger_message_received(
                &self,
                commands: &mut Commands,
                lobby_management: &LobbyManagementSystemParam,
                lobby_management_arg: LobbyManagementArgument
            ) -> Result<(), String> {
                match &self.message {
                    #( #message_match_arms , )*
                }
                Ok(())
            }
        }
    };

    // Assemble our unholy creation using the cleaned enums.
    let output = quote! {
        // Generated target enum – the first pillar of our dark design.
        #cleaned_target_enum

        // Generated message enum – the second pillar of our dark scheme.
        #cleaned_message_enum

        // The original structure, now empowered by eldritch forces.
        #input_ast

        // The infernal implementation of trigger_message_received.
        #generated_impl
    };
    output.into()
}
