pub type Result<T> = core::result::Result<T, AidokuError>;

pub struct AidokuError {
    pub reason: AidokuErrorKind,
}

pub enum AidokuErrorKind {
    ValueCast(ValueCastError),
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
