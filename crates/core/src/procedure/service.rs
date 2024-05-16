use callbacks::Callback;
use injector::InjectorRef;

use crate::{
    app::AppRef, call::CurrentCall, from_request::FromRequest, json::JsonValue,
    responder::Responder,
};

use super::{response::ProcedureResponse, Procedureable};

pub type ProcedureService = Box<Callback<(AppRef, InjectorRef, CurrentCall), JsonValue>>;

pub fn new_procedure_service<F, Args>(procedure: F) -> ProcedureService
where
    F: Procedureable<Args>,
    Args: FromRequest + Send,
    F::Output: Responder,
{
    Box::new(move |(app, injector, call)| {
        let procedure = procedure.clone();

        Box::pin(async move {
            let args = Args::from_request(&app, &injector, &call);

            let response = match args {
                Ok(args) => {
                    let response = procedure.call(args).await;
                    response.into_json(call.key)
                }
                Err(err) => {
                    let response = ProcedureResponse::error(call.key, err);
                    response.into()
                }
            };

            response.into()
        })
    })
}
