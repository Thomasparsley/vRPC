use serde::{Deserialize, Serialize};

use super::SchemaField;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SchemaTypes {
    Enum,
    Struct,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SchemaType {
    pub name: String,
    #[serde(rename = "type")]
    pub ty: SchemaTypes,
    pub fields: Vec<SchemaField>,
}

impl SchemaType {
    pub(crate) fn new(name: String, ty: SchemaTypes, fields: Vec<SchemaField>) -> Self {
        Self { name, ty, fields }
    }
}
