use std::sync::Arc;

use crate::{
    schema::schema_root::{insert_into_type_map_ref, TypeMapRef},
    SchemaProcedure, SchemableField, SchemableType,
};

pub trait SchemableResult {
    fn apply_schema(proc: &mut SchemaProcedure, type_map: TypeMapRef);
}

impl SchemableResult for () {
    #[inline]
    fn apply_schema(_: &mut SchemaProcedure, _: TypeMapRef) {}
}

macro_rules! factory_result {
    ($($t:ty),*) => {
        $(
            impl SchemableResult for $t {
                #[inline]
                fn apply_schema(proc: &mut SchemaProcedure, _: TypeMapRef) {
                    proc.result = Some(<$t as SchemableField>::get_rel_type());
                }
            }

            impl SchemableResult for Vec<$t> {
                fn apply_schema(proc: &mut SchemaProcedure, _: TypeMapRef) {
                    proc.result = Some(<Vec<$t> as SchemableField>::get_rel_type());
                }
            }
        )*
    };
}

factory_result!(
    bool,
    String,
    &str,
    Arc<str>,
    i8,
    i16,
    i32,
    i64,
    i128,
    isize,
    u8,
    u16,
    u32,
    u64,
    u128,
    usize,
    f32,
    f64,
    serde_json::Value,
    Option<bool>,
    Option<String>,
    Option<&str>,
    Option<i8>,
    Option<i16>,
    Option<i32>,
    Option<i64>,
    Option<i128>,
    Option<isize>,
    Option<u8>,
    Option<u16>,
    Option<u32>,
    Option<u64>,
    Option<u128>,
    Option<usize>,
    Option<f32>,
    Option<f64>,
    Option<serde_json::Value>
);

impl<T> SchemableResult for Vec<T>
where
    T: SchemableField + SchemableType,
{
    fn apply_schema(proc: &mut SchemaProcedure, type_map: TypeMapRef) {
        proc.result = Some(<Vec<T> as SchemableField>::get_rel_type());

        insert_into_type_map_ref::<T>(T::schema_type(), type_map);
    }
}

impl<T> SchemableResult for Option<T>
where
    T: SchemableField + SchemableType,
{
    fn apply_schema(proc: &mut SchemaProcedure, type_map: TypeMapRef) {
        proc.result = Some(<Option<T> as SchemableField>::get_rel_type());

        insert_into_type_map_ref::<T>(T::schema_type(), type_map);
    }
}

impl<Ok, Err> SchemableResult for Result<Ok, Err>
where
    Ok: SchemableResult,
{
    fn apply_schema(proc: &mut SchemaProcedure, type_map: TypeMapRef) {
        Ok::apply_schema(proc, type_map);
    }
}
