use std::sync::Arc;

use serde::Serialize;

use crate::{call::CallKey, json::JsonValue, procedure::response::ProcedureResponse};

pub trait Responder {
    fn into_json(self, call_key: CallKey) -> JsonValue;
}

impl Responder for () {
    fn into_json(self, call_key: CallKey) -> JsonValue {
        ProcedureResponse::empty(call_key).into()
    }
}

macro_rules! factory_responder {
    ($($type:ty), *) => {
        $(
            impl Responder for $type {
                fn into_json(self, call_key: CallKey) -> JsonValue {
                    ProcedureResponse::result(call_key, self).into()
                }
            }
        )*
    };
}

factory_responder!(
    bool,
    i8,
    i16,
    i32,
    i64,
    i128,
    isize,
    u8,
    u16,
    u32,
    u64,
    u128,
    usize,
    f32,
    f64,
    String,
    &str,
    Arc<str>
);

impl<T: Serialize> Responder for Option<T> {
    fn into_json(self, call_key: CallKey) -> JsonValue {
        match self {
            Some(result) => ProcedureResponse::result(call_key, result).into(),
            None => ProcedureResponse::empty(call_key).into(),
        }
    }
}

impl<Ok: Serialize, Err: Serialize> Responder for Result<Ok, Err> {
    fn into_json(self, call_key: CallKey) -> JsonValue {
        match self {
            Ok(result) => ProcedureResponse::result(call_key, result).into(),
            Err(error) => ProcedureResponse::error(call_key, error).into(),
        }
    }
}

impl<T: Serialize> Responder for Vec<T> {
    fn into_json(self, call_key: CallKey) -> JsonValue {
        ProcedureResponse::result(call_key, self).into()
    }
}
