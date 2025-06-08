//! Additional functions for HTML elements.
use crate::alloc::String;
use crate::imports::html::{Document, Element, ElementList, Html};

pub trait ElementHelpers {
	/// Get the text of the element(s) and their children.
	///
	/// This is different from [Element::text](crate::imports::html::Element::text)
	/// in that `<p>` and `<br>` are considered and linebreaks will be inserted.
	fn text_with_newlines(&self) -> String;
}

/// Macro to implement the ElementHelpers trait for types that provide
/// select(), html(), and other HTML element operations
macro_rules! impl_element_helpers {
	($type:ty) => {
		impl ElementHelpers for $type {
			fn text_with_newlines(&self) -> String {
				let has_ps = self.select("p").map(|arr| arr.is_empty()).unwrap_or(true);
				if !has_ps {
					Html::parse_fragment(self.html().unwrap_or_default().replace("<br>", "\\n<br>"))
						.map(|node| {
							let mut ret = String::new();
							while let Some(p) = node.select("p") {
								ret.push_str(
									p.text()
										.unwrap_or_default()
										.replace("\\n", "\n")
										.replace("\n ", "\n")
										.trim(),
								);
								ret.push('\n');
							}
							ret
						})
						.unwrap_or_default()
				} else {
					Html::parse_fragment(self.html().unwrap_or_default().replace("<br>", "\n<br>"))
						.ok()
						.and_then(|v| v.select("body").and_then(|body| body.untrimmed_text()))
						.unwrap_or_default()
				}
			}
		}
	};
}

// Implement ElementHelpers for both Element and ElementList
impl_element_helpers!(Element);
impl_element_helpers!(ElementList);

impl ElementHelpers for Document {
	fn text_with_newlines(&self) -> String {
		self.0.text_with_newlines()
	}
}
