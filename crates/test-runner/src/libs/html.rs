use ego_tree::NodeId;
use scraper::{CaseSensitivity, ElementRef, Html, Selector};
use url::Url;

#[derive(Debug, Clone)]
pub struct HtmlDocument {
	pub html: Html,
	pub base_uri: Option<Url>,
}

#[derive(Debug, Clone)]
pub struct HtmlElement {
	pub html: Html,
	pub id: NodeId,
	pub base_uri: Option<Url>,
}

#[derive(Debug, Clone)]
pub struct HtmlElementList(pub Vec<HtmlElement>);

impl HtmlDocument {
	pub fn parse(html: &str, base_uri: Option<&str>) -> Self {
		let html = Html::parse_document(html);
		let base_uri = base_uri.and_then(|s| Url::parse(s).ok());
		Self { html, base_uri }
	}

	pub fn parse_fragment(html: &str, base_uri: Option<&str>) -> Self {
		let html = Html::parse_fragment(html);
		let base_uri = base_uri.and_then(|s| Url::parse(s).ok());
		Self { html, base_uri }
	}

	pub fn select(&self, selector: &Selector) -> HtmlElementList {
		let elements: Vec<HtmlElement> = self
			.html
			.select_with_root(selector)
			.map(|element| HtmlElement {
				html: self.html.clone(),
				id: element.id(),
				base_uri: self.base_uri.clone(),
			})
			.collect();

		HtmlElementList(elements)
	}
}

impl HtmlElement {
	pub fn select(&self, selector: &Selector) -> Option<HtmlElementList> {
		let node = self.html.tree.get(self.id)?;
		let element = ElementRef::wrap(node)?;

		let elements: Vec<HtmlElement> = element
			.select_with_root(selector)
			.map(|element| HtmlElement {
				html: self.html.clone(),
				id: element.id(),
				base_uri: self.base_uri.clone(),
			})
			.collect();

		Some(HtmlElementList(elements))
	}

	pub fn select_first(&self, selector: &Selector) -> Option<HtmlElement> {
		let node = self.html.tree.get(self.id)?;
		let element = ElementRef::wrap(node)?;

		element
			.select_with_root(selector)
			.next()
			.map(|element| HtmlElement {
				html: self.html.clone(),
				id: element.id(),
				base_uri: self.base_uri.clone(),
			})
	}

	pub fn attr(&self, name: &str) -> Option<String> {
		let has_abs_prefix = name.starts_with("abs:");
		let name = if has_abs_prefix { &name[4..] } else { name };
		let node = self.html.tree.get(self.id)?;
		let element = ElementRef::wrap(node)?;
		let attr = element.attr(name).map(|value| value.to_string());
		if has_abs_prefix {
			if let Some(base_uri) = self.base_uri.as_ref() {
				attr.as_ref()
					// if the attribute is already a url, return it
					.and_then(|value| Url::parse(value).ok())
					.map(|url| url.to_string())
					.or_else(|| {
						// otherwise, try to join it with the base uri
						attr.as_ref()
							.and_then(|value| base_uri.join(value.as_str()).ok())
							.map(|value| value.to_string())
					})
			} else {
				attr
			}
		} else {
			attr
		}
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

	pub fn parent(&self) -> Option<HtmlElement> {
		let node = self.html.tree.get(self.id)?;
		let element = ElementRef::wrap(node)?;
		element.parent().map(|element| HtmlElement {
			html: self.html.clone(),
			id: element.id(),
			base_uri: self.base_uri.clone(),
		})
	}

	pub fn children(&self) -> Option<HtmlElementList> {
		let node = self.html.tree.get(self.id)?;
		let element = ElementRef::wrap(node)?;
		Some(HtmlElementList(
			element
				.child_elements()
				.map(|element| HtmlElement {
					html: self.html.clone(),
					id: element.id(),
					base_uri: self.base_uri.clone(),
				})
				.collect::<Vec<HtmlElement>>(),
		))
	}

	pub fn siblings(&self) -> Option<HtmlElementList> {
		let node = self.html.tree.get(self.id)?;
		let element = ElementRef::wrap(node)?;
		Some(HtmlElementList(
			element
				.next_siblings()
				.map(|element| HtmlElement {
					html: self.html.clone(),
					id: element.id(),
					base_uri: self.base_uri.clone(),
				})
				.collect::<Vec<HtmlElement>>(),
		))
	}

	pub fn next_sibling(&self) -> Option<HtmlElement> {
		let node = self.html.tree.get(self.id)?;
		let element = ElementRef::wrap(node)?;
		element.next_sibling().map(|element| HtmlElement {
			html: self.html.clone(),
			id: element.id(),
			base_uri: self.base_uri.clone(),
		})
	}

	pub fn prev_sibling(&self) -> Option<HtmlElement> {
		let node = self.html.tree.get(self.id)?;
		let element = ElementRef::wrap(node)?;
		element.prev_sibling().map(|element| HtmlElement {
			html: self.html.clone(),
			id: element.id(),
			base_uri: self.base_uri.clone(),
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

pub trait SelectWithRoot<'a, 'b> {
	fn select_with_root(&'a self, selector: &'b Selector) -> SelectRoot<'a, 'b>;
}

pub struct SelectRoot<'a, 'b> {
	root: Option<std::iter::Once<ElementRef<'a>>>,
	select_el: Option<scraper::element_ref::Select<'a, 'b>>,
	select_doc: Option<scraper::html::Select<'a, 'b>>,
}

impl<'a> Iterator for SelectRoot<'a, '_> {
	type Item = ElementRef<'a>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.root.is_some() {
			self.root.take().unwrap().next()
		} else if let Some(select) = self.select_el.as_mut() {
			select.next()
		} else if let Some(select) = self.select_doc.as_mut() {
			select.next()
		} else {
			None
		}
	}
}

impl<'a, 'b> SelectWithRoot<'a, 'b> for Html {
	fn select_with_root(&'a self, selector: &'b Selector) -> SelectRoot<'a, 'b> {
		let root_el = self.tree.root();

		let root = if let Some(element) = ElementRef::wrap(root_el)
			&& selector.matches(&element)
		{
			Some(std::iter::once(element))
		} else {
			None
		};

		let select = self.select(selector);

		SelectRoot {
			root,
			select_el: None,
			select_doc: Some(select),
		}
	}
}

impl<'a, 'b> SelectWithRoot<'a, 'b> for ElementRef<'a> {
	fn select_with_root(&'a self, selector: &'b Selector) -> SelectRoot<'a, 'b> {
		let root = if selector.matches(self) {
			Some(std::iter::once(*self))
		} else {
			None
		};

		let select = self.select(selector);

		SelectRoot {
			root,
			select_el: Some(select),
			select_doc: None,
		}
	}
}
