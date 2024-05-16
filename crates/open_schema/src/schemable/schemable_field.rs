use std::{
    collections::{BTreeSet, HashSet},
    sync::Arc,
};

use crate::schema::{FieldFormat, FieldType, SchemaField, SchemaFieldRel, TypeMapRef};

pub trait SchemableField {
    fn get_rel_type() -> SchemaFieldRel;
    fn explore_type(_field: &mut SchemaField, _type_map: TypeMapRef);
}

macro_rules! impl_schemable_field {
    ($($t:ty),*) => {
        $(
            impl SchemableField for $t {
                fn get_rel_type() -> SchemaFieldRel  {
                    SchemaFieldRel::Native {
                        ty: <$t as FieldType>::field_type(),
                        format: <$t as FieldFormat>::field_format(),
                    }
                }

                fn explore_type(field: &mut SchemaField, _type_map: TypeMapRef) {
                    field.rel = Some(<$t as SchemableField>::get_rel_type());
                }
            }
        )*
    };
}

impl_schemable_field!(
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
    bool,
    String,
    &str,
    Arc<str>,
    std::time::Duration,
    time::Date,
    time::OffsetDateTime,
    serde_json::Value,
    errs::Error
);

impl<T: SchemableField> SchemableField for Option<T> {
    fn get_rel_type() -> SchemaFieldRel {
        SchemaFieldRel::Nullable {
            value: Box::new(T::get_rel_type()),
        }
    }

    fn explore_type(field: &mut SchemaField, type_map: TypeMapRef) {
        T::explore_type(field, type_map);
        field.rel = Some(Option::<T>::get_rel_type());
    }
}

impl<T: SchemableField> SchemableField for Vec<T> {
    fn get_rel_type() -> SchemaFieldRel {
        SchemaFieldRel::Array {
            value: Box::new(T::get_rel_type()),
        }
    }

    fn explore_type(field: &mut SchemaField, type_map: TypeMapRef) {
        T::explore_type(field, type_map);

        field.rel = Some(Vec::<T>::get_rel_type());
    }
}

impl<T: SchemableField> SchemableField for [T] {
    fn get_rel_type() -> SchemaFieldRel {
        SchemaFieldRel::Array {
            value: Box::new(T::get_rel_type()),
        }
    }

    fn explore_type(field: &mut SchemaField, type_map: TypeMapRef) {
        T::explore_type(field, type_map);
        field.rel = Some(Vec::<T>::get_rel_type());
    }
}

impl<T: SchemableField> SchemableField for HashSet<T> {
    fn get_rel_type() -> SchemaFieldRel {
        SchemaFieldRel::Array {
            value: Box::new(T::get_rel_type()),
        }
    }

    fn explore_type(field: &mut SchemaField, type_map: TypeMapRef) {
        T::explore_type(field, type_map);
        field.rel = Some(HashSet::<T>::get_rel_type());
    }
}

impl<T: SchemableField> SchemableField for BTreeSet<T> {
    fn get_rel_type() -> SchemaFieldRel {
        SchemaFieldRel::Array {
            value: Box::new(T::get_rel_type()),
        }
    }

    fn explore_type(field: &mut SchemaField, type_map: TypeMapRef) {
        T::explore_type(field, type_map);
        field.rel = Some(HashSet::<T>::get_rel_type());
    }
}

impl<Ok: SchemableField, Err> SchemableField for Result<Ok, Err> {
    fn get_rel_type() -> SchemaFieldRel {
        Ok::get_rel_type()
    }

    fn explore_type(field: &mut SchemaField, type_map: TypeMapRef) {
        Ok::explore_type(field, type_map);
    }
}

impl<K, V> SchemableField for std::collections::HashMap<K, V>
where
    K: SchemableField,
    V: SchemableField,
{
    fn get_rel_type() -> SchemaFieldRel {
        SchemaFieldRel::Map {
            key: Box::new(K::get_rel_type()),
            value: Box::new(V::get_rel_type()),
        }
    }

    fn explore_type(field: &mut SchemaField, type_map: TypeMapRef) {
        K::explore_type(field, type_map.clone());
        V::explore_type(field, type_map);

        field.rel = Some(std::collections::HashMap::<K, V>::get_rel_type());
    }
}

impl<K, V> SchemableField for std::collections::BTreeMap<K, V>
where
    K: SchemableField,
    V: SchemableField,
{
    fn get_rel_type() -> SchemaFieldRel {
        SchemaFieldRel::Map {
            key: Box::new(K::get_rel_type()),
            value: Box::new(V::get_rel_type()),
        }
    }

    fn explore_type(field: &mut SchemaField, type_map: TypeMapRef) {
        K::explore_type(field, type_map.clone());
        V::explore_type(field, type_map);

        field.rel = Some(std::collections::HashMap::<K, V>::get_rel_type());
    }
}
