use std::future::Future;

use crate::{from_request::FromRequest, responder::Responder};

pub trait Procedureable<Args>: Clone + Send + Sync + 'static
where
    Args: FromRequest,
{
    type Output: Responder + Send;
    type Future: Future<Output = Self::Output> + Send;

    fn call(&self, args: Args) -> Self::Future;
}

macro_rules! factory_tuple ({ $($param:ident)* } => {
    impl<Func, Fut, $($param,)*> Procedureable<($($param,)*)> for Func
    where
        Func: Fn($($param),*) -> Fut + Clone + Send + Sync + 'static,
        Fut: Future + Send,
        Fut::Output: Responder + Send,
        $($param: FromRequest,)*
    {
        type Output = Fut::Output;
        type Future = Fut;

        #[inline]
        #[allow(non_snake_case)]
        fn call(&self, ($($param,)*): ($($param,)*)) -> Self::Future {
            (self)($($param,)*)
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
