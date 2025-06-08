#[cfg(feature = "helpers")]
use aidoku::helpers::uri::QueryParameters;

#[cfg(feature = "helpers")]
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
