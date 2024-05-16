use std::{fmt::Debug, sync::Arc};

use injector::InjectorRef;

use rpc_openschema::{
    procedurelike::ProcedureLike,
    schema::{ProcedureType, TypeMapRef},
    SchemaProcedure, SchemableParams, SchemableResult,
};
use serde::Serialize;

use crate::{app::AppRef, call::CurrentCall, from_request::FromRequest, json::JsonValue};

use self::{
    procedureable::Procedureable,
    schema::{new_procedure_schema_service, ProcedureSchemaService},
    service::{new_procedure_service, ProcedureService},
};

pub mod procedureable;
pub mod response;
pub mod schema;
pub mod service;

pub type ProcedureId = usize;
pub type ProcedureName = Arc<str>;

pub(crate) type ProcedureServiceRef = Arc<ProcedureService>;
pub(crate) type ProcedureSchemaServiceRef = Arc<ProcedureSchemaService>;

#[derive(Clone)]
pub struct Procedure {
    pub(crate) id: Option<ProcedureId>,
    pub(crate) name: ProcedureName,
    pub(crate) ty: ProcedureType,
    pub(crate) service: ProcedureServiceRef,
    pub(crate) schema: ProcedureSchemaServiceRef,
}

impl Procedure {
    fn init(
        name: &str,
        ty: ProcedureType,
        service: ProcedureServiceRef,
        schema: ProcedureSchemaServiceRef,
    ) -> Self {
        Self {
            id: None,
            name: Arc::from(name),
            ty,
            service,
            schema,
        }
    }

    pub fn new_query(
        name: &str,
        service: ProcedureServiceRef,
        schema: ProcedureSchemaServiceRef,
    ) -> Self {
        Self::init(name, ProcedureType::Query, service, schema)
    }

    pub fn new_mutation(
        name: &str,
        service: ProcedureServiceRef,
        schema: ProcedureSchemaServiceRef,
    ) -> Self {
        Self::init(name, ProcedureType::Mutation, service, schema)
    }

    pub fn id(&self) -> ProcedureId {
        self.id.unwrap_or_default()
    }

    pub fn name(&self) -> ProcedureName {
        self.name.clone()
    }

    pub fn procedure_type(&self) -> ProcedureType {
        self.ty.clone()
    }

    pub(crate) fn set_id(&mut self, id: ProcedureId) {
        self.id = Some(id);
    }

    pub async fn execute(
        &self,
        app: AppRef,
        injector: InjectorRef,
        call: CurrentCall,
    ) -> JsonValue {
        let future = (self.service)((app, injector, call));
        future.await
    }
}

impl ProcedureLike for Procedure {
    fn id(&self) -> usize {
        self.id()
    }

    fn procedure_type(&self) -> ProcedureType {
        self.procedure_type()
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn call_schema(
        &self,
        procedure_schema: SchemaProcedure,
        type_map: TypeMapRef,
    ) -> SchemaProcedure {
        (self.schema)((procedure_schema, type_map))
    }
}

impl Debug for Procedure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Procedure")
            .field("id", &self.id)
            .field("name", &self.name)
            .field("ty", &self.ty)
            .finish()
    }
}

#[derive(Debug)]
pub struct Procedures {
    pub(crate) items: Vec<Procedure>,
}

impl Procedures {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    /// Get procedure by id
    pub fn get(&self, id: ProcedureId) -> Option<Procedure> {
        if id > self.items.len() {
            return None;
        }

        let procedure = self.items.get(id).unwrap();
        let procedure = procedure.clone();

        return Some(procedure);
    }

    /// Add procedure to router
    fn add(&mut self, procedure: Procedure) {
        self.items.push(procedure);
    }

    /// Insert clone of procedure to procedures
    pub(crate) fn insert(&mut self, procedure: &mut Procedure, id: ProcedureId) {
        procedure.set_id(id);

        let cloned_procedure = procedure.clone();
        self.add(cloned_procedure);
    }

    /// Add query procedure to router
    pub fn add_query<F, Args>(&mut self, name: &str, procedure: F)
    where
        F: Procedureable<Args>,
        Args: FromRequest + SchemableParams + Send,
        F::Output: SchemableResult + Serialize,
    {
        let service = new_procedure_service(procedure.clone());
        let schema = new_procedure_schema_service(procedure);

        let procedure = Procedure::new_query(name, Arc::new(service), Arc::new(schema));
        self.add(procedure);
    }

    /// Add mutation procedure to router
    pub fn add_mutation<F, Args>(&mut self, name: &str, procedure: F)
    where
        F: Procedureable<Args>,
        Args: FromRequest + SchemableParams + Send,
        F::Output: SchemableResult + Serialize,
    {
        let service = new_procedure_service(procedure.clone());
        let schema = new_procedure_schema_service(procedure);

        let procedure = Procedure::new_mutation(name, Arc::new(service), Arc::new(schema));
        self.add(procedure);
    }
}
