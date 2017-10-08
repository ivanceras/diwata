use syn;
use quote;


pub fn impl_from_dao(ast: &syn::MacroInput) -> quote::Tokens {
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
            _ => panic!("Only struct is supported for #[derive(FromDao)]"),
        },
        syn::Body::Enum(_) => panic!("#[derive(FromDao)] can only be used with structs"),
    };
    let from_fields: Vec<quote::Tokens> = fields
        .iter()
        .map(|&(field, _ty)| {
            quote!{ #field: dao.get(stringify!(#field)).unwrap(),}
        })
        .collect();

    quote! {
        impl FromDao for  #name {

            fn from_dao(dao: &dao::Dao) -> Self {
                #name {
                    #(#from_fields)*
                }

            }
        }
    }
}


pub fn impl_to_dao(ast: &syn::MacroInput) -> quote::Tokens {
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
            _ => panic!("Only struct is supported for #[derive(ToDao)]"),
        },
        syn::Body::Enum(_) => panic!("#[derive(ToDao)] can only be used with structs"),
    };
    let from_fields: &Vec<quote::Tokens> = &fields
        .iter()
        .map(|&(field, _ty)| {
            quote!{ dao.insert(stringify!(#field), &self.#field);}
        })
        .collect();

    quote! {
        impl ToDao for  #name {

            fn to_dao(&self) -> dao::Dao {
                let mut dao = dao::Dao::new();
                #(#from_fields)*
                dao
            }
        }

    }
}

#[cfg(test)]
mod tests {}
