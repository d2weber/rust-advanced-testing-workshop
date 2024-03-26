use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{Attribute, ItemFn, Path};

#[proc_macro_attribute]
pub fn vanilla_test(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut test_fn: ItemFn = syn::parse(input).unwrap();
    if test_fn
        .attrs
        .iter()
        .find(|a| is_test_attribute(a))
        .is_some()
    {
        test_fn.to_token_stream()
    } else {
        quote!{
            #[test]
            #test_fn
        }
    }
    .into()
}

fn is_test_attribute(attr: &Attribute) -> bool {
    attr.path().is_ident("test")
}
