use serde::de::DeserializeOwned;

use errs::Catch;
use injector::InjectorRef;

use rpc_openschema::{schema::TypeMapRef, SchemaProcedure, SchemableField, SchemableParams};

use crate::{app::AppRef, call::CurrentCall, errors, from_request::FromRequest};

pub struct Args<T: DeserializeOwned>(pub T);

impl<T: DeserializeOwned> Args<T> {
    #[inline]
    pub fn inner(self) -> T {
        self.0
    }
}

impl<T> FromRequest for Args<T>
where
    T: DeserializeOwned,
{
    fn from_request(_app: &AppRef, _injector: &InjectorRef, call: &CurrentCall) -> Catch<Self> {
        let args = match call.args.clone() {
            Some(args) => args,
            _ => return Err(errors::empty_call_args()),
        };

        let value = serde_json::from_value(args);
        let value = match value {
            Ok(value) => Self(value),
            Err(e) => return Err(errors::unparsable_call_args(e.to_string())),
        };

        Ok(value)
    }
}

impl<T> SchemableParams for Args<T>
where
    T: DeserializeOwned + SchemableParams,
{
    #[inline]
    fn apply_schema(proc: &mut SchemaProcedure, type_map: TypeMapRef) {
        T::apply_schema(proc, type_map)
    }
}

impl<T> From<T> for Args<T>
where
    T: DeserializeOwned + SchemableParams,
{
    fn from(value: T) -> Self {
        Self(value)
    }
}

pub struct OptionalArgs<T: DeserializeOwned>(pub Option<T>);

impl<T: DeserializeOwned> OptionalArgs<T> {
    #[inline]
    pub fn inner(self) -> Option<T> {
        self.0
    }
}

impl<T> FromRequest for OptionalArgs<T>
where
    T: DeserializeOwned,
{
    fn from_request(_app: &AppRef, _injector: &InjectorRef, call: &CurrentCall) -> Catch<Self> {
        let args = match call.args.clone() {
            Some(args) => args,
            _ => return Ok(Self(None)),
        };

        let value = serde_json::from_value(args);
        let value = match value {
            Ok(value) => Self(value),
            Err(e) => return Err(errors::unparsable_call_args(e.to_string())),
        };

        Ok(value)
    }
}

impl<T> SchemableParams for OptionalArgs<T>
where
    T: DeserializeOwned + SchemableParams + SchemableField,
{
    #[inline]
    fn apply_schema(proc: &mut SchemaProcedure, type_map: TypeMapRef) {
        Option::<T>::apply_schema(proc, type_map)
    }
}
