use crate::schema::{schema_type::SchemaTypes, SchemaField, TypeMapRef};

pub trait SchemableType {
    fn schema_type() -> SchemaTypes;
    fn type_name() -> String;
    fn type_fields(type_map: TypeMapRef) -> Vec<SchemaField>;
}

impl<T: SchemableType> SchemableType for Option<T> {
    #[inline]
    fn schema_type() -> SchemaTypes {
        T::schema_type()
    }

    #[inline]
    fn type_name() -> String {
        T::type_name()
    }

    #[inline]
    fn type_fields(type_map: TypeMapRef) -> Vec<SchemaField> {
        T::type_fields(type_map)
    }
}

impl<T: SchemableType> SchemableType for Vec<T> {
    #[inline]
    fn schema_type() -> SchemaTypes {
        T::schema_type()
    }

    #[inline]
    fn type_name() -> String {
        T::type_name()
    }

    #[inline]
    fn type_fields(type_map: TypeMapRef) -> Vec<SchemaField> {
        T::type_fields(type_map)
    }
}

impl<Ok: SchemableType, Err> SchemableType for Result<Ok, Err> {
    #[inline]
    fn schema_type() -> SchemaTypes {
        Ok::schema_type()
    }

    #[inline]
    fn type_name() -> String {
        Ok::type_name()
    }

    #[inline]
    fn type_fields(type_map: TypeMapRef) -> Vec<SchemaField> {
        Ok::type_fields(type_map)
    }
}
