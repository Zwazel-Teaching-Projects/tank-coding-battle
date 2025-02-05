use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemEnum, ItemStruct};

pub fn generate(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the attribute as an enum definition.
    let message_enum = parse_macro_input!(attr as ItemEnum);
    let enum_ident = message_enum.ident.clone();

    // Parse the item as the container struct.
    let container = parse_macro_input!(item as ItemStruct);
    let container_ident = container.ident.clone();

    // Generate match arms for each enum variant.
    let arms = message_enum.variants.iter().map(|variant| {
    let variant_ident = &variant.ident;
    match &variant.fields {
        syn::Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
            // Create trigger type name by appending "Trigger" to the variant name.
            let trigger_ident = syn::Ident::new(&format!("{}Trigger", variant_ident), variant_ident.span());
            quote! {
                #enum_ident::#variant_ident(data) => {
                    commands.trigger(#trigger_ident { message: data.clone(), sender: self.sender.clone().unwrap() });
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

    // Generate the impl block for MessageContainer.
    let impl_block = quote! {
        impl #container_ident {
            pub fn trigger_message_received(&self, commands: &mut Commands) {
                match &self.message {
                    #(#arms),*
                }
            }
        }
    };

    // Re-emit the container struct, the enum definition, and the generated impl block.
    let expanded = quote! {
        #container

        #message_enum

        #impl_block
    };

    expanded.into()
}
