use errs::Catch;
use injector::InjectorRef;

use crate::{app::AppRef, call::CurrentCall};

/// A trait that takes a request and tries to convert it into arguments for a
/// procedure.
pub trait FromRequest: Sized {
    fn from_request(app: &AppRef, injector: &InjectorRef, call: &CurrentCall) -> Catch<Self>;
}

macro_rules! factory_tuple ({ $($param:ident)* } => {
    impl<$($param,)*> FromRequest for ($($param,)*)
    where
        $($param: FromRequest,)*
    {
        #[inline]
        fn from_request(
            _app: &AppRef,
            _injector: &InjectorRef,
            _call: &CurrentCall,
        ) -> Catch<Self> {
            Ok(($($param::from_request(_app, _injector, _call)?,)*))
        }
    }
});

factory_tuple! {}
factory_tuple! { A }
factory_tuple! { A B }
factory_tuple! { A B C }
factory_tuple! { A B C D }
factory_tuple! { A B C D E }
factory_tuple! { A B C D E F }
factory_tuple! { A B C D E F G }
factory_tuple! { A B C D E F G H }
