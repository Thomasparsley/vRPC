use std::{collections::BTreeMap, rc::Rc, sync::Mutex};

use serde::{Deserialize, Serialize};

use crate::{applike::AppInfoLike, SchemableType};

use super::{schema_type::SchemaTypes, SchemaProcedure, SchemaType};

pub type TypeMap = BTreeMap<String, SchemaType>;
pub type TypeMapRef = Rc<Mutex<TypeMap>>;

pub fn insert_into_type_map_ref<T: SchemableType>(ty: SchemaTypes, map: TypeMapRef) {
    let name = T::type_name();

    let contains = map.lock().unwrap().contains_key(&name);
    if contains {
        return;
    }

    let fields = T::type_fields(map.clone());

    map.lock()
        .unwrap()
        .insert(name.clone(), SchemaType::new(name, ty, fields));
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SchemaRoot<I>
where
    I: AppInfoLike + Serialize,
{
    pub rpcapi: String,
    pub info: I,
    pub procedures: Vec<SchemaProcedure>,
    pub types: TypeMap,
}

impl<I> From<SchemaRoot<I>> for serde_json::Value
where
    I: AppInfoLike + Serialize,
{
    #[inline]
    fn from(root: SchemaRoot<I>) -> Self {
        serde_json::to_value(root).unwrap()
    }
}

pub fn new_type_map() -> TypeMap {
    BTreeMap::new()
}

pub fn new_type_map_ref() -> TypeMapRef {
    Rc::new(Mutex::new(new_type_map()))
}
