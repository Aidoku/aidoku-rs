use core::str::Utf8Error;

pub type Result<T> = core::result::Result<T, AidokuError>;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct AidokuError {
    pub reason: AidokuErrorKind,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum AidokuErrorKind {
    /// Error when typecasting a ValueRef to their types.
    ValueCast(ValueCastError),

    /// There was an error handling UTF-8 data.
    Utf8Error(Utf8Error),

    /// This feature is unimplemented.
    Unimplemented,

    /// Error when handling HTML content through [html::Node](crate::html::Node)
    NodeError(NodeError),

    /// JSON parsing error.
    JsonParseError,

    /// The defaults key doesn't have a value set.
    DefaultNotFound,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum NodeError {
    /// There was an error parsing HTML.
    ParseError,

    /// There was an error modifying HTML.
    ModifyError,   
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum ValueCastError {
    NotArray,
    NotObject,
    NotString,
    NotInt,
    NotFloat,
    NotBool,
    NotNode,
}

impl From<ValueCastError> for AidokuError {
    fn from(why: ValueCastError) -> Self {
        Self {
            reason: AidokuErrorKind::ValueCast(why),
        }
    }
}

impl From<Utf8Error> for AidokuError {
    fn from(why: Utf8Error) -> Self {
        Self {
            reason: AidokuErrorKind::Utf8Error(why),
        }
    }
}

impl From<NodeError> for AidokuError {
    fn from(why: NodeError) -> Self {
        Self { reason: AidokuErrorKind::NodeError(why) }
    }
}
