
use syn;
use quote;
use proc_macro::TokenStream;


pub fn impl_to_table(ast: &syn::MacroInput) -> quote::Tokens {
    let name = &ast.ident;
    quote! {
        impl ToTable for  #name {

            fn to_table(&self) -> dao::Table {
                dao::Table{
                    name: stringify!(#name).into(),
                    schema: None,
                    alias: None,
                }
            }
        }
    }
}
