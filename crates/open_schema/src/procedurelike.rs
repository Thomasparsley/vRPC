use crate::{
    schema::{procedure_type::ProcedureType, TypeMapRef},
    SchemaProcedure,
};

pub trait ProcedureLike {
    fn id(&self) -> usize;
    fn procedure_type(&self) -> ProcedureType;
    fn name(&self) -> &str;
    fn call_schema(
        &self,
        procedure_schema: SchemaProcedure,
        type_map: TypeMapRef,
    ) -> SchemaProcedure;
}

pub trait ProcedureTypeLike {
    fn into_schema(&self) -> ProcedureType;
}
