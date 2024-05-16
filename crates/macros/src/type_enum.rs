use proc_macro::TokenStream;
use syn::Variant;

#[derive(Debug)]
struct EnumVariant {
    ident: syn::Ident,
    expr: Option<syn::Expr>,
}

pub fn implement(ident: syn::Ident, enum_variants: Vec<Variant>) -> TokenStream {
    let mut properties = Vec::new();

    for variant in enum_variants {
        match variant.fields {
            syn::Fields::Named(_) => {
                panic!("Named fields in enum is not supported");
            }
            syn::Fields::Unnamed(_) => {
                panic!("Unnamed fields in enum is not supported");
            }
            _ => {}
        }

        let field = EnumVariant {
            ident: variant.ident.clone(),
            expr: {
                if variant.discriminant.is_some() {
                    let discriminant = variant.discriminant.unwrap();
                    let (_, expr) = discriminant;

                    match expr {
                        syn::Expr::Lit(_) => {}
                        _ => panic!("Only literal expressions are supported in enum"),
                    }

                    Some(expr)
                } else {
                    None
                }
            },
        };

        properties.push(field);
    }

    let it_is_expr_enum = properties.iter().all(|p| p.expr.is_some());

    let mut fields = Vec::new();

    for property in properties {
        let ident = property.ident;

        let mut field_code = quote::quote! {};

        if it_is_expr_enum {
            let expr = property.expr.unwrap();
            let (str_value, ty, format) = match expr {
                syn::Expr::Lit(expr) => match expr.lit {
                    syn::Lit::Str(lit_str) => {
                        let str_value = lit_str.value();

                        (str_value, "string", "type")
                    }
                    syn::Lit::Char(lit_char) => {
                        let str_value = lit_char.value().to_string();

                        (str_value, "char", "type")
                    }
                    syn::Lit::Int(lit_int) => {
                        let str_value = lit_int.base10_digits().to_string();

                        (str_value, "integer", "int64")
                    }
                    syn::Lit::Float(lit_float) => {
                        let str_value = lit_float.base10_digits().to_string();

                        (str_value, "float", "float64")
                    }
                    syn::Lit::Bool(lit_bool) => {
                        let str_value = lit_bool.value().to_string();

                        (str_value, "boolean", "type")
                    }
                    _ => panic!("Unsupported literal type"),
                },
                _ => panic!("Only literal expressions are supported in enum"),
            };

            field_code = quote::quote! {
                #field_code
                let ty = rpc::open_schema::schema::SchemaFieldType::from_str(#ty);
                let format = rpc::open_schema::schema::SchemaFieldFormat::from_str(#format);
                let value = Some(String::from(#str_value));
            };
        } else {
            let str_ident = ident.to_string();

            field_code = quote::quote! {
                #field_code
                let ty = rpc::open_schema::schema::SchemaFieldType::String;
                let format = rpc::open_schema::schema::SchemaFieldFormat::Type;
                let value = Some(String::from(#str_ident));
            };
        }

        field_code = quote::quote! {
            #field_code

            let mut field = rpc::open_schema::schema::SchemaField {
                name: String::from(stringify!(#ident)),
                rel: Some(rpc::open_schema::schema::SchemaFieldRel::Native {
                    ty,
                    format,
                }),
                value,
            };
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
                rpc::open_schema::schema::SchemaTypes::Enum
            }

            #[inline]
            fn type_name() -> String {
                String::from(#str_ident)
            }

            fn type_fields(
                _: rpc::open_schema::schema::TypeMapRef,
            ) -> Vec<rpc::open_schema::schema::SchemaField> {
                let mut fields = Vec::new();

                #(#fields)*

                fields
            }
        }

        impl rpc::open_schema::schema::FieldType for #ident {
            #[inline]
            fn field_type() -> rpc::open_schema::schema::SchemaFieldType {
                rpc::open_schema::schema::SchemaFieldType::Enum
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
                rpc::open_schema::schema::SchemaFieldRel::Enum {
                    name: String::from(#str_ident),
                }
            }

            #[inline]
            fn explore_type(
                field: &mut rpc::open_schema::schema::SchemaField,
                type_map: rpc::open_schema::schema::TypeMapRef,
            ) {
                field.rel = Some(
                    <#ident as rpc::open_schema::SchemableField>::get_rel_type(),
                );

                // Insert enum into type map
                rpc::open_schema::schema::insert_into_type_map_ref::<#ident>(
                    rpc::open_schema::schema::SchemaTypes::Enum,
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

                // Insert enum into type map
                rpc::open_schema::schema::insert_into_type_map_ref::<#ident>(
                    rpc::open_schema::schema::SchemaTypes::Enum,
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

                // Insert enum into type map
                rpc::open_schema::schema::insert_into_type_map_ref::<#ident>(
                    rpc::open_schema::schema::SchemaTypes::Enum,
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
