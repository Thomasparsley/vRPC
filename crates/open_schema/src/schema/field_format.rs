use std::sync::Arc;

use super::SchemaFieldFormat;

pub trait FieldFormat {
    fn field_format() -> SchemaFieldFormat;
}

macro_rules! impl_field_format {
    ($({ $t:ty, $e:ident }),*) => {
        $(
            impl FieldFormat for $t {
                #[inline]
                fn field_format() -> SchemaFieldFormat {
                    SchemaFieldFormat::$e
                }
            }
        )*
    };
}

impl_field_format!(
    // Ints
    { i8, Int8 },
    { i16, Int16 },
    { i32, Int32 },
    { i64, Int64 },
    { i128, Int128 },
    { u8, UInt8 },
    { u16, UInt16 },
    { u32, UInt32 },
    { u64, UInt64 },
    { u128, UInt128 },
    // Default size ints
    { isize, Isize },
    { usize, Usize },

    // Floats
    { f32, Float32 },
    { f64, Float64 },

    // Time
    { std::time::Duration, UInt64 },
    { time::Date, Date },
    { time::OffsetDateTime, DateTime }
);

macro_rules! impl_field_format {
    ($($t:ty),*) => {
        $(
            impl FieldFormat for $t {
                #[inline]
                fn field_format() -> SchemaFieldFormat {
                    SchemaFieldFormat::Type
                }
            }
        )*
    };
}

impl_field_format!(bool, String, str, &str, Arc<str>, serde_json::Value, errs::Error);

impl<T: FieldFormat> FieldFormat for Vec<T> {
    #[inline]
    fn field_format() -> SchemaFieldFormat {
        T::field_format()
    }
}

impl<T: FieldFormat> FieldFormat for Option<T> {
    #[inline]
    fn field_format() -> SchemaFieldFormat {
        T::field_format()
    }
}

impl<Ok: FieldFormat, Err> FieldFormat for Result<Ok, Err> {
    #[inline]
    fn field_format() -> SchemaFieldFormat {
        Ok::field_format()
    }
}

impl<K, V> FieldFormat for std::collections::HashMap<K, V> {
    #[inline]
    fn field_format() -> SchemaFieldFormat {
        SchemaFieldFormat::HashMap
    }
}

impl<K, V> FieldFormat for std::collections::BTreeMap<K, V> {
    #[inline]
    fn field_format() -> SchemaFieldFormat {
        SchemaFieldFormat::BTreeMap
    }
}
