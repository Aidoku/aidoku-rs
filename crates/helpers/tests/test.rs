use aidoku_helpers::substring::Substring;
use aidoku_helpers::uri::QueryParameters;

#[test]
fn substring_before() {
    assert_eq!(
        r#"background-image: url("paper.gif");"#.substring_before(r#"(""#),
        Some("background-image: url"),
    );
    assert_eq!(
        String::from(r#"background-image: url("paper.gif");"#).substring_before(r#"(""#),
        Some("background-image: url"),
    );
    assert_eq!("Löwe 老虎 Léopard Gepardi".substring_before('L'), Some(""));
    assert_eq!("wewewewewewewewew".substring_before("hhhhhh"), None);
}

#[test]
fn substring_before_last() {
    assert_eq!(
        "Baker Betty Botter Bought some Butter".substring_before_last('B'),
        Some("Baker Betty Botter Bought some "),
    )
}

#[test]
fn substring_after() {
    assert_eq!(
        "Löwe 老虎 Léopard Gepardi".substring_after('L'),
        Some("öwe 老虎 Léopard Gepardi"),
    )
}

#[test]
fn substring_after_last() {
    assert_eq!(
        "Baker Betty Botter Bought some Butter".substring_after_last('B'),
        Some("utter")
    )
}

#[test]
fn substring_chaining() {
    assert_eq!(
        r#"background-image: url("paper.gif");"#
            .substring_after(r#"(""#)
            .unwrap_or_default()
            .substring_before(r#"")"#),
        Some("paper.gif"),
    );
}

#[test]
fn query_builder() {
    let mut query = QueryParameters::new();
    query.push("name", Some("value"));
    query.push("name2", None);
    query.push("send help", Some("now"));
    query.push("bruh", None);
    assert_eq!(query.to_string(), "name=value&name2&send%20help=now&bruh");

    query.remove_all("name2");
    assert_eq!(query.to_string(), "name=value&send%20help=now&bruh");
}
