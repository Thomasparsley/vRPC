use serde::{Deserialize, Serialize};

use errs::Catch;
use injector::InjectorRef;

use rpc_openschema::applike::AppInfoLike;

use crate::{app::AppRef, call::CurrentCall, from_request::FromRequest};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppInfo {
    pub name: String,
    pub version: String,
    pub description: String,
}

impl AppInfo {
    pub fn new(name: &str, version: &str, description: &str) -> Self {
        Self {
            name: name.to_string(),
            version: version.to_string(),
            description: description.to_string(),
        }
    }
}

impl AppInfoLike for AppInfo {
    #[inline]
    fn name(&self) -> String {
        self.name.clone()
    }

    #[inline]
    fn version(&self) -> String {
        self.version.clone()
    }

    #[inline]
    fn description(&self) -> String {
        self.description.clone()
    }
}

impl FromRequest for AppInfo {
    #[inline]
    fn from_request(app: &AppRef, _injector: &InjectorRef, _call: &CurrentCall) -> Catch<Self> {
        Ok(app.info.clone())
    }
}
