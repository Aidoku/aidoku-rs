use aidoku_helpers::substring::Substring;

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
