#![cfg(feature = "helpers")]

use buny::{
	helpers::uri::{encode_uri_component, QueryParameters, SerializeError},
	HashMap,
};
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
				$expected
			);
		}
	})+};

	($($name:ident($value:expr))+) => {
		value! {
			$($name($value => format!("key={}", encode_uri_component($value.to_string()))))+
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

	char(' ')
	str("a b c")
}
value! {
	none(None::<()> => "key")
	some(Some(' ') => "key=%20")

	unit(() => "key=")
	unit_struct({
		#[derive(Serialize)]
		struct A;
		A
	} => "key=")

	unit_variant({
		#[derive(Serialize)]
		enum A {
			B,
		}
		A::B
	} => "key=B")

	newtype_struct({
		#[derive(Serialize)]
		struct A(char);
		A(' ')
	} => "key=%20")
	newtype_variant({
	#[derive(Serialize)]
		enum A {
			B(char),
		}
		A::B(' ')
	} => "key=%20")
}

#[test]
fn none_key() {
	let map: HashMap<Option<()>, ()> = [(None, ())].into();
	assert_eq!(
		QueryParameters::from_data(&map).unwrap_err(),
		SerializeError::InvalidKey("Option<T>")
	);
}

#[test]
fn some_key() {
	let map: HashMap<Option<()>, ()> = [(Some(()), ())].into();
	assert_eq!(
		QueryParameters::from_data(&map).unwrap_err(),
		SerializeError::InvalidKey("Option<T>")
	);
}

macro_rules! invalid_value {
	($($name:ident($value:expr => $type:literal))+) => {$(paste! {
		#[test]
		fn [<$name _value>]() {
			assert_eq!(
				QueryParameters::from_data(&Test { key: $value }).unwrap_err(),
				SerializeError::Invalid($type.into())
			);
		}
	})+};
}
invalid_value! {
	seq(vec![()] => "Vec<T>")

	tuple((0, 'a') => "(T0, T1,...)` or `[T, T,...]")
	array([0, 1] => "(T0, T1,...)` or `[T, T,...]")

	tuple_struct({
		#[derive(Serialize)]
		struct A(u8,u8);
		A(0,0)
	} => "A")

	tuple_variant({
		#[derive(Serialize)]
		enum A {
			B(u8, u8),
		}
		A::B(0,0)
	} => "A::B")

	struct_variant({
		#[derive(Serialize)]
		enum A {
			B { b: u8 },
		}
		A::B { b: 0 }
	} => "A::B")
}

#[test]
fn map_value() {
	let map: HashMap<char, Option<bool>> = [('à', Some(true)), ('a', None)].into();
	assert_eq!(
		QueryParameters::from_data(&Test { key: map }).unwrap_err(),
		SerializeError::NotTopLevel("Map<K, V>")
	);
}

#[test]
fn flattened_map_value() {
	#[derive(Serialize)]
	struct A {
		a: char,
		#[serde(flatten)]
		map: HashMap<char, Option<bool>>,
		b: (),
	}
	let result = QueryParameters::from_data(&A {
		a: ' ',
		map: [('ç', Some(true)), ('c', None)].into(),
		b: (),
	})
	.unwrap()
	.to_string();
	// hash map is unordered, so either of these could be the result
	let expected1 = "a=%20&%C3%A7=true&c&b=";
	let expected2 = "a=%20&c&%C3%A7=true&b=";
	assert!(
		result == expected1 || result == expected2,
		"Unexpected result: {result}"
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

#[test]
fn flattened_struct_value() {
	#[derive(Serialize)]
	struct A {
		a: char,
		#[serde(flatten)]
		test: Test<()>,
		b: Option<()>,
	}
	assert_eq!(
		QueryParameters::from_data(&A {
			a: ' ',
			test: Test { key: () },
			b: None
		})
		.unwrap()
		.to_string(),
		"a=%20&key=&b"
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

	unit(() => "()")
	unit_struct({
		#[derive(Serialize)]
		struct A;
		A
	} => "A")

	unit_variant({
		#[derive(Serialize)]
		enum A {
			B,
		}
		A::B
	} => "A")

	newtype_struct({
		#[derive(Serialize)]
		struct A(());
		A(())
	} => "A")
	newtype_variant({
		#[derive(Serialize)]
		enum A {
			B(()),
		}
		A::B(())
	} => "A")
}

#[test]
fn top_level_map() {
	let map: HashMap<char, Option<bool>> = [('à', Some(true)), ('a', None)].into();
	let result = QueryParameters::from_data(&map).unwrap().to_string();
	let expected1 = "%C3%A0=true&a";
	let expected2 = "a&%C3%A0=true";
	assert!(
		result == expected1 || result == expected2,
		"Unexpected result: {result}"
	);
}
