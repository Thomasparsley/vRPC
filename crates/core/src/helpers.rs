use std::any::type_name;

#[inline]
pub fn function_name<T: 'static>() -> &'static str {
    type_name::<T>().trim().split("::").last().unwrap()
}
