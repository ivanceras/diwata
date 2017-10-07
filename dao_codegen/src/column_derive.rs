use syn;
use quote;


pub fn impl_to_columns(ast: &syn::MacroInput) -> quote::Tokens {
    let name = &ast.ident;
    let fields: Vec<(&syn::Ident, &syn::Ty)> = match ast.body {
        syn::Body::Struct(ref data) => match *data {
            syn::VariantData::Struct(ref fields) => fields
                .iter()
                .map(|f| {
                    let ident = f.ident.as_ref().unwrap();
                    let ty = &f.ty;
                    (ident, ty)
                })
                .collect::<Vec<_>>(),
            _ => panic!("Only struct is supported for #[derive(ToColumns)]"),
        },
        syn::Body::Enum(_) => panic!("#[derive(ToColumns)] can only be used with structs"),
    };
    let from_fields: Vec<quote::Tokens> = fields
        .iter()
        .map(|&(field, _ty)| {
            quote!{
                dao::Column{
                    name: stringify!(#field).into(),
                    table: Some(stringify!(#name).to_lowercase().into()),
                    alias: None,
                },
            }
        })
        .collect();

    quote! {
        impl ToColumns for  #name {

            fn to_columns() -> Vec<dao::Column> {
                vec![
                    #(#from_fields)*
                ]
            }
        }
    }
}
