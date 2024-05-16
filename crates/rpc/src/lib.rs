#[cfg(any(feature = "openschema", feature = "core", feature = "full"))]
pub use rpc_macros::Schemable;

#[cfg(any(feature = "core", feature = "full"))]
pub use rpc_core::app::App;

#[cfg(any(feature = "core", feature = "full"))]
pub use rpc_core::call;

#[cfg(any(feature = "core", feature = "full"))]
pub use rpc_core::extractors;

#[cfg(any(feature = "core", feature = "full"))]
pub use rpc_core::json;

#[cfg(any(feature = "core", feature = "full"))]
pub use rpc_core::procedure;

#[cfg(any(feature = "core", feature = "full"))]
pub use rpc_core::responder::*;

#[cfg(any(feature = "core", feature = "full"))]
pub use rpc_core::router;

#[cfg(feature = "full")]
pub mod server {
    pub use rpc_server::*;
}

#[cfg(any(feature = "openschema", feature = "core", feature = "full"))]
pub mod open_schema {
    pub use rpc_openschema::*;
}
