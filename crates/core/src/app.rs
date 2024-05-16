use std::sync::Arc;

use dbg::only_dbg;
use errs::Catch;
use injector::InjectorRef;

use crate::{
    call::{CurrentCall, IncomingCalls},
    errors,
    extractors::AppInfo,
    json::JsonValue,
    procedure::response::ProcedureResponse,
    router::{BuildedRouter, Routers},
    schema::build_schema,
};

#[derive(Debug)]
pub struct App {
    pub info: AppInfo,
    pub routers: Routers,
    pub builded_router: BuildedRouter,
}

pub type AppRef = Arc<App>;

impl App {
    pub fn new(info: AppInfo, mut routers: Routers) -> Self {
        let builded_router = BuildedRouter::new(&mut routers);

        Self {
            info,
            routers,
            builded_router,
        }
    }

    /// Process request
    pub async fn process_request(
        &self,
        app_ref: AppRef,
        injector_ref: InjectorRef,
        calls: IncomingCalls,
    ) -> Catch<Vec<JsonValue>> {
        let calls_len = calls.len();
        let mut results = Vec::with_capacity(calls_len);

        // If there is no calls, return empty response
        if calls_len == 0 {
            return Ok(results);
        }

        let mut futures = Vec::with_capacity(calls_len);
        for call in calls {
            let proc_id = call.proc;
            let procedure = self.builded_router.find_procedure(proc_id);

            let procedure = match procedure {
                Some(p) => p,
                None => {
                    results.push(ProcedureResponse::error(call.key, errors::procedure_not_found()).into());
                    continue;
                }
            };

            let current_call = CurrentCall::new(call);

            let app = app_ref.clone();
            let injector = injector_ref.clone();

            let future = tokio::spawn(async move {
                only_dbg! {
                    let key = current_call.key.clone();
                    let procedure_name = procedure.name();

                    tracing::info!(
                        "Start processing call({}): {}",
                        key.clone(),
                        procedure_name.clone()
                    );
                }

                let response = procedure.execute(app, injector, current_call).await;

                only_dbg! {
                    tracing::info!("End processing call({}): {}", key, procedure_name);
                }

                response
            });

            futures.push(future);
        }

        for future in futures {
            let result = future.await;
            let result = match result {
                Ok(r) => r,
                Err(_) => return Err(errors::one_of_calls_failed()),
            };

            results.push(result);
        }

        Ok(results)
    }

    /// Generate schema
    ///
    /// This method is used to generate schema for client.
    /// This schema is used to generate client code.
    pub fn schema(&self) -> JsonValue {
        let schema = build_schema(self);

        schema.into()
    }
}
