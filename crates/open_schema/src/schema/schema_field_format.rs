use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum SchemaFieldFormat {
    /// Same as field type
    Type,
    /// Integer of isize type
    Isize,
    /// Integer of i128 type
    Int128,
    /// Integer of i64 type
    Int64,
    /// Integer of i32 type
    Int32,
    /// Integer of i16 type
    Int16,
    /// Integer of i8 type
    Int8,
    /// Integer of usize type
    Usize,
    /// Integer of u128 type
    UInt128,
    /// Integer of u64 type
    UInt64,
    /// Integer of u32 type
    UInt32,
    /// Integer of u16 type
    UInt16,
    /// Integer of u8 type
    UInt8,
    /// Float of f64 type
    Float64,
    /// Float of f32 type
    Float32,
    /// String of date type
    Date,
    /// String of date-time type
    DateTime,
    /// Map of hash map type
    HashMap,
    /// Map of btree map type
    BTreeMap,
}

impl SchemaFieldFormat {
    pub fn from_str(value: &str) -> Self {
        match value {
            "type" => Self::Type,
            "isize" => Self::Isize,
            "int128" => Self::Int128,
            "int64" => Self::Int64,
            "int32" => Self::Int32,
            "int16" => Self::Int16,
            "int8" => Self::Int8,
            "usize" => Self::Usize,
            "uint128" => Self::UInt128,
            "uint64" => Self::UInt64,
            "uint32" => Self::UInt32,
            "uint16" => Self::UInt16,
            "uint8" => Self::UInt8,
            "float64" => Self::Float64,
            "float32" => Self::Float32,
            "date" => Self::Date,
            "datetime" => Self::DateTime,
            "hashmap" => Self::HashMap,
            "btreemap" => Self::BTreeMap,
            _ => panic!("Unsupported format"),
        }
    }
}
