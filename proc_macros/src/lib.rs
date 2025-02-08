use proc_macro::TokenStream;

mod auto_trigger_message_received;
mod generate_message_data_triggers;

#[proc_macro_attribute]
pub fn generate_message_data_triggers(_attr: TokenStream, item: TokenStream) -> TokenStream {
    generate_message_data_triggers::generate(item)
}

#[proc_macro_attribute]
pub fn auto_trigger_message_received(attr: TokenStream, item: TokenStream) -> TokenStream {
    auto_trigger_message_received::generate(attr, item)
}
