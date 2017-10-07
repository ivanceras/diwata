#![deny(warnings)]

extern crate dao;
extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

mod dao_derive;
mod table_derive;
mod column_derive;


use proc_macro::TokenStream;

#[proc_macro_derive(FromDao)]
pub fn from_dao(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_macro_input(&s).unwrap();
    let gen = dao_derive::impl_from_dao(&ast);
    gen.parse().unwrap()
}

#[proc_macro_derive(ToDao)]
pub fn to_dao(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_macro_input(&s).unwrap();
    let gen = dao_derive::impl_to_dao(&ast);
    gen.parse().unwrap()
}

#[proc_macro_derive(ToTable)]
pub fn to_table(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_macro_input(&s).unwrap();
    let gen = table_derive::impl_to_table(&ast);
    gen.parse().unwrap()
}

#[proc_macro_derive(ToColumns)]
pub fn to_columns(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_macro_input(&s).unwrap();
    let gen = column_derive::impl_to_columns(&ast);
    gen.parse().unwrap()
}
