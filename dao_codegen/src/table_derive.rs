
use syn;
use quote;


pub fn impl_to_table(ast: &syn::MacroInput) -> quote::Tokens {
    let name = &ast.ident;
    quote! {
        impl ToTable for  #name {

            fn to_table() -> dao::Table {
                dao::Table{
                    name: stringify!(#name).to_lowercase().into(),
                    schema: None,
                    alias: None,
                }
            }
        }
    }
}
