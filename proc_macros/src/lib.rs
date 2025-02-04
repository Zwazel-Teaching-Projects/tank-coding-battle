use proc_macro::TokenStream;

mod generate_message_data_triggers;

#[proc_macro_attribute]
pub fn generate_message_data_triggers(_attr: TokenStream, item: TokenStream) -> TokenStream {
    generate_message_data_triggers::generate(item)
}
