pub type JsValueRef = i32;

pub enum JsValue {
    Undefined,
}

impl JsValue {
    pub fn from(value_ref: JsValueRef) -> Self {
        Self::Undefined
    }
}
