use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn vanilla_test(args: TokenStream, item: TokenStream) -> TokenStream {
    return item
}
