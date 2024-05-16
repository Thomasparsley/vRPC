use serde::Serialize;

use crate::{call::CallKey, json::JsonValue};

#[derive(Serialize)]
pub struct ProcedureResponse {
    pub key: CallKey,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ok: Option<JsonValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub err: Option<JsonValue>,
}

impl ProcedureResponse {
    fn init(key: CallKey, ok: Option<JsonValue>, err: Option<JsonValue>) -> Self {
        Self { key, ok, err }
    }

    /// Empty response
    pub fn empty(key: CallKey) -> Self {
        Self::init(key, None, None)
    }

    /// Response with result
    pub fn result<T>(key: CallKey, result: T) -> Self
    where
        T: Serialize,
    {
        let result = serde_json::to_value(result).unwrap();
        Self::init(key, Some(result), None)
    }

    /// Response with error
    pub fn error<T>(key: CallKey, error: T) -> Self
    where
        T: Serialize,
    {
        let error = serde_json::to_value(error).unwrap();
        Self::init(key, None, Some(error))
    }
}

impl From<ProcedureResponse> for JsonValue {
    fn from(call: ProcedureResponse) -> Self {
        serde_json::to_value(call).unwrap()
    }
}
