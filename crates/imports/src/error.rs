use core::str::Utf8Error;

pub type Result<T> = core::result::Result<T, AidokuError>;

pub struct AidokuError {
    pub reason: AidokuErrorKind,
}

pub enum AidokuErrorKind {
    ValueCast(ValueCastError),
    Utf8Error(Utf8Error),
    Unimplemented,
}

pub enum ValueCastError {
    NotArray,
    NotObject,
    NotString,
    NotInt,
    NotFloat,
    NotBool,
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
