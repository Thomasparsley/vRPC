use serde::{Deserialize, Serialize};

use super::{SchemaFieldFormat, SchemaFieldType};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "variant")]
pub enum SchemaFieldRel {
    /// Native types
    Native {
        #[serde(rename = "type")]
        ty: SchemaFieldType,
        format: SchemaFieldFormat,
    },
    /// Array type
    Array { value: Box<SchemaFieldRel> },
    /// Nullable type
    Nullable { value: Box<SchemaFieldRel> },
    /// Custom type
    Type { name: String, ty: Box<SchemaFieldRel> },
    /// Struct type
    Struct { name: String },
    /// Enum type
    Enum { name: String },
    /// Map type - key and value are boxed
    Map {
        key: Box<SchemaFieldRel>,
        value: Box<SchemaFieldRel>,
    },
}
