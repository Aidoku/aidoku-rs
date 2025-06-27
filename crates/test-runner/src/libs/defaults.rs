use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum DefaultValue {
	Data(Vec<u8>),
	Bool(bool),
	Int(i32),
	Float(f32),
	String(String),
	StringArray(Vec<String>),
	Null,
	HashMap(HashMap<String, String>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DefaultKind {
	Data,
	Bool,
	Int,
	Float,
	String,
	StringArray,
	Null,
}

impl From<u8> for DefaultKind {
	fn from(value: u8) -> Self {
		match value {
			0 => Self::Data,
			1 => Self::Bool,
			2 => Self::Int,
			3 => Self::Float,
			4 => Self::String,
			5 => Self::StringArray,
			6 => Self::Null,
			_ => panic!("Invalid default kind"),
		}
	}
}

#[derive(Debug, Clone, Default)]
pub struct UserDefaults(HashMap<String, DefaultValue>);

impl UserDefaults {
	pub fn new() -> Self {
		UserDefaults::default()
	}
}

impl UserDefaults {
	pub fn get(&self, key: &str) -> Option<&DefaultValue> {
		self.0.get(key)
	}

	pub fn get_map(&self, key: &str) -> Option<(&[String], &[String])> {
		let keys = self.get(&format!("{}.keys", key))?;
		let values = self.get(&format!("{}.values", key))?;
		match (keys, values) {
			(DefaultValue::StringArray(keys), DefaultValue::StringArray(values)) => {
				Some((keys, values))
			}
			_ => None,
		}
	}

	pub fn set(&mut self, key: String, value: DefaultValue) {
		if let DefaultValue::HashMap(map) = value {
			// write the keys and values as separate arrays
			let (keys, values): (Vec<_>, Vec<_>) = map.into_iter().unzip();
			self.0
				.insert(format!("{}.keys", key), DefaultValue::StringArray(keys));
			self.0
				.insert(format!("{}.values", key), DefaultValue::StringArray(values));
		} else {
			self.0.insert(key, value);
		}
	}
}
