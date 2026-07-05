use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

#[proc_macro_derive(ConvertDbRow)]
pub fn convert_db_row_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate.
    let ast = syn::parse(input).expect("Could not parse input token stream");

    // Build the trait implementation.
    impl_convert_db_row(&ast)
}

fn impl_convert_db_row(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;

    println!("DATA: {:#?}", ast.data);

    let fields = match &ast.data {
        syn::Data::Struct(data_struct) => &data_struct.fields,
        _ => panic!("Invalid data"),
    };
    let fields = match fields {
        syn::Fields::Named(fields) => fields,
        _ => panic!("Invalid fields"),
    };

    let named = fields
        .named
        .iter()
        .map(|f| f.ident.clone().unwrap())
        .collect::<Vec<_>>();

    let generated = quote! {
        impl Into<DbRow> for #name {
            fn into(self) -> DbRow {
                rltbl_db::db_row! {
                    #( stringify!(#named) => self.#named ),*
                }
            }
        }

        impl From<DbRow> for #name {
            fn from(value: DbRow) -> Self {
                rltbl_db::serde::from_db_row(&value).unwrap()
            }
        }
    };
    generated.into()
}
