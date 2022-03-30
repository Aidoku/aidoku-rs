use super::error::{AidokuError, Result};
use core::iter::Iterator;
use std::marker::PhantomData;

pub type Rid = i32;
pub type ValueRid = i32;

#[link(wasm_import_module = "iterator")]
extern "C" {
    #[link_name = "iterator_read"]
    fn __wasm_iterator_read(iter: Rid) -> bool;

    #[link_name = "iterator_value"]
    fn __wasm_iterator_value(iter: Rid) -> ValueRid;
}

pub trait FromIteratorValue: Sized {
    fn from_rid(rid: ValueRid) -> Result<Self>;
}

pub struct AidokuIterator<T>(Rid, PhantomData<T>)
where
    T: FromIteratorValue;

impl<T: FromIteratorValue> AidokuIterator<T> {
    fn from_rid(rid: Rid) -> Self {
        AidokuIterator::<T>(rid, PhantomData)
    }
}

impl<T: FromIteratorValue> Iterator for AidokuIterator<T> {
    type Item = Result<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if unsafe { __wasm_iterator_read(self.0) } {
            None
        } else {
            let value_id = unsafe { __wasm_iterator_value(self.0) };
            Some(T::from_rid(value_id))
        }
    }
}
