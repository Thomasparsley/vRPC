use std::sync::Arc;

use super::SchemaFieldType;

pub trait FieldType {
    fn field_type() -> SchemaFieldType;
}

macro_rules! impl_field_type {
    ($e:ident, { $($t:ty),* }) => {
        $(
            impl FieldType for $t {
                #[inline]
                fn field_type() -> SchemaFieldType {
                    SchemaFieldType::$e
                }
            }
        )*
    };
}

impl_field_type!(Boolean, { bool });
impl_field_type!(String, { String, str, &str, Arc<str>, time::Date, time::OffsetDateTime });
impl_field_type!(Integer, { i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, std::time::Duration });
impl_field_type!(Float, { f32, f64 });
impl_field_type!(Object, { serde_json::Value });
impl_field_type!(Struct, { errs::Error });

impl<T: FieldType> FieldType for Vec<T> {
    #[inline]
    fn field_type() -> SchemaFieldType {
        T::field_type()
    }
}

impl<T: FieldType> FieldType for [T] {
    #[inline]
    fn field_type() -> SchemaFieldType {
        T::field_type()
    }
}

impl<T: FieldType + Default> FieldType for Option<T> {
    #[inline]
    fn field_type() -> SchemaFieldType {
        T::field_type()
    }
}

impl<Ok: FieldType, Err> FieldType for Result<Ok, Err> {
    #[inline]
    fn field_type() -> SchemaFieldType {
        Ok::field_type()
    }
}

impl<K, V> FieldType for std::collections::HashMap<K, V> {
    #[inline]
    fn field_type() -> SchemaFieldType {
        SchemaFieldType::Map
    }
}

impl<K, V> FieldType for std::collections::BTreeMap<K, V> {
    #[inline]
    fn field_type() -> SchemaFieldType {
        SchemaFieldType::Map
    }
}
