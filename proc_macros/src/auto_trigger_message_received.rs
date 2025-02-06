use proc_macro::TokenStream;
use quote::quote;
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
    let target_enum = args.target_enum;
    let mut message_enum = args.message_enum;
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

    // For each variant in the message enum, generate a match arm.
    // Iterate over message enum variants mutably so we can remove the custom attributes.
    let arms = message_enum.variants.iter_mut().map(|variant| {
        // Extract allowed targets from variant attributes, and simultaneously remove them.
        let mut allowed_targets: Vec<Ident> = Vec::new();
        variant.attrs.retain(|attr| {
            if let syn::Meta::Path(path) = &attr.meta {
                if let Some(ident) = path.get_ident() {
                    if valid_target_names.contains(&ident.to_string()) {
                        allowed_targets.push(ident.clone());
                        return false; // remove this attribute from the variant
                    }
                }
            }
            true // keep all other attributes
        });
        
        // If no allowed target attribute is specified, allow all targets.
        if allowed_targets.is_empty() {
            allowed_targets = valid_targets.clone();
        }
        
        // Build an array of allowed targets, e.g. [MessageTarget::Team, MessageTarget::All, ...]
        let allowed_array = quote! {
            [ #( #target_enum_ident::#allowed_targets ),* ]
        };
        
        // Generate the match arm for this variant.
        match &variant.fields {
            syn::Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                let variant_ident = &variant.ident;
                let trigger_ident = syn::Ident::new(&format!("{}Trigger", variant_ident), variant_ident.span());
                quote! {
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
                        if targets.is_empty() {
                            commands.trigger(#trigger_ident { message: data.clone(), sender: self.sender.clone().unwrap() });
                        } else {
                            commands.trigger_targets(#trigger_ident { message: data.clone(), sender: self.sender.clone().unwrap() }, targets);
                        }
                        Ok(())
                    }
                }
            },
            _ => {
                return syn::Error::new_spanned(
                    variant,
                    "Each enum variant must be a tuple with exactly one field"
                )
                .to_compile_error();
            }
        }
    });

    // Generate the impl block that includes the new target-checking logic.
    let impl_block = quote! {
        impl #container_ident {
            pub fn trigger_message_received(&self, commands: &mut Commands, targets: Vec<Entity>) -> Result<(), String> {
                match &self.message {
                    #(#arms),*
                }
            }
        }
    };

    // Re-emit the container struct, the target enum, the message enum, and the impl block.
    let expanded = quote! {
        #container

        #target_enum

        #message_enum

        #impl_block
    };

    expanded.into()
}
