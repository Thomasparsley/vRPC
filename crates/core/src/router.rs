use std::sync::Arc;

use rpc_openschema::{SchemableParams, SchemableResult};
use serde::Serialize;

use crate::{
    from_request::FromRequest,
    helpers::function_name,
    procedure::{procedureable::Procedureable, Procedure, ProcedureId, Procedures},
};

#[derive(Debug)]
pub struct Router {
    pub(crate) name: Arc<str>,
    pub(crate) routers: Routers,
    pub(crate) procedures: Procedures,
}

pub type Routers = Vec<Router>;
pub type RouterCreator = fn(&mut Router);

impl Router {
    pub fn new(name: &str) -> Self {
        Self {
            name: Arc::from(name),
            routers: Routers::new(),
            procedures: Procedures::new(),
        }
    }

    pub fn add_router(&mut self, name: &str, modify: fn(&mut Router)) -> &mut Self {
        // Make new router
        let mut router = Router::new(name);

        // Call modify function
        modify(&mut router);

        // Push new router to routers
        self.routers.push(router);

        self
    }

    /// Add query procedure to router
    pub fn add_query<F, Args>(&mut self, procedure: F)
    where
        F: Procedureable<Args>,
        Args: FromRequest + SchemableParams + Send,
        F::Output: SchemableResult + Serialize,
    {
        let name = function_name::<F>();

        self.procedures.add_query(name, procedure);
    }

    /// Add mutation procedure to router
    pub fn add_mutation<F, Args>(&mut self, procedure: F)
    where
        F: Procedureable<Args>,
        Args: FromRequest + SchemableParams + Send,
        F::Output: SchemableResult + Serialize,
    {
        let name = function_name::<F>();

        self.procedures.add_mutation(name, procedure);
    }
}

#[derive(Debug)]
pub struct BuildedRouter {
    pub flatten_router: Procedures,
}

impl BuildedRouter {
    fn empty() -> Self {
        Self {
            flatten_router: Procedures::new(),
        }
    }

    pub(crate) fn new(routers: &mut Routers) -> Self {
        let mut id = 0;
        let mut builded_router = Self::empty();

        for router in routers {
            builded_router.add_router(router, &mut id);
        }

        builded_router
    }

    fn add_router(&mut self, router: &mut Router, id: &mut ProcedureId) {
        // Add procedures
        self.add_procedures(&mut router.procedures, id);

        // Add inner routers
        for inner_router in &mut router.routers {
            self.add_router(inner_router, id);
        }
    }

    fn add_procedures(&mut self, procedures: &mut Procedures, id: &mut ProcedureId) {
        /* procedures */

        for mut procedure in &mut procedures.items {
            // Add procedure to flatten router
            self.flatten_router.insert(&mut procedure, id.clone());

            // Increment id
            *id += 1;
        }
    }

    pub fn find_procedure(&self, id: ProcedureId) -> Option<Procedure> {
        self.flatten_router.get(id)
    }
}
