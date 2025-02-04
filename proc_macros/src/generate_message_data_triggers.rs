use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Fields, ItemEnum};

pub fn generate(item: TokenStream) -> TokenStream {
    // Parse the incoming enum
    let input = parse_macro_input!(item as ItemEnum);

    // Prepare a cauldron for our newly forged structs
    let mut generated_structs = Vec::new();

    // For each variant in the enum, perform the unholy transformation
    for variant in input.variants.iter() {
        let variant_name = &variant.ident;
        // Create the new struct name by appending "Trigger" to the variant name
        let trigger_name =
            syn::Ident::new(&format!("{}Trigger", variant_name), variant_name.span());

        // Ensure the variant is a tuple with exactly one field
        match &variant.fields {
            Fields::Unnamed(ref fields) if fields.unnamed.len() == 1 => {
                let field_type = &fields.unnamed.first().unwrap().ty;
                // Forge the new struct with the required derives
                let generated = quote! {
                    #[derive(Event, Debug, Deref, DerefMut)]
                    pub struct #trigger_name(pub #field_type);
                };
                generated_structs.push(generated);
            }
            _ => {
                // If the variant is not a single-field tuple, the ritual fails.
                return syn::Error::new_spanned(
                    variant,
                    "Each enum variant must be a tuple with exactly one field.",
                )
                .to_compile_error()
                .into();
            }
        }
    }

    // Combine the original enum with our newly forged structs.
    let expanded = quote! {
        #input
        #(#generated_structs)*
    };

    TokenStream::from(expanded)
}
