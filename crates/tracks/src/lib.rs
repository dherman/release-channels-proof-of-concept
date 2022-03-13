extern crate proc_macro;

use std::env;

use proc_macro::TokenStream;
use regex::Regex;
use semver::Version;
use syn::parse::{self, Parse, ParseStream};
use syn::LitStr;
use syn::parse_macro_input;

#[allow(unused_imports)]
use quote::quote;

struct UnlessMatches {
    pattern: LitStr,
}

impl Parse for UnlessMatches {
    fn parse(input: ParseStream) -> parse::Result<Self> {
        let pattern = input.parse()?;
        Ok(Self { pattern })
    }
}

struct IfEnv {
    key: LitStr,
}

impl Parse for IfEnv {
    fn parse(input: ParseStream) -> parse::Result<Self> {
        let key = input.parse()?;
        Ok(Self { key })
    }
}

#[proc_macro_attribute]
pub fn unless_matches(
    attr: TokenStream,
    item: TokenStream,
) -> TokenStream {
    let parsed = parse_macro_input!(attr as UnlessMatches);
    let pattern = Regex::new(&parsed.pattern.value()).unwrap();
    let version = Version::parse(&env::var("CARGO_PKG_VERSION").unwrap()).unwrap();

    if !pattern.is_match(version.pre.as_str()) {
        item
    } else {
        TokenStream::from(quote! { })
    }
}

#[proc_macro_attribute]
pub fn if_env(
    attr: TokenStream,
    item: TokenStream,
) -> TokenStream {
    let parsed = parse_macro_input!(attr as IfEnv);
    let key = parsed.key.value();

    if env::var(key).ok().filter(|value| value.trim().len() > 0).is_some() {
        item
    } else {
        TokenStream::from(quote! { })
    }
}
