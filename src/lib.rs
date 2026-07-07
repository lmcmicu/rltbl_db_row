use proc_macro::TokenStream;
use quote::quote;
use syn::{Data::Struct, DeriveInput, Fields::Named, Type::Path, TypePath};

#[proc_macro_derive(ConvertDbRow)]
pub fn convert_db_row_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate.
    let ast = syn::parse(input).expect("Could not parse input token stream");

    // Build the trait implementation.
    impl_convert_db_row(&ast)
}

fn impl_convert_db_row(ast: &DeriveInput) -> TokenStream {
    fn expand_type_path(path: &TypePath) -> String {
        path.path
            .segments
            .iter()
            .map(|seg| seg.ident.to_string())
            .collect::<Vec<_>>()
            .join("::")
    }

    let mut sources = vec![];
    let mut targets = vec![];
    match &ast.data {
        Struct(data_struct) => match &data_struct.fields {
            Named(fields) => {
                for field in fields.named.iter() {
                    let field_type = match &field.ty {
                        Path(path) => expand_type_path(path),
                        _ => panic!("Unsupported field type: {:?}", field.ty),
                    };
                    let (source_code, target_code) = {
                        let source_code = field.ident.clone().expect("No field ident");
                        let target_code = match field_type.as_str() {
                            "serde_json::Value" | "JsonValue" => {
                                quote! { DbValue::Json(self.#source_code) }
                            }
                            _ => {
                                quote! { self.#source_code }
                            }
                        };
                        (source_code, target_code)
                    };
                    sources.push(source_code);
                    targets.push(target_code);
                }
            }
            _ => panic!("Unupported data fields format"),
        },
        _ => panic!("Unsupported data format"),
    };
    let type_name = &ast.ident;

    let generated = quote! {
        impl Into<DbRow> for #type_name {
            fn into(self) -> DbRow {
                rltbl_db::db_row! {
                    #( stringify!(#sources) => #targets ),*
                }
            }
        }

        impl From<DbRow> for #type_name {
            fn from(value: DbRow) -> Self {
                rltbl_db::serde::from_db_row(&value).expect("Error deserializing row")
            }
        }
    };
    TokenStream::from(generated)
}
