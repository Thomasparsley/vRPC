use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum SchemaFieldType {
    String,
    Char,
    Integer,
    Float,
    Boolean,
    Map,
    Struct,
    Enum,
    Object,
}

impl SchemaFieldType {
    pub fn from_str(value: &str) -> Self {
        match value {
            "string" => Self::String,
            "char" => Self::Char,
            "integer" => Self::Integer,
            "float" => Self::Float,
            "boolean" => Self::Boolean,
            "map" => Self::Map,
            "struct" => Self::Struct,
            "enum" => Self::Enum,
            "object" => Self::Object,
            _ => panic!("Unsupported type"),
        }
    }
}
