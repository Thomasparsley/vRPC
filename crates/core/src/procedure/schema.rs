use serde::Serialize;

use callbacks::BlockingCallback;

use rpc_openschema::{schema::TypeMapRef, SchemaProcedure, SchemableParams, SchemableResult};

use crate::from_request::FromRequest;

use super::procedureable::Procedureable;

pub type ProcedureSchemaService =
    Box<BlockingCallback<(SchemaProcedure, TypeMapRef), SchemaProcedure>>;

pub fn new_procedure_schema_service<F, Args>(_: F) -> ProcedureSchemaService
where
    F: Procedureable<Args>,
    Args: FromRequest + SchemableParams,
    F::Output: SchemableResult + Serialize,
{
    Box::new(|(mut procedure_schema, type_map)| {
        Args::apply_schema(&mut procedure_schema, type_map.clone());
        F::Output::apply_schema(&mut procedure_schema, type_map);

        procedure_schema
    })
}
