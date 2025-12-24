use aidoku_test_runner::libs::HtmlDocument;
use scraper::{ElementRef, Selector};

#[test]
fn test_select_root_matches() {
	let html = HtmlDocument::parse(
		r#"
		<div id="root" class="item">
			<span class="item">Child</span>
		</div>
		"#,
		None,
	);
	let selector = Selector::parse(".item").unwrap();

	// select div.item
	let root = html
		.select(&selector)
		.0
		.into_iter()
		.next()
		.expect("select failed");

	// select .item, which should include div.item and span.item
	let items = root.select(&selector).expect("select failed").0;
	assert_eq!(items.len(), 2);

	let id = items.first().unwrap().id;
	let node = html.html.tree.get(id).unwrap();
	let element = ElementRef::wrap(node).unwrap();
	assert_eq!(element.value().id(), Some("root"));
}
