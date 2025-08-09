#![cfg(feature = "helpers")]

use aidoku::helpers::uri::{QueryParameters, SerializeError};
use paste::paste;
use serde::Serialize;

#[test]
fn query_builder() {
	let mut query = QueryParameters::new();
	query.push("name", Some("value"));
	query.push("name2", None);
	query.push(&String::from("send help"), Some("now"));
	query.push("bruh", None);
	assert_eq!(query.to_string(), "name=value&name2&send%20help=now&bruh");

	query.remove_all("name2");
	assert_eq!(query.to_string(), "name=value&send%20help=now&bruh");
}

#[derive(Serialize)]
struct Test<V> {
	key: V,
}

macro_rules! value {
    ($($name:ident($value:expr => $expected:expr))+) => {$(paste! {
		#[test]
		fn [<$name _value>]() {
			assert_eq!(
				QueryParameters::from_data(&Test { key: $value })
					.unwrap()
					.to_string(),
				format!("key={}", $expected)
			);
		}
	})+};

	($($name:ident($value:expr))+) => {
		value! {
			$($name($value => $value.to_string()))+
		}
	};
}
value! {
	bool(true)

	i8(i8::MIN)
	i16(i16::MIN)
	i32(i32::MIN)
	i64(i64::MIN)
	i128(i128::MIN)
	u8(u8::MAX)
	u16(u16::MAX)
	u32(u32::MAX)
	u64(u64::MAX)
	u128(u128::MAX)

	f32(f32::MIN)
	f64(f64::MIN)
}
value! {
	char(' ' => "%20")
	str("a b c" => "a%20b%20c")

	some(Some(' ') => "%20")
}

#[test]
fn none_value() {
	assert_eq!(
		QueryParameters::from_data(&Test { key: None::<()> })
			.unwrap()
			.to_string(),
		"key"
	);
}

#[test]
fn struct_value() {
	#[derive(Serialize)]
	struct A {
		a: (),
	}
	assert_eq!(
		QueryParameters::from_data(&Test { key: A { a: () } }).unwrap_err(),
		SerializeError::NotTopLevel("A")
	);
}

macro_rules! top_level {
	($($name:ident($value:expr => $type:expr))+) => {$(paste! {
		#[test]
		fn [<top_level_ $name>]() {
			assert_eq!(
				QueryParameters::from_data(&$value).unwrap_err(),
				SerializeError::TopLevel($type)
			);
		}
	})+};

	($($value:literal => $type:ident)+) => {
		top_level! {
			$($type($value => stringify!($type)))+
		}
	};
}
top_level! {
	true => bool

	0_i8 => i8
	0_i16 => i16
	0_i32 => i32
	0_i64 => i64
	0_i128 => i128
	0_u8 => u8
	0_u16 => u16
	0_u32 => u32
	0_u64 => u64
	0_u128 => u128

	0_f32 => f32
	0_f64 => f64

	' ' => char
}
top_level! {
	str("" => "&str")

	none(None::<()> => "Option<T>")
	some(Some(()) => "Option<T>")
}
