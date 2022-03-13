extern crate proc_macro;

use dev_channel::is_legal;

use proc_macro::TokenStream;
use syn::parse::{self, Parse, ParseStream};
use syn::LitStr;
use syn::parse_macro_input;

#[allow(unused_imports)]
use quote::quote;

struct Enforce {
    message: LitStr,
}

impl Parse for Enforce {
    fn parse(input: ParseStream) -> parse::Result<Self> {
        let message = input.parse()?;
        Ok(Self {
            message,
        })
    }
}

#[proc_macro]
pub fn enforce(
    input: proc_macro::TokenStream,
) -> TokenStream {
    let parsed = parse_macro_input!(input as Enforce);

    let result = if !is_legal() {
        let message = parsed.message;
        quote! {
            compile_error!(#message);
        }
    } else {
        quote! { }
    };

    TokenStream::from(result)
}
