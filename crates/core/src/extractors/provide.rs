use std::any::Any;

use errs::Catch;
use injector::InjectorRef;

use rpc_openschema::{schema::TypeMapRef, SchemaProcedure, SchemableParams};

use crate::{app::AppRef, call::CurrentCall, errors, from_request::FromRequest};

pub struct Provide<T: Any + Send + Sync + Clone>(pub T);

impl<T: Any + Send + Sync + Clone> Provide<T> {
    #[inline]
    pub fn inner(self) -> T {
        self.0
    }
}

impl<T: Any + Send + Sync + Clone> FromRequest for Provide<T> {
    fn from_request(_app: &AppRef, injector: &InjectorRef, _call: &CurrentCall) -> Catch<Self> {
        let value = injector.obtain::<T>();

        match value {
            None => Err(errors::injector_not_found()),
            Some(value) => Ok(Self(value)),
        }
    }
}

impl<T: Any + Send + Sync + Clone> SchemableParams for Provide<T> {
    #[inline]
    fn apply_schema(_proc: &mut SchemaProcedure, _: TypeMapRef) {}
}
