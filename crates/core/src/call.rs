use std::sync::Arc;

use serde::Deserialize;

use crate::{json::JsonValue, procedure::ProcedureId};

pub type CallKey = Arc<str>;
pub type CallArgs = Option<JsonValue>;

#[derive(Deserialize, Debug, Clone)]
pub struct IncomingCall {
    pub key: CallKey,
    pub proc: ProcedureId,
    pub args: CallArgs,
}

pub type IncomingCalls = Vec<IncomingCall>;

pub struct CurrentCall {
    pub key: CallKey,
    pub args: CallArgs,
}

impl CurrentCall {
    pub fn new(incoming: IncomingCall) -> Self {
        Self {
            key: incoming.key,
            args: incoming.args,
        }
    }
}
