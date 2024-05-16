use proc_macro::TokenStream;

pub(crate) mod rename;
pub(crate) mod type_enum;
pub(crate) mod type_named_struct;
// pub(crate) mod type_unnamed_struct;

/// Procedural macro 'Schemable' that implements multiple traits for a struct and enum
/// that is used to build schema for client-server communication.
///
/// # Example
///
/// ```ignore
/// #[derive(rpc::Schemable, serde::Serialize, serde::Deserialize)]
/// struct Foo {
///    pub bar: String,
///    skip: usize,
///    pub baz: Option<u8>,
/// }
///
/// #[derive(rpc::Schemable, serde::Serialize, serde::Deserialize)]
/// enum FooEnum {
///    Bar,
///    Baz,
/// }
///
/// #[derive(rpc::Schemable, serde::Serialize, serde::Deserialize)]
/// enum FooEnum2 {
///    Bar = 1,
///    Baz = 2,
/// }
/// ```
#[proc_macro_derive(Schemable)]
pub fn derive_schemable(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let ident = &ast.ident;

    let rename_all = rename::find_type_in_attr(ast.attrs, "rename_all");

    match ast.data {
        syn::Data::Struct(syn::DataStruct {
            fields: syn::Fields::Named(syn::FieldsNamed { named, .. }),
            ..
        }) => type_named_struct::implement(ident.clone(), named.clone(), rename_all),
        // syn::Data::Struct(syn::DataStruct {
        //     fields: syn::Fields::Unnamed(syn::FieldsUnnamed { unnamed, .. }),
        //     ..
        // }) => {
        //     type_unnamed_struct::implement(ident.clone(), unnamed.clone());
        //     panic!("Unnamed fields are not supported")
        // }
        syn::Data::Enum(enum_item) => {
            let enum_fields = enum_item.variants.into_iter().collect::<Vec<_>>();

            type_enum::implement(ident.clone(), enum_fields)
        }
        _ => panic!("Only named structs and enums are supported"),
    }
}

// let table_name = input.attrs.iter().find_map(|attr| {
//     if attr.path.is_ident("table_name") {
//         attr.parse_args::<syn::LitStr>().ok()
//     } else {
//         None
//     }
// }).expect("table_name attribute not found");
