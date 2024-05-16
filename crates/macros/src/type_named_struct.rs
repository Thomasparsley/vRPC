use proc_macro::TokenStream;
use syn::{punctuated::Punctuated, token::Comma, Field};

use convert_case::{Case, Casing};

use crate::rename;

/// Struct that represents a field of a struct
struct StructField {
    /// Name of the field
    ident: syn::Ident,
    /// Type of the field
    ty: syn::Type,
}

pub fn implement(
    ident: syn::Ident,
    struct_fields: Punctuated<Field, Comma>,
    rename_all: rename::RenameType,
) -> TokenStream {
    // List of all properties of the struct
    let mut properties = Vec::new();

    // Iterate over all fields of the struct
    for field in struct_fields {
        // let vis = &field.vis;

        // if field is private, skip it
        /* if let syn::Visibility::Inherited = vis {
            continue;
        }

        // if field is restricted, skip it
        if let syn::Visibility::Restricted(_) = vis {
            continue;
        } */

        let ident = field.ident.as_ref().unwrap();

        let field = StructField {
            ident: ident.clone(),
            ty: field.ty.clone(),
        };

        properties.push(field);
    }

    let mut fields = Vec::new();

    // Iterate over all properties of the struct
    for property in properties {
        // Get name of the field
        let ident = property.ident;
        let mut str_ident = ident.to_string();

        match rename_all {
            rename::RenameType::CamelCase => {
                str_ident = str_ident.to_case(Case::Camel);
            }
            _ => {}
        };

        // Get type of the field
        let ty = property.ty;

        let mut field_code = quote::quote! {
            let mut field = rpc::open_schema::schema::SchemaField {
                name: String::from(#str_ident),
                rel: None,
                value: None,
            };
        };

        field_code = quote::quote! {
            #field_code

            <#ty as rpc::open_schema::SchemableField>::explore_type(&mut field, type_map.clone());
        };

        field_code = quote::quote! {
            #field_code
            fields.push(field);
        };

        fields.push(field_code);
    }

    let str_ident = ident.to_string();

    let gen = quote::quote! {
        impl rpc::open_schema::SchemableType for #ident {
            #[inline]
            fn schema_type() -> rpc::open_schema::schema::SchemaTypes {
                rpc::open_schema::schema::SchemaTypes::Struct
            }

            #[inline]
            fn type_name() -> String {
                String::from(#str_ident)
            }

            fn type_fields(
                type_map: rpc::open_schema::schema::TypeMapRef,
            ) -> Vec<rpc::open_schema::schema::SchemaField> {
                let mut fields = Vec::new();

                #(#fields)*

                fields
            }
        }

        impl rpc::open_schema::schema::FieldType for #ident {
            #[inline]
            fn field_type() -> rpc::open_schema::schema::SchemaFieldType {
                rpc::open_schema::schema::SchemaFieldType::Struct
            }
        }

        impl rpc::open_schema::schema::FieldFormat for #ident {
            #[inline]
            fn field_format() -> rpc::open_schema::schema::SchemaFieldFormat {
                rpc::open_schema::schema::SchemaFieldFormat::Type
            }
        }

        impl rpc::open_schema::SchemableField for #ident {
            #[inline]
            fn get_rel_type() -> rpc::open_schema::schema::SchemaFieldRel {
                rpc::open_schema::schema::SchemaFieldRel::Struct {
                    name: String::from(#str_ident),
                }
            }

            #[inline]
            fn explore_type(
                field: &mut rpc::open_schema::schema::SchemaField,
                type_map: rpc::open_schema::schema::TypeMapRef,
            ) {
                field.rel = Some(
                    <#ident as rpc::open_schema::SchemableField>::get_rel_type()
                );

                // Insert struct into type map
                rpc::open_schema::schema::insert_into_type_map_ref::<#ident>(
                    rpc::open_schema::schema::SchemaTypes::Struct,
                    type_map.clone(),
                );
            }
        }

        impl rpc::open_schema::SchemableParams for #ident {
            fn apply_schema(
                proc: &mut rpc::open_schema::schema::SchemaProcedure,
                type_map: rpc::open_schema::schema::TypeMapRef,
            ) {
                proc.params = Some(
                    <#ident as rpc::open_schema::SchemableField>::get_rel_type(),
                );

                // Insert struct into type map
                rpc::open_schema::schema::insert_into_type_map_ref::<#ident>(
                    rpc::open_schema::schema::SchemaTypes::Struct,
                    type_map.clone(),
                );
            }
        }

        impl rpc::open_schema::SchemableResult for #ident {
            fn apply_schema(
                proc: &mut rpc::open_schema::schema::SchemaProcedure,
                type_map: rpc::open_schema::schema::TypeMapRef,
            ) {
                proc.result = Some(
                    <#ident as rpc::open_schema::SchemableField>::get_rel_type(),
                );

                // Insert struct into type map
                rpc::open_schema::schema::insert_into_type_map_ref::<#ident>(
                    rpc::open_schema::schema::SchemaTypes::Struct,
                    type_map.clone(),
                );
            }
        }

        impl rpc::Responder for #ident {
            #[inline]
            fn into_json(self, call_key: rpc::call::CallKey) -> rpc::json::JsonValue {
                rpc::procedure::response::ProcedureResponse::result(call_key, self).into()
            }
        }
    };

    gen.into()
}
