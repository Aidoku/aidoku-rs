//! Additional functions for HTML elements.
use crate::alloc::{String, Vec};
use crate::imports::html::{Document, Element, ElementList, Html};

pub trait ElementHelpers {
	/// Get the text of the element(s) and their children.
	///
	/// This is different from [Element::text](crate::imports::html::Element::text)
	/// in that `<p>` and `<br>` are considered and linebreaks will be inserted.
	fn text_with_newlines(&self) -> Option<String>;
}

/// Macro to implement the ElementHelpers trait for types that provide
/// select(), html(), and other HTML element operations
macro_rules! impl_element_helpers {
	($type:ty) => {
		impl ElementHelpers for $type {
			fn text_with_newlines(&self) -> Option<String> {
				let node = self
					.html()
					// replace <br /> with newline marker
					.and_then(|html| Html::parse_fragment(html.replace("<br />", "\\n")).ok())
					.and_then(|html| html.select_first("body"));
				// add newlines surrounding paragraphs
				if let Some(elements) = node.as_ref().and_then(|node| node.select("p")) {
					for mut p in elements.into_iter() {
						let mut new_text = String::from("\\n");
						new_text.push_str(&p.text().unwrap_or_default());
						new_text.push_str("\\n");
						_ = p.set_text(new_text);
					}
				}
				node.and_then(|node| node.text()).map(|text| {
					text
						// replace newline markers with newlines
						.replace("\\n", "\n")
						// normalize lines (remove leading and trailing whitespace)
						.lines()
						.map(str::trim)
						.collect::<Vec<_>>()
						.join("\n")
				})
			}
		}
	};
}

// Implement ElementHelpers for both Element and ElementList
impl_element_helpers!(Element);
impl_element_helpers!(ElementList);

impl ElementHelpers for Document {
	fn text_with_newlines(&self) -> Option<String> {
		self.0.text_with_newlines()
	}
}
