pub use crate::{
    schema::SchemaProcedure,
    schemable::{
        schemable_field::SchemableField, schemable_params::SchemableParams, schemable_result::SchemableResult,
        schemable_type::SchemableType,
    },
};

pub use rpc_macros::Schemable;

pub mod applike;
pub mod procedurelike;
pub mod schema;

pub const SCHEMA_VERSION: &'static str = "2.0.0";
pub const SCHEMA_PATH_SEPARATOR: &'static str = "/";

mod schemable;
