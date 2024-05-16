use std::{rc::Rc, sync::Arc};

use rpc_openschema::{
    schema::{new_type_map_ref, SchemaRoot, TypeMapRef},
    SchemaProcedure, SCHEMA_PATH_SEPARATOR, SCHEMA_VERSION,
};

use crate::{app::App, extractors::AppInfo, procedure::Procedure, router::Router};

pub(crate) fn build_schema(app: &App) -> SchemaRoot<AppInfo> {
    let procedures_len = 0; //app.procedures.len();
    let mut procedures = Vec::with_capacity(procedures_len);
    let structs = new_type_map_ref();

    for router in &app.routers {
        let path = vec![router.name.clone()];
        visit_router(&mut procedures, structs.clone(), router, path);
    }

    let structs = Rc::try_unwrap(structs).unwrap().into_inner().unwrap();

    let root = SchemaRoot {
        rpcapi: SCHEMA_VERSION.to_string(),
        info: app.info.clone(),
        procedures,
        types: structs,
    };

    root
}

fn visit_router(
    procs: &mut Vec<SchemaProcedure>,
    structs: TypeMapRef,
    app_router: &Router,
    path: Vec<Arc<str>>,
) {
    for router in &app_router.routers {
        let mut path = path.clone();
        path.push(router.name.clone());

        visit_router(procs, structs.clone(), router, path);
    }

    let path = path.join(SCHEMA_PATH_SEPARATOR);
    let path: Arc<str> = Arc::from(path);

    for procedure in &app_router.procedures.items {
        visit_procedure(procs, structs.clone(), procedure.clone(), path.clone());
    }
}

fn visit_procedure(
    procs: &mut Vec<SchemaProcedure>,
    structs: TypeMapRef,
    app_proc: Procedure,
    path: Arc<str>,
) {
    let path = path.clone();
    let procedure_schema = SchemaProcedure::from_app(path.to_string(), app_proc, structs);

    procs.push(procedure_schema);
}
