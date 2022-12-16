extern crate proc_macro;
extern crate syn;
extern crate quote;

use proc_macro::TokenStream;
use syn::parse::{Parse, ParseStream, Result};
use syn::{parse_macro_input, Ident, Token, LitStr};
use quote::quote;

struct StaticInclude {
    name: Ident,
    filepath: String,
}

impl Parse for StaticInclude {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        input.parse::<Token![=]>()?;
        let filepath: String = input.parse::<LitStr>()?.value();
        Ok(StaticInclude{ name, filepath })
    }
}

#[proc_macro]
pub fn progmem_include_bytes(tokens: TokenStream) -> TokenStream {
    let StaticInclude{ name, filepath } = parse_macro_input!(tokens as StaticInclude);

    let data: Vec<u8> = std::fs::read(&filepath).expect(&format!("File {:?} could not be read", filepath));
    let len = data.len();
    TokenStream::from(quote! {
        #[link_section = ".progmem.data"]
        static #name: [u8; #len] = [#(#data),*];
    })
}
