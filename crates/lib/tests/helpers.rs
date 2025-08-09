#![cfg(feature = "helpers")]

use aidoku::helpers::uri::{QueryParameters, SerializeError};
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

#[test]
fn bool_value() {
	assert_eq!(
		QueryParameters::from_data(&Test { key: true })
			.unwrap()
			.to_string(),
		"key=true"
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
fn top_level_bool() {
	assert_eq!(
		QueryParameters::from_data(&true).unwrap_err(),
		SerializeError::TopLevel("bool")
	);
}
