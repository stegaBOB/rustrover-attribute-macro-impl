use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{ItemImpl, parse_macro_input};

#[proc_macro_attribute]
pub fn impl_for(args: TokenStream, item: TokenStream) -> TokenStream {
    let mut impl_item = parse_macro_input!(item as ItemImpl);
    let args = parse_macro_input!(args as syn::Type);
    impl_item.self_ty = args.into();
    impl_item.to_token_stream().into()
}
