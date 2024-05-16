use std::fmt::Display;

use errs::{code::HttpCode, Error};

pub fn one_of_calls_failed() -> Error {
    Error::new(codes::RPC_CORE_ONE_OF_CALLS_FAILED, HttpCode::InternalServerError, None)
}

pub fn procedure_not_found() -> Error {
    Error::new(codes::RPC_CORE_PROCEDURE_NOT_FOUND, HttpCode::NotFound, None)
}

pub fn empty_call_args() -> Error {
    Error::new(codes::RPC_CORE_EMPTY_CALL_ARGS, HttpCode::BadRequest, None)
}

pub fn unparsable_call_args<D: Display>(detail: D) -> Error {
    Error::new(
        codes::RPC_CORE_UNPARSABLE_CALL_ARGS,
        HttpCode::BadRequest,
        Some(detail.to_string()),
    )
}

pub fn injector_not_found() -> Error {
    Error::new(codes::RPC_CORE_INJECTOR_NOT_FOUND, HttpCode::InternalServerError, None)
}

pub mod codes {
    pub const RPC_CORE_ONE_OF_CALLS_FAILED: &str = "RPC_CORE_ONE_OF_CALLS_FAILED";
    pub const RPC_CORE_PROCEDURE_NOT_FOUND: &str = "RPC_CORE_PROCEDURE_NOT_FOUND";
    pub const RPC_CORE_EMPTY_CALL_ARGS: &str = "RPC_CORE_EMPTY_CALL_ARGS";
    pub const RPC_CORE_UNPARSABLE_CALL_ARGS: &str = "RPC_CORE_UNPARSABLE_CALL_ARGS";
    pub const RPC_CORE_INJECTOR_NOT_FOUND: &str = "RPC_CORE_INJECTOR_NOT_FOUND";
}
