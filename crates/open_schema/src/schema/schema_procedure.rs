use serde::{Deserialize, Serialize};

use crate::procedurelike::ProcedureLike;

use super::{procedure_type::ProcedureType, SchemaFieldRel, TypeMapRef};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SchemaProcedure {
    pub id: usize,
    #[serde(rename = "type")]
    pub ty: ProcedureType,
    pub path: String,
    pub name: String,
    pub params: Option<SchemaFieldRel>,
    pub result: Option<SchemaFieldRel>,
}

impl SchemaProcedure {
    fn new(
        id: usize,
        ty: ProcedureType,
        path: String,
        name: String,
        params: Option<SchemaFieldRel>,
        result: Option<SchemaFieldRel>,
    ) -> Self {
        Self {
            id,
            ty,
            path,
            name,
            params,
            result,
        }
    }

    pub fn from_app<P: ProcedureLike>(path: String, app_proc: P, type_map: TypeMapRef) -> Self {
        let schema = Self::new(
            app_proc.id(),
            app_proc.procedure_type(),
            path,
            app_proc.name().to_string(),
            None,
            None,
        );

        app_proc.call_schema(schema, type_map)
    }
}
