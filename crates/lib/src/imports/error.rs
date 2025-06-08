//! Error handling for Aidoku source library functions.
use super::{html::HtmlError, js::JsError, net::RequestError};
use crate::{alloc::String, imports::canvas::CanvasError};
use core::str::Utf8Error;

pub type Result<T> = core::result::Result<T, AidokuError>;

/// An error passed back to the source runner.
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum AidokuError {
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
	/// There was an error handling a canvas operation.
	CanvasError(CanvasError),
	/// There was an error handling UTF-8 data.
	Utf8Error(Utf8Error),
	/// JSON parsing error.
	JsonParseError,
	/// Deserialization error.
	DeserializeError,
}

impl AidokuError {
	/// Creates a new message error.
	pub fn message<S: Into<String>>(message: S) -> Self {
		Self::Message(message.into())
	}
}

impl From<RequestError> for AidokuError {
	fn from(value: RequestError) -> Self {
		Self::RequestError(value)
	}
}

impl From<HtmlError> for AidokuError {
	fn from(error: HtmlError) -> AidokuError {
		AidokuError::HtmlError(error)
	}
}

impl From<JsError> for AidokuError {
	fn from(error: JsError) -> AidokuError {
		AidokuError::JsError(error)
	}
}

impl From<CanvasError> for AidokuError {
	fn from(error: CanvasError) -> AidokuError {
		AidokuError::CanvasError(error)
	}
}

impl From<Utf8Error> for AidokuError {
	fn from(error: Utf8Error) -> AidokuError {
		AidokuError::Utf8Error(error)
	}
}
