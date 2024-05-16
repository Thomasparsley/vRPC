use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProcedureType {
    Query,
    Mutation,
}

pub trait ProcedureTypeTrait {
    fn procedure_type() -> ProcedureType;
}
