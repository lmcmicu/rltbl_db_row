use proc_macro::TokenStream;
use quote::quote;
use syn::{Data::Struct, DeriveInput, Fields::Named};

#[proc_macro_derive(ConvertDbRow)]
pub fn convert_db_row_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate.
    let ast = syn::parse(input).expect("Could not parse input token stream");

    // Build the trait implementation.
    impl_convert_db_row(&ast)
}

fn impl_convert_db_row(ast: &DeriveInput) -> TokenStream {
    let type_name = &ast.ident;
    let named = match &ast.data {
        Struct(data_struct) => match &data_struct.fields {
            Named(fields) => fields
                .named
                .iter()
                .map(|f| f.ident.clone().unwrap())
                .collect::<Vec<_>>(),
            _ => panic!("Invalid fields"),
        },
        _ => panic!("Invalid data"),
    };

    let generated = quote! {
        impl Into<DbRow> for #type_name {
            fn into(self) -> DbRow {
                // TODO: Handle JSON columns specially.
                rltbl_db::db_row! {
                    #( stringify!(#named) => self.#named ),*
                }
            }
        }

        impl From<DbRow> for #type_name {
            fn from(value: DbRow) -> Self {
                rltbl_db::serde::from_db_row(&value).unwrap()
            }
        }
    };
    generated.into()
}
