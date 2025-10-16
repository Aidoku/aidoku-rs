//! Error handling for Buny source library functions.
use super::{html::HtmlError, js::JsError, net::RequestError};
use crate::{
	alloc::{string::ToString, String},
};
use core::{fmt::Display, str::Utf8Error};

pub type Result<T> = core::result::Result<T, BunyError>;

/// An error passed back to the source runner.
#[derive(Debug)]
pub enum BunyError {
	/// This feature is unimplemented.
	Unimplemented,
	/// Pass a message back to the app.
	Message(String),
	/// There was an error making a request.
	RequestError(RequestError),
	/// There was an error performing an HTML operation.
	HtmlError(HtmlError),
	/// There was an error performing a JavaScript operation.
	JsError(JsError),
	/// There was an error handling UTF-8 data.
	Utf8Error(Utf8Error),
	#[cfg(feature = "json")]
	/// JSON parsing error.
	JsonParseError(serde_json::Error),
	/// Deserialization error.
	DeserializeError,
}

impl BunyError {
	/// Creates a new message error.
	pub fn message<S: Display>(message: S) -> Self {
		Self::Message(message.to_string())
	}
}

impl From<RequestError> for BunyError {
	fn from(value: RequestError) -> Self {
		Self::RequestError(value)
	}
}

impl From<HtmlError> for BunyError {
	fn from(error: HtmlError) -> BunyError {
		BunyError::HtmlError(error)
	}
}

impl From<JsError> for BunyError {
	fn from(error: JsError) -> BunyError {
		BunyError::JsError(error)
	}
}


impl From<Utf8Error> for BunyError {
	fn from(error: Utf8Error) -> BunyError {
		BunyError::Utf8Error(error)
	}
}

#[cfg(feature = "json")]
impl From<serde_json::Error> for BunyError {
	fn from(error: serde_json::Error) -> BunyError {
		BunyError::JsonParseError(error)
	}
}
