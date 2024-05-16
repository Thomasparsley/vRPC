use serde::de::DeserializeOwned;

use crate::{schema::TypeMapRef, SchemaProcedure, SchemableField};

pub trait SchemableParams {
    fn apply_schema(proc: &mut SchemaProcedure, type_map: TypeMapRef);
}

macro_rules! factory_params_tuple ({ $($param:ident)* } => {
    impl<$($param,)*> SchemableParams for ($($param,)*)
    where
        $($param: SchemableParams,)*
    {
        #[inline]
        #[allow(non_snake_case)]
        fn apply_schema(_proc: &mut SchemaProcedure, _type_map: TypeMapRef) {
            $($param::apply_schema(_proc, _type_map.clone());)*
        }
    }
});

factory_params_tuple! {}
factory_params_tuple! { A }
factory_params_tuple! { A B }
factory_params_tuple! { A B C }
factory_params_tuple! { A B C D }
factory_params_tuple! { A B C D E }
factory_params_tuple! { A B C D E F }
factory_params_tuple! { A B C D E F G }

impl<T> SchemableParams for Option<T>
where
    T: SchemableParams + SchemableField + DeserializeOwned,
{
    #[inline]
    fn apply_schema(proc: &mut SchemaProcedure, type_map: TypeMapRef) {
        T::apply_schema(proc, type_map);

        if proc.params.is_some() {
            proc.params = Some(Option::<T>::get_rel_type())
        }
    }
}

macro_rules! factory_param {
    ($($t:ty),*) => {
        $(
            impl SchemableParams for $t {
                #[inline]
                fn apply_schema(proc: &mut SchemaProcedure, _: TypeMapRef) {
                    proc.params = Some(<$t as SchemableField>::get_rel_type());
                }
            }
        )*
    }
}

factory_param!(
    bool,
    String,
    &str,
    i8,
    i16,
    i32,
    i64,
    u8,
    u16,
    u32,
    u64,
    f32,
    f64,
    serde_json::Value
);
