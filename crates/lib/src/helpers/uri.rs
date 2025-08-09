//! Module for encoding URIs.
//!
//! This module encodes a UTF-8 URI string by replacing each instance of
//! certain characters with an escape sequence representing the UTF-8
//! encoding of the character.
use core::fmt::Display;

extern crate alloc;
use crate::AidokuError;
use alloc::{
	string::{String, ToString},
	vec::Vec,
};
use serde::{
	ser::{
		Error as SerError, SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant,
		SerializeTuple, SerializeTupleStruct, SerializeTupleVariant,
	},
	Serialize, Serializer,
};
use thiserror::Error;
/// Percent-encode an entire URI string that is valid UTF-8.
///
/// `internal_encode_uri` escapes all non-alphanumeric characters not
/// in the `charset` parameter.
///
/// This function is made public for use with a custom unencoded charset.
pub fn internal_encode_uri<T: AsRef<[u8]>>(url: T, charset: T) -> String {
	let bytes = url.as_ref();
	let charset = charset.as_ref();
	let hex = b"0123456789ABCDEF";

	let mut result: Vec<u8> = Vec::with_capacity(bytes.len() * 3);

	for &byte in bytes {
		if byte.is_ascii_alphanumeric() || charset.contains(&byte) {
			result.push(byte);
		} else {
			result.push(b'%');
			result.push(hex[(byte >> 4) as usize]);
			result.push(hex[(byte & 0x0F) as usize]);
		}
	}
	String::from_utf8(result).unwrap_or_default()
}

/// Percent-encode an entire URI string that is valid UTF-8.
///
/// `encode_uri` escapes all characters except `a-z A-Z 0-9 ; , / ? : @ & = +
/// $ - _ . ! ~ * ' ( ) #`.
///
/// # Examples
/// ```
/// use aidoku::helpers::uri::encode_uri;
/// assert_eq!(
///     encode_uri("http://www.example.org/a file with spaces.html"),
///     "http://www.example.org/a%20file%20with%20spaces.html",
/// )
/// ```
pub fn encode_uri<T: AsRef<[u8]>>(url: T) -> String {
	internal_encode_uri(url.as_ref(), b";,/?:@&=+$-_.!~*'()#")
}

/// Percent-encode a URI component string that is valid UTF-8.
///
/// `encode_uri_component` escapes all characters except `a-z A-Z 0-9 - _ . !
/// ~ * ' ( )`.
///
/// # Examples
/// ```
/// use aidoku::helpers::uri::encode_uri_component;
/// assert_eq!(
///     encode_uri_component(";,/?:@&=+$"),
///     "%3B%2C%2F%3F%3A%40%26%3D%2B%24",
/// )
/// ```
pub fn encode_uri_component<T: AsRef<[u8]>>(url: T) -> String {
	internal_encode_uri(url.as_ref(), b"-_.!~*'()")
}

/// Alternating, decoded query names and values.
#[derive(Clone, Debug, Default)]
pub struct QueryParameters {
	params: Vec<(String, Option<String>)>,
}

impl QueryParameters {
	#[inline]
	pub fn new() -> Self {
		QueryParameters { params: Vec::new() }
	}

	#[inline]
	pub fn with_capacity(capacity: usize) -> Self {
		QueryParameters {
			params: Vec::with_capacity(capacity),
		}
	}

	#[inline]
	pub fn len(&self) -> usize {
		self.params.len()
	}

	#[inline]
	pub fn is_empty(&self) -> bool {
		self.params.is_empty()
	}

	/// Percent-encode the query paramter with [encode_uri_component] and
	/// add it to the query string along with a value.
	pub fn push(&mut self, name: &str, value: Option<&str>) {
		self.params
			.push((encode_uri_component(name), value.map(encode_uri_component)));
	}

	/// Percent-encode the query paramter with [encode_uri_component] and
	/// add it to the query string.
	pub fn push_key<K: AsRef<str>>(&mut self, name: K) {
		self.params
			.push((encode_uri_component(name.as_ref()), None));
	}

	/// Add a pre-encoded query parameter to the query string.
	pub fn push_encoded(&mut self, name: &str, value: Option<&str>) {
		self.params
			.push((name.to_string(), value.map(|v| v.to_string())));
	}

	/// Percent-encode the query parameter with [encode_uri_component] and
	/// replace any existing values.
	pub fn set(&mut self, name: &str, value: Option<&str>) {
		self.remove_all(name);
		self.push(name, value);
	}

	/// Replace any existing values with the given pair, without encoding.
	pub fn set_encoded(&mut self, name: &str, value: Option<&str>) {
		self.remove_all(name);
		self.push_encoded(name, value);
	}

	/// Remove all query parameters matching given name.
	pub fn remove_all<T: AsRef<str>>(&mut self, name: T) {
		let name = name.as_ref();
		self.remove_all_encoded(encode_uri_component(name));
	}

	/// Remove all query parameters matching given pre-encoded name.
	pub fn remove_all_encoded<T: AsRef<str>>(&mut self, name: T) {
		let name = name.as_ref();
		self.params.retain(|(n, _)| n != name);
	}

	/// Serialize the given data as a [`QueryParameters`] struct.
	pub fn from_data<T: Serialize>(value: &T) -> Result<Self, SerializeError> {
		let mut query = Self::new();
		value.serialize(&mut query)?;
		Ok(query)
	}
}

impl Display for QueryParameters {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		let mut first = true;
		for (n, v) in &self.params {
			if !first {
				write!(f, "&")?;
			} else {
				first = false;
			}
			write!(f, "{}", n)?;
			if let Some(v) = v {
				write!(f, "={}", v)?;
			}
		}
		Ok(())
	}
}

impl Serializer for &mut QueryParameters {
	type Ok = ();
	type Error = SerializeError;

	type SerializeSeq = Self;
	type SerializeTuple = Self;
	type SerializeTupleStruct = Self;
	type SerializeTupleVariant = Self;
	type SerializeMap = Self;
	type SerializeStruct = Self;
	type SerializeStructVariant = Self;

	/// key1=`true`&key2=`false`&...
	fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
		self.params.try_last_mut("bool")?.1 = Some(if v { "true" } else { "false" }.into());
		Ok(())
	}

	fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
		todo!()
	}

	fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
		todo!()
	}

	fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
		todo!()
	}

	fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
		todo!()
	}

	fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
		todo!()
	}

	fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
		todo!()
	}

	fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
		todo!()
	}

	fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
		todo!()
	}

	fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
		todo!()
	}

	fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
		todo!()
	}

	fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
		todo!()
	}

	fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
		todo!()
	}

	fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
		todo!()
	}

	fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
		todo!()
	}

	fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
	where
		T: ?Sized + Serialize,
	{
		todo!()
	}

	fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
		todo!()
	}

	fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
		todo!()
	}

	fn serialize_unit_variant(
		self,
		name: &'static str,
		variant_index: u32,
		variant: &'static str,
	) -> Result<Self::Ok, Self::Error> {
		todo!()
	}

	fn serialize_newtype_struct<T>(
		self,
		name: &'static str,
		value: &T,
	) -> Result<Self::Ok, Self::Error>
	where
		T: ?Sized + Serialize,
	{
		todo!()
	}

	fn serialize_newtype_variant<T>(
		self,
		name: &'static str,
		variant_index: u32,
		variant: &'static str,
		value: &T,
	) -> Result<Self::Ok, Self::Error>
	where
		T: ?Sized + Serialize,
	{
		todo!()
	}

	fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
		todo!()
	}

	fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
		todo!()
	}

	fn serialize_tuple_struct(
		self,
		name: &'static str,
		len: usize,
	) -> Result<Self::SerializeTupleStruct, Self::Error> {
		todo!()
	}

	fn serialize_tuple_variant(
		self,
		name: &'static str,
		variant_index: u32,
		variant: &'static str,
		len: usize,
	) -> Result<Self::SerializeTupleVariant, Self::Error> {
		todo!()
	}

	fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
		todo!()
	}

	fn serialize_struct(
		self,
		name: &'static str,
		_len: usize,
	) -> Result<Self::SerializeStruct, Self::Error> {
		if !self.params.is_empty() {
			return Err(SerializeError::NotTopLevel(name));
		}

		Ok(self)
	}

	fn serialize_struct_variant(
		self,
		name: &'static str,
		variant_index: u32,
		variant: &'static str,
		len: usize,
	) -> Result<Self::SerializeStructVariant, Self::Error> {
		todo!()
	}
}

impl SerializeSeq for &mut QueryParameters {
	type Ok = ();
	type Error = SerializeError;

	fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
	where
		T: ?Sized + Serialize,
	{
		todo!()
	}

	fn end(self) -> Result<Self::Ok, Self::Error> {
		todo!()
	}
}

impl SerializeTuple for &mut QueryParameters {
	type Ok = ();
	type Error = SerializeError;

	fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
	where
		T: ?Sized + Serialize,
	{
		todo!()
	}

	fn end(self) -> Result<Self::Ok, Self::Error> {
		todo!()
	}
}

impl SerializeTupleStruct for &mut QueryParameters {
	type Ok = ();
	type Error = SerializeError;

	fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
	where
		T: ?Sized + Serialize,
	{
		todo!()
	}

	fn end(self) -> Result<Self::Ok, Self::Error> {
		todo!()
	}
}

impl SerializeTupleVariant for &mut QueryParameters {
	type Ok = ();
	type Error = SerializeError;

	fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
	where
		T: ?Sized + Serialize,
	{
		todo!()
	}

	fn end(self) -> Result<Self::Ok, Self::Error> {
		todo!()
	}
}

impl SerializeMap for &mut QueryParameters {
	type Ok = ();
	type Error = SerializeError;

	fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
	where
		T: ?Sized + Serialize,
	{
		todo!()
	}

	fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
	where
		T: ?Sized + Serialize,
	{
		todo!()
	}

	fn end(self) -> Result<Self::Ok, Self::Error> {
		todo!()
	}
}

impl SerializeStruct for &mut QueryParameters {
	type Ok = ();
	type Error = SerializeError;

	fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
	where
		T: ?Sized + Serialize,
	{
		self.push_key(key);
		value.serialize(&mut **self)
	}

	fn end(self) -> Result<Self::Ok, Self::Error> {
		Ok(())
	}
}

impl SerializeStructVariant for &mut QueryParameters {
	type Ok = ();
	type Error = SerializeError;

	fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
	where
		T: ?Sized + Serialize,
	{
		todo!()
	}

	fn end(self) -> Result<Self::Ok, Self::Error> {
		todo!()
	}
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum SerializeError {
	#[error("{0}")]
	Custom(String),
	#[error("`{0}` can only be serialized at the top level")]
	NotTopLevel(&'static str),
	#[error("cannot serialize `{0}` at the top level")]
	TopLevel(&'static str),
}

impl From<SerializeError> for AidokuError {
	fn from(error: SerializeError) -> Self {
		Self::Message(error.to_string())
	}
}

impl SerError for SerializeError {
	fn custom<T>(msg: T) -> Self
	where
		T: Display,
	{
		Self::Custom(msg.to_string())
	}
}

trait TryParams {
	type Param;
	fn try_last_mut(&mut self, r#type: &'static str) -> Result<&mut Self::Param, SerializeError>;
}

impl<T> TryParams for Vec<T> {
	type Param = T;
	fn try_last_mut(&mut self, r#type: &'static str) -> Result<&mut Self::Param, SerializeError> {
		self.last_mut().ok_or(SerializeError::TopLevel(r#type))
	}
}
