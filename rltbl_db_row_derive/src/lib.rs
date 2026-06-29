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

    let fields = match &ast.data {
        syn::Data::Struct(data_struct) => &data_struct.fields,
        _ => panic!("Invalid data"),
    };
    let fields = match fields {
        syn::Fields::Named(fields) => fields,
        _ => panic!("Invalid fields"),
    };

    let mut field_names = vec![];
    for field in fields.named.iter() {
        let field_ident = field.ident.clone().unwrap();
        println!("FIELD IDENT: {field_ident}");
        field_names.push(field_ident.to_string());
    }
    println!("FIELD_NAMES: {field_names:#?}");

    let generated = quote! {
        impl Into<DbRow> for #name {
            fn into(self) -> DbRow {
                self.into_db_row()
            }
        }

        impl From<DbRow> for #name {
            fn from(value: DbRow) -> Self {
                #name::from_db_row(value)
            }
        }

        impl ConvertDbRow for #name {
            fn into_db_row(&self) -> DbRow {
                // TODO.
                DbRow::new()
            }

            fn from_db_row(value: DbRow) -> Self {
                rltbl_db::serde::from_db_row(&value).unwrap()
            }
        }
    };
    generated.into()
}
