use ego_tree::NodeId;
use scraper::{CaseSensitivity, ElementRef, Html, Selector};

#[derive(Debug, Clone)]
pub struct HtmlElement {
	pub html: Html,
	pub id: NodeId,
}

#[derive(Debug, Clone)]
pub struct HtmlElementList(pub Vec<HtmlElement>);

impl HtmlElement {
	pub fn select(&self, selector: &Selector) -> Option<HtmlElementList> {
		let node = self.html.tree.get(self.id)?;
		let element = ElementRef::wrap(node)?;

		let elements: Vec<HtmlElement> = element
			.select(selector)
			.map(|element| HtmlElement {
				html: self.html.clone(),
				id: element.id(),
			})
			.collect();

		Some(HtmlElementList(elements))
	}

	pub fn select_first(&self, selector: &Selector) -> Option<HtmlElement> {
		let node = self.html.tree.get(self.id)?;
		let element = ElementRef::wrap(node)?;

		element.select(selector).next().map(|element| HtmlElement {
			html: self.html.clone(),
			id: element.id(),
		})
	}

	pub fn attr(&self, name: &str) -> Option<String> {
		let node = self.html.tree.get(self.id)?;
		let element = ElementRef::wrap(node)?;
		element.attr(name).map(|value| value.to_string())
	}

	pub fn text(&self, trimmed: bool) -> Option<String> {
		let result = ElementRef::wrap(self.html.tree.get(self.id)?)?
			.text()
			.collect::<String>();
		if trimmed {
			Some(result.trim().to_string())
		} else {
			Some(result)
		}
	}

	pub fn html(&self) -> Option<String> {
		let node = self.html.tree.get(self.id)?;
		let element = ElementRef::wrap(node)?;
		Some(element.inner_html())
	}

	pub fn outer_html(&self) -> Option<String> {
		let node = self.html.tree.get(self.id)?;
		let element = ElementRef::wrap(node)?;
		Some(element.html())
	}

	pub fn next_sibling(&self) -> Option<HtmlElement> {
		let node = self.html.tree.get(self.id)?;
		let element = ElementRef::wrap(node)?;
		element.next_sibling().map(|element| HtmlElement {
			html: self.html.clone(),
			id: element.id(),
		})
	}

	pub fn prev_sibling(&self) -> Option<HtmlElement> {
		let node = self.html.tree.get(self.id)?;
		let element = ElementRef::wrap(node)?;
		element.prev_sibling().map(|element| HtmlElement {
			html: self.html.clone(),
			id: element.id(),
		})
	}

	pub fn own_text(&self) -> Option<String> {
		ElementRef::wrap(self.html.tree.get(self.id)?)?
			.text()
			.next()
			.map(|text| text.to_string())
	}

	pub fn id(&self) -> Option<String> {
		let node = self.html.tree.get(self.id)?;
		let element = ElementRef::wrap(node)?;
		element.value().id().map(|s| s.to_string())
	}

	pub fn tag_name(&self) -> Option<String> {
		let node = self.html.tree.get(self.id)?;
		let element = ElementRef::wrap(node)?;
		Some(element.value().name().to_string())
	}

	pub fn class_name(&self) -> Option<String> {
		let node = self.html.tree.get(self.id)?;
		let element = ElementRef::wrap(node)?;
		Some(element.value().classes().collect::<Vec<&str>>().join(" "))
	}

	pub fn has_class(&self, class: &str) -> bool {
		let Some(node) = self.html.tree.get(self.id) else {
			return false;
		};
		let Some(element) = ElementRef::wrap(node) else {
			return false;
		};
		element
			.value()
			.has_class(class, CaseSensitivity::AsciiCaseInsensitive)
	}

	pub fn has_attr(&self, name: &str) -> bool {
		let Some(node) = self.html.tree.get(self.id) else {
			return false;
		};
		let Some(element) = ElementRef::wrap(node) else {
			return false;
		};
		element.value().attrs().any(|(k, _)| k == name)
	}
}

impl HtmlElementList {
	pub fn select(&self, selector: &Selector) -> Option<HtmlElementList> {
		let elements: Vec<HtmlElement> = self
			.0
			.iter()
			.filter_map(|element| element.select(selector).map(|e| e.0))
			.flatten()
			.collect();
		Some(HtmlElementList(elements))
	}

	pub fn select_first(&self, selector: &Selector) -> Option<HtmlElement> {
		self.0
			.iter()
			.filter_map(|element| element.select(selector).map(|e| e.0))
			.flatten()
			.next()
	}

	pub fn attr(&self, name: &str) -> Option<String> {
		self.0
			.iter()
			.filter_map(|element| element.attr(name))
			.next()
	}

	pub fn text(&self, trimmed: bool) -> Option<String> {
		Some(
			self.0
				.iter()
				.filter_map(|element| element.text(trimmed))
				.collect::<Vec<String>>()
				.join(" "),
		)
	}

	pub fn html(&self) -> Option<String> {
		Some(
			self.0
				.iter()
				.filter_map(|element| element.html())
				.collect::<Vec<String>>()
				.join("\n"),
		)
	}

	pub fn outer_html(&self) -> Option<String> {
		Some(
			self.0
				.iter()
				.filter_map(|element| element.outer_html())
				.collect::<Vec<String>>()
				.join("\n"),
		)
	}
}
