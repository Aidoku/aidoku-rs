pub type Rid = i32;

use alloc::string::String;

use super::error::{AidokuError, Result, ValueCastError};

const BUFFER_CHUNK_SIZE: usize = 0x80;

#[repr(C)]
#[derive(PartialEq)]
pub enum Kind {
    Null,
    Int,
    Float,
    String,
    Bool,
    Array,
    Object,
    Date,
}

#[link(wasm_import_module = "std")]
extern "C" {
    pub fn copy(rid: Rid) -> Rid;
    pub fn destroy(rid: Rid);

    // fn create_null() -> Rid;
    fn create_array() -> Rid;
    fn create_object() -> Rid;
    fn create_string(buf: *const u8, len: usize) -> Rid;
    fn create_bool(value: bool) -> Rid;
    fn create_float(value: f64) -> Rid;
    fn create_int(value: i64) -> Rid;
    // fn create_date() -> Rid;

    #[link_name = "typeof"]
    fn value_kind(ctx: Rid) -> Kind;
    fn string_len(ctx: Rid) -> usize;
    fn read_string(ctx: Rid, buf: *mut u8, len: usize);
    fn read_int(ctx: Rid) -> i64;
    fn read_float(ctx: Rid) -> f64;
    fn read_bool(ctx: Rid) -> bool;
    // fn read_date(ctx: Rid) -> f64;
    // fn read_date_string(ctx: Rid, format: *mut u8, format_length: usize, locale: *mut u8, locale_length: usize, timezone: *mut u8, timezone_length: usize) -> f64;

    fn object_len(arr: Rid) -> usize;
    fn object_get(arr: Rid, key: *const u8, len: usize) -> Rid;
    fn object_set(arr: Rid, key: *const u8, len: usize, value: Rid);
    fn object_remove(arr: Rid, key: *const u8, len: usize);
    fn object_keys(arr: Rid) -> Rid;
    fn object_values(arr: Rid) -> Rid;

    fn array_len(arr: Rid) -> usize;
    fn array_get(arr: Rid, idx: usize) -> Rid;
    fn array_set(arr: Rid, idx: usize, value: Rid);
    fn array_append(arr: Rid, value: Rid);
    fn array_remove(arr: Rid, idx: usize);
}

pub struct ValueRef(Rid);

pub struct ArrayRef(pub Rid, usize);
pub struct ObjectRef(pub Rid);
pub struct StringRef(pub Rid);

// ==========================
//         Value Ref
// ==========================
impl ValueRef {
    pub fn new(rid: Rid) -> Self {
        ValueRef(rid)
    }

    pub fn kind(&self) -> Kind {
        unsafe { value_kind(self.0) }
    }

    pub fn is_none(&self) -> bool {
        self.kind() == Kind::Null
    }

    pub fn is_some(&self) -> bool {
        !self.is_none()
    }

    pub fn as_string(self) -> Result<StringRef> {
        if self.kind() == Kind::String {
            Ok(StringRef(self.0))
        } else {
            Err(AidokuError::from(ValueCastError::NotString))
        }
    }

    pub fn as_object(self) -> Result<ObjectRef> {
        if self.kind() == Kind::Object {
            Ok(ObjectRef(self.0))
        } else {
            Err(AidokuError::from(ValueCastError::NotObject))
        }
    }

    pub fn as_array(&self) -> Result<ArrayRef> {
        if self.kind() == Kind::Array {
            Ok(ArrayRef(self.0, 0))
        } else {
            Err(AidokuError::from(ValueCastError::NotArray))
        }
    }

    pub fn as_int(&self) -> Result<i64> {
        if self.kind() == Kind::Int {
            let val = unsafe { read_int(self.0) };
            Ok(val)
        } else {
            Err(AidokuError::from(ValueCastError::NotInt))
        }
    }

    pub fn as_float(&self) -> Result<f64> {
        if self.kind() == Kind::Float {
            let val = unsafe { read_float(self.0) };
            Ok(val)
        } else {
            Err(AidokuError::from(ValueCastError::NotFloat))
        }
    }

    pub fn as_bool(&self) -> Result<bool> {
        if self.kind() == Kind::Bool {
            let val = unsafe { read_bool(self.0) };
            Ok(val)
        } else {
            Err(AidokuError::from(ValueCastError::NotBool))
        }
    }
}

impl Clone for ValueRef {
    fn clone(&self) -> Self {
        let rid = unsafe { copy(self.0) };
        Self(rid)
    }
}

impl Drop for ValueRef {
    fn drop(&mut self) {
        unsafe { destroy(self.0) }
    }
}

impl From<i32> for ValueRef {
    fn from(val: i32) -> Self {
        let rid = unsafe { create_int(val as i64) };
        Self(rid)
    }
}

impl From<i64> for ValueRef {
    fn from(val: i64) -> Self {
        let rid = unsafe { create_int(val) };
        Self(rid)
    }
}

impl From<f32> for ValueRef {
    fn from(val: f32) -> Self {
        let rid = unsafe { create_float(val as f64) };
        Self(rid)
    }
}

impl From<f64> for ValueRef {
    fn from(val: f64) -> Self {
        let rid = unsafe { create_float(val) };
        Self(rid)
    }
}

impl From<bool> for ValueRef {
    fn from(val: bool) -> Self {
        let rid = unsafe { create_bool(val) };
        Self(rid)
    }
}

// =========================
//        String Ref
// =========================
impl StringRef {
    pub fn read<'a>(self) -> String {
        let rid = self.0;
        let len = unsafe { string_len(rid) };
        let mut string = String::with_capacity(len);
        let mut offset: usize = 0;
        while offset < len {
            let ending_offset = offset + BUFFER_CHUNK_SIZE;
            let chunk = if ending_offset < len {
                &mut string[offset..ending_offset]
            } else {
                &mut string[offset..]
            };
            unsafe { read_string(self.0, chunk.as_mut_ptr(), chunk.len()) };
            offset = ending_offset;
        }
        string
    }
}

impl<S> From<S> for StringRef
where
    S: AsRef<str>,
{
    fn from(string: S) -> Self {
        let string_slice = string.as_ref();
        let rid = unsafe { create_string(string_slice.as_ptr(), string_slice.len()) };
        Self(rid)
    }
}

impl Clone for StringRef {
    fn clone(&self) -> Self {
        let rid = unsafe { copy(self.0) };
        Self(rid)
    }
}

impl Drop for StringRef {
    fn drop(&mut self) {
        unsafe { destroy(self.0) }
    }
}

// =========================
//        Array Ref
// =========================
impl ArrayRef {
    pub fn new() -> Self {
        let rid = unsafe { create_array() };
        Self(rid, 0)
    }

    pub fn len(&self) -> usize {
        unsafe { array_len(self.0) }
    }

    pub fn get(&self, index: usize) -> ValueRef {
        let rid = unsafe { array_get(self.0, index) };
        ValueRef::new(rid)
    }

    pub fn set(&mut self, index: usize, value: ValueRef) {
        unsafe { array_set(self.0, index, value.0) };
    }

    pub fn insert(&mut self, value: ValueRef) {
        unsafe { array_append(self.0, value.0) };
    }

    pub fn remove(&mut self, index: usize) {
        unsafe { array_remove(self.0, index) };
    }
}

impl Iterator for ArrayRef {
    type Item = ValueRef;

    fn next(&mut self) -> Option<Self::Item> {
        if self.1 >= self.len() {
            return None;
        }
        let value_ref = self.get(self.1);
        self.1 += 1;
        Some(value_ref)
    }
}

impl Clone for ArrayRef {
    fn clone(&self) -> Self {
        let rid = unsafe { copy(self.0) };
        Self(rid, self.1)
    }
}

impl Drop for ArrayRef {
    fn drop(&mut self) {
        unsafe { destroy(self.0) }
    }
}

// =========================
//        Object Ref
// =========================
impl ObjectRef {
    pub fn new() -> Self {
        let rid = unsafe { create_object() };
        Self(rid)
    }

    pub fn len(&self) -> usize {
        unsafe { object_len(self.0) }
    }

    pub fn get(&self, key: &str) -> ValueRef {
        let rid = unsafe { object_get(self.0, key.as_ptr(), key.len()) };
        ValueRef::new(rid)
    }

    pub fn set(&mut self, key: &str, value: ValueRef) {
        unsafe { object_set(self.0, key.as_ptr(), key.len(), value.0) };
    }

    pub fn remove(&mut self, key: &str) {
        unsafe { object_remove(self.0, key.as_ptr(), key.len()) };
    }

    pub fn keys(&mut self) -> ArrayRef {
        let rid = unsafe { object_keys(self.0) };
        ArrayRef(rid, 0)
    }

    pub fn values(&mut self) -> ArrayRef {
        let rid = unsafe { object_values(self.0) };
        ArrayRef(rid, 0)
    }
}

impl Clone for ObjectRef {
    fn clone(&self) -> Self {
        let rid = unsafe { copy(self.0) };
        Self(rid)
    }
}

impl Drop for ObjectRef {
    fn drop(&mut self) {
        unsafe { destroy(self.0) }
    }
}
