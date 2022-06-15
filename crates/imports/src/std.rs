pub type Rid = i32;

use alloc::string::String;
use alloc::vec::Vec;

use super::html::Node;

use super::error::{AidokuError, Result, ValueCastError};

#[repr(C)]
#[derive(PartialEq, Eq, Debug)]
pub enum Kind {
    Null,
    Int,
    Float,
    String,
    Bool,
    Array,
    Object,
    Date,
    Node,
    Unknown,
}

#[link(wasm_import_module = "std")]
extern "C" {
    pub fn copy(rid: Rid) -> Rid;
    pub fn destroy(rid: Rid);

    fn create_null() -> Rid;
    fn create_array() -> Rid;
    fn create_object() -> Rid;
    fn create_string(buf: *const u8, len: usize) -> Rid;
    fn create_bool(value: bool) -> Rid;
    fn create_float(value: f64) -> Rid;
    fn create_int(value: i64) -> Rid;
    fn create_date(value: f64) -> Rid;

    #[link_name = "typeof"]
    pub fn value_kind(ctx: Rid) -> Kind;
    fn string_len(ctx: Rid) -> usize;
    fn read_string(ctx: Rid, buf: *mut u8, len: usize);
    fn read_int(ctx: Rid) -> i64;
    fn read_float(ctx: Rid) -> f64;
    fn read_bool(ctx: Rid) -> bool;
    fn read_date(ctx: Rid) -> f64;
    fn read_date_string(
        ctx: Rid,
        format: *const u8,
        format_length: usize,
        locale: *const u8,
        locale_length: usize,
        timezone: *const u8,
        timezone_length: usize,
    ) -> f64;

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

pub fn print<T: AsRef<str>>(string: T) {
    let string = string.as_ref();
    extern "C" {
        fn print(string: *const u8, size: usize);
    }
    unsafe {
        print(string.as_ptr(), string.len());
    }
}

/// Gets the current time as a Unix timestamp.
pub fn current_date() -> f64 {
    unsafe { read_date(create_date(-1.0)) }
}

pub struct ValueRef(pub Rid, pub bool);

pub struct ArrayRef(pub ValueRef, pub usize);
pub struct ObjectRef(pub ValueRef);
pub struct StringRef(pub ValueRef);

// ==========================
//         Value Ref
// ==========================
impl ValueRef {
    pub fn new(rid: Rid) -> Self {
        ValueRef(rid, true)
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
            Ok(StringRef(self))
        } else {
            Err(AidokuError::from(ValueCastError::NotString))
        }
    }

    pub fn as_object(self) -> Result<ObjectRef> {
        if self.kind() == Kind::Object {
            Ok(ObjectRef(self))
        } else {
            Err(AidokuError::from(ValueCastError::NotObject))
        }
    }

    pub fn as_array(self) -> Result<ArrayRef> {
        if self.kind() == Kind::Array {
            Ok(ArrayRef(self, 0))
        } else {
            Err(AidokuError::from(ValueCastError::NotArray))
        }
    }

    pub fn as_int(&self) -> Result<i64> {
        let kind = self.kind();
        if kind == Kind::Int || kind == Kind::Float || kind == Kind::Bool || kind == Kind::String {
            let val = unsafe { read_int(self.0) };
            Ok(val)
        } else {
            Err(AidokuError::from(ValueCastError::NotInt))
        }
    }

    pub fn as_float(&self) -> Result<f64> {
        let kind = self.kind();
        if kind == Kind::Float || kind == Kind::Int || kind == Kind::String {
            let val = unsafe { read_float(self.0) };
            Ok(val)
        } else {
            Err(AidokuError::from(ValueCastError::NotFloat))
        }
    }

    pub fn as_bool(&self) -> Result<bool> {
        let kind = self.kind();
        if kind == Kind::Bool || kind == Kind::Int {
            let val = unsafe { read_bool(self.0) };
            Ok(val)
        } else {
            Err(AidokuError::from(ValueCastError::NotBool))
        }
    }

    /// Converts a textual representaion of a date to a Unix timestamp.
    ///
    /// # Arguments
    /// * `format`: The date format, as compatible with
    /// [NSDateFormatter](https://nsdateformatter.com/).
    /// * `locale`: The locale identifier for this date string.
    /// Also available on [NSDateFormatter](https://nsdateformatter.com/).
    /// * `timezone`: The time zone for this date, as compatible with
    /// [TimeZone](https://developer.apple.com/documentation/foundation/timezone).
    /// They can be a [zoneinfo timezone](https://en.wikipedia.org/wiki/List_of_tz_database_time_zones),
    /// or an [abbreviation](https://gist.github.com/mteece/80fff3329074cf90d7991e55f4fc8de4).
    pub fn as_date<T: Default + AsRef<str>>(
        &self,
        format: T,
        locale: Option<T>,
        timezone: Option<T>,
    ) -> Result<f64> {
        if self.kind() == Kind::String {
            let locale = locale.unwrap_or_default();
            let timezone = timezone.unwrap_or_default();

            let format = format.as_ref();
            let locale_val = locale.as_ref();
            let timezone_val = timezone.as_ref();
            let val = unsafe {
                read_date_string(
                    self.0,
                    format.as_ptr(),
                    format.len(),
                    locale_val.as_ptr(),
                    locale_val.len(),
                    timezone_val.as_ptr(),
                    timezone_val.len(),
                )
            };
            Ok(val)
        } else {
            Err(AidokuError::from(ValueCastError::NotBool))
        }
    }

    pub fn as_node(&self) -> Node {
        Node::from(self.0)
    }
}

impl Clone for ValueRef {
    fn clone(&self) -> Self {
        let rid = unsafe { copy(self.0) };
        Self(rid, true)
    }
}

impl Drop for ValueRef {
    fn drop(&mut self) {
        if self.1 {
            unsafe { destroy(self.0) };
        }
    }
}

impl From<i32> for ValueRef {
    fn from(val: i32) -> Self {
        let rid = unsafe { create_int(val as i64) };
        Self(rid, true)
    }
}

impl From<i64> for ValueRef {
    fn from(val: i64) -> Self {
        let rid = unsafe { create_int(val) };
        Self(rid, true)
    }
}

impl From<f32> for ValueRef {
    fn from(val: f32) -> Self {
        let rid = unsafe { create_float(val as f64) };
        Self(rid, true)
    }
}

impl From<f64> for ValueRef {
    fn from(val: f64) -> Self {
        let rid = unsafe { create_float(val) };
        Self(rid, true)
    }
}

impl From<bool> for ValueRef {
    fn from(val: bool) -> Self {
        let rid = unsafe { create_bool(val) };
        Self(rid, true)
    }
}

impl Default for ValueRef {
    /// Creates a null ValueRef.
    fn default() -> Self {
        ValueRef::new(unsafe { create_null() })
    }
}

// =========================
//        String Ref
// =========================
impl StringRef {
    pub fn read(self) -> String {
        let len = unsafe { string_len(self.0 .0) };
        let mut buf = Vec::with_capacity(len);
        unsafe {
            read_string(self.0 .0, buf.as_mut_ptr(), len);
            buf.set_len(len);
        };
        String::from_utf8(buf).unwrap_or_default()
    }

    /// Convenience method that calls [ValueRef::as_date](crate::std::ValueRef::as_date).
    ///
    /// # Returns
    /// If, for some reason, this StringRef is not a string, returns `-1`,
    /// else returns the parsed Unix timestamp.
    pub fn as_date<T: Default + AsRef<str>>(
        &self,
        format: T,
        locale: Option<T>,
        timezone: Option<T>,
    ) -> f64 {
        self.0.as_date(format, locale, timezone).unwrap_or(-1.0)
    }
}

impl<S> From<S> for StringRef
where
    S: AsRef<str>,
{
    fn from(string: S) -> Self {
        let string_slice = string.as_ref();
        let rid = unsafe { create_string(string_slice.as_ptr(), string_slice.len()) };
        Self(ValueRef::new(rid))
    }
}

impl Clone for StringRef {
    fn clone(&self) -> Self {
        let rid = unsafe { copy(self.0 .0) };
        Self(ValueRef::new(rid))
    }
}

impl Default for StringRef {
    /// Creates an empty StringRef.
    fn default() -> Self {
        StringRef::from("")
    }
}

// =========================
//        Array Ref
// =========================
impl ArrayRef {
    pub fn new() -> Self {
        let rid = unsafe { create_array() };
        Self(ValueRef::new(rid), 0)
    }

    pub fn len(&self) -> usize {
        unsafe { array_len(self.0 .0) }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get(&self, index: usize) -> ValueRef {
        let rid = unsafe { array_get(self.0 .0, index) };
        ValueRef::new(rid)
    }

    pub fn set(&mut self, index: usize, value: ValueRef) {
        unsafe { array_set(self.0 .0, index, value.0) };
    }

    pub fn insert(&mut self, value: ValueRef) {
        unsafe { array_append(self.0 .0, value.0) };
    }

    pub fn remove(&mut self, index: usize) {
        unsafe { array_remove(self.0 .0, index) };
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

impl FromIterator<ValueRef> for ArrayRef {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = ValueRef>,
    {
        let mut array = Self::new();
        for value in iter {
            array.insert(value);
        }
        array
    }
}

impl Clone for ArrayRef {
    fn clone(&self) -> Self {
        let rid = unsafe { copy(self.0 .0) };
        Self(ValueRef::new(rid), self.1)
    }
}

impl Default for ArrayRef {
    /// Creates an empty ArrayRef.
    fn default() -> Self {
        Self::new()
    }
}

// =========================
//        Object Ref
// =========================
impl ObjectRef {
    pub fn new() -> Self {
        let rid = unsafe { create_object() };
        Self(ValueRef::new(rid))
    }

    pub fn len(&self) -> usize {
        unsafe { object_len(self.0 .0) }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get(&self, key: &str) -> ValueRef {
        let rid = unsafe { object_get(self.0 .0, key.as_ptr(), key.len()) };
        ValueRef::new(rid)
    }

    pub fn set(&mut self, key: &str, value: ValueRef) {
        unsafe { object_set(self.0 .0, key.as_ptr(), key.len(), value.0) };
    }

    pub fn remove(&mut self, key: &str) {
        unsafe { object_remove(self.0 .0, key.as_ptr(), key.len()) };
    }

    pub fn keys(&self) -> ArrayRef {
        let rid = unsafe { object_keys(self.0 .0) };
        ArrayRef(ValueRef::new(rid), 0)
    }

    pub fn values(&self) -> ArrayRef {
        let rid = unsafe { object_values(self.0 .0) };
        ArrayRef(ValueRef::new(rid), 0)
    }
}

impl Clone for ObjectRef {
    fn clone(&self) -> Self {
        let rid = unsafe { copy(self.0 .0) };
        Self(ValueRef::new(rid))
    }
}

impl Default for ObjectRef {
    /// Creates an empty ObjectRef.
    fn default() -> Self {
        Self::new()
    }
}
