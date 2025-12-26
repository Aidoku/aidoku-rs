use super::{HtmlDocument, HtmlElement, HtmlElementList, NetRequest, Rid};
use boa_engine::Context;
use font_kit::font::Font;
use raqote::DrawTarget;
use serde::Serialize;
use std::collections::HashMap;

pub struct ImageData {
	pub data: Vec<u8>,
	pub width: i32,
	pub height: i32,
}

pub enum StoreItem {
	String(String),
	Request(Box<NetRequest>),
	HtmlDocument(HtmlDocument),
	HtmlElement(HtmlElement),
	HtmlElementList(HtmlElementList),
	JsContext(Box<Context>),
	Encoded(Vec<u8>),
	Canvas(Box<DrawTarget>),
	Font(Font),
	ImageData(ImageData),
}

// boa's context is not thread safe, but this shouldn't be in multiple threads anyways
unsafe impl Send for StoreItem {}
unsafe impl Sync for StoreItem {}

impl StoreItem {
	pub fn as_string(&self) -> Option<&String> {
		if let StoreItem::String(s) = self {
			Some(s)
		} else {
			None
		}
	}

	pub fn as_request(&mut self) -> Option<&mut NetRequest> {
		if let StoreItem::Request(r) = self {
			Some(r)
		} else {
			None
		}
	}

	pub fn as_html_document(&mut self) -> Option<&mut HtmlDocument> {
		if let StoreItem::HtmlDocument(h) = self {
			Some(h)
		} else {
			None
		}
	}

	pub fn as_html_element(&mut self) -> Option<&mut HtmlElement> {
		if let StoreItem::HtmlElement(h) = self {
			Some(h)
		} else {
			None
		}
	}

	pub fn as_html_element_list(&mut self) -> Option<&mut HtmlElementList> {
		if let StoreItem::HtmlElementList(h) = self {
			Some(h)
		} else {
			None
		}
	}

	pub fn as_js_context(&mut self) -> Option<&mut Context> {
		if let StoreItem::JsContext(c) = self {
			Some(c)
		} else {
			None
		}
	}

	pub fn as_canvas(&mut self) -> Option<&mut DrawTarget> {
		if let StoreItem::Canvas(c) = self {
			Some(c)
		} else {
			None
		}
	}

	pub fn as_font(&self) -> Option<&Font> {
		if let StoreItem::Font(s) = self {
			Some(s)
		} else {
			None
		}
	}

	pub fn as_image_data(&self) -> Option<&ImageData> {
		if let StoreItem::ImageData(s) = self {
			Some(s)
		} else {
			None
		}
	}

	pub fn as_encoded(&self) -> Option<&[u8]> {
		if let StoreItem::Encoded(data) = self {
			Some(data)
		} else if let StoreItem::String(s) = self {
			Some(s.as_bytes())
		} else {
			None
		}
	}
}

pub struct GlobalStore {
	pointer: Rid,
	storage: HashMap<i32, StoreItem>,
}

impl GlobalStore {
	pub fn new() -> Self {
		GlobalStore {
			pointer: 1,
			storage: HashMap::new(),
		}
	}

	pub fn store(&mut self, item: StoreItem) -> Rid {
		let rid = self.pointer;
		self.storage.insert(rid, item);
		self.pointer += 1;
		rid
	}

	pub fn store_encoded<T: Serialize>(&mut self, item: &T) -> Result<i32, postcard::Error> {
		let encoded = postcard::to_allocvec(item)?;
		Ok(self.store(StoreItem::Encoded(encoded)))
	}

	pub fn get(&self, rid: Rid) -> Option<&StoreItem> {
		self.storage.get(&rid)
	}

	pub fn get_mut(&mut self, rid: Rid) -> Option<&mut StoreItem> {
		self.storage.get_mut(&rid)
	}

	pub fn remove(&mut self, rid: Rid) {
		self.storage.remove(&rid);
		if self.storage.is_empty() {
			self.pointer = 1;
		}
	}
}

impl Default for GlobalStore {
	fn default() -> Self {
		GlobalStore::new()
	}
}
