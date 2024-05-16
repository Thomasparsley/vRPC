use serde::{Deserialize, Serialize};

use super::schema_field_rel::SchemaFieldRel;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SchemaField {
    pub name: String,
    pub rel: Option<SchemaFieldRel>,
    /// The value of the field, for enums
    pub value: Option<String>,
}
