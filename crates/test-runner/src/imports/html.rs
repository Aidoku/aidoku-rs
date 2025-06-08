use crate::{
	libs::{HtmlElement, HtmlElementList, StoreItem},
	FFIResult, Ptr, Rid, WasmEnv,
};
use scraper::{Html, Selector};
use wasmer::FunctionEnvMut;

enum Result {
	// Success,
	InvalidDescriptor,
	InvalidString,
	// InvalidHtml,
	InvalidQuery,
	#[allow(clippy::enum_variant_names)]
	NoResult,
	// SwiftSoupError,
}

impl From<Result> for i32 {
	fn from(result: Result) -> Self {
		match result {
			// Result::Success => 0,
			Result::InvalidDescriptor => -1,
			Result::InvalidString => -2,
			Result::InvalidQuery => -4,
			Result::NoResult => -5,
			// Result::SwiftSoupError => -6,
		}
	}
}

pub fn parse(
	mut env: FunctionEnvMut<WasmEnv>,
	html_ptr: Ptr,
	html_len: u32,
	_base_url_ptr: Ptr,
	_base_url_len: u32,
) -> FFIResult {
	let Ok(text) = env.data().read_string(&env, html_ptr, html_len) else {
		return Result::InvalidString.into();
	};
	// let Ok(base_url) = env.data().read_string(&env, base_url_ptr, base_url_len) else {
	// 	return Result::InvalidString.into();
	// };
	let html = Html::parse_document(&text);
	env.data_mut().store.store(StoreItem::Html(html))
}
pub fn parse_fragment(
	mut env: FunctionEnvMut<WasmEnv>,
	html_ptr: Ptr,
	html_len: u32,
	_base_url_ptr: Ptr,
	_base_url_len: u32,
) -> FFIResult {
	let Ok(text) = env.data().read_string(&env, html_ptr, html_len) else {
		return Result::InvalidString.into();
	};
	// let Ok(base_url) = env.data().read_string(&env, base_url_ptr, base_url_len) else {
	// 	return Result::InvalidString.into();
	// };
	let html = Html::parse_fragment(&text);
	env.data_mut().store.store(StoreItem::Html(html))
}
pub fn escape(mut env: FunctionEnvMut<WasmEnv>, text_ptr: Ptr, text_len: u32) -> FFIResult {
	let Ok(text) = env.data().read_string(&env, text_ptr, text_len) else {
		return Result::InvalidString.into();
	};
	let mut escaped = String::with_capacity(text.len());
	for c in text.chars() {
		match c {
			'&' => escaped.push_str("&amp;"),
			'<' => escaped.push_str("&lt;"),
			'>' => escaped.push_str("&gt;"),
			_ => escaped.push(c),
		}
	}
	env.data_mut().store.store(StoreItem::String(escaped))
}
pub fn unescape(mut env: FunctionEnvMut<WasmEnv>, text_ptr: Ptr, text_len: u32) -> FFIResult {
	let Ok(text) = env.data().read_string(&env, text_ptr, text_len) else {
		return Result::InvalidString.into();
	};

	let mut unescaped = String::with_capacity(text.len());
	let mut chars = text.chars().peekable();

	while let Some(c) = chars.next() {
		if c == '&' {
			let mut entity = String::from("&");
			while let Some(&next_c) = chars.peek() {
				entity.push(next_c);
				if next_c == ';' || entity.len() > 10 {
					chars.next();
					break;
				}
				chars.next();
			}
			match entity.as_str() {
				"&amp;" => unescaped.push('&'),
				"&lt;" => unescaped.push('<'),
				"&gt;" => unescaped.push('>'),
				_ => unescaped.push_str(&entity),
			}
		} else {
			unescaped.push(c);
		}
	}

	env.data_mut().store.store(StoreItem::String(unescaped))
}

pub fn select(
	mut env: FunctionEnvMut<WasmEnv>,
	rid: Rid,
	query_ptr: Ptr,
	query_len: u32,
) -> FFIResult {
	let Ok(string) = env.data().read_string(&env, query_ptr, query_len) else {
		return Result::InvalidString.into();
	};
	let Ok(selector) = Selector::parse(&string) else {
		return Result::InvalidQuery.into();
	};
	let Some(item) = env.data_mut().store.get_mut(rid) else {
		return Result::InvalidDescriptor.into();
	};
	if let Some(html) = item.as_html() {
		let elements: Vec<HtmlElement> = html
			.select(&selector)
			.map(|element| HtmlElement {
				html: html.clone(),
				id: element.id(),
			})
			.collect();
		env.data_mut()
			.store
			.store(StoreItem::HtmlElementList(HtmlElementList(elements)))
	} else if let Some(element) = item.as_html_element() {
		let Some(elements) = element.select(&selector) else {
			return Result::NoResult.into();
		};
		env.data_mut()
			.store
			.store(StoreItem::HtmlElementList(elements))
	} else if let Some(elements) = item.as_html_element_list() {
		let Some(elements) = elements.select(&selector) else {
			return Result::NoResult.into();
		};
		env.data_mut()
			.store
			.store(StoreItem::HtmlElementList(elements))
	} else {
		Result::InvalidDescriptor.into()
	}
}
pub fn select_first(
	mut env: FunctionEnvMut<WasmEnv>,
	rid: Rid,
	query_ptr: Ptr,
	query_len: u32,
) -> FFIResult {
	let Ok(string) = env.data().read_string(&env, query_ptr, query_len) else {
		return Result::InvalidString.into();
	};
	let Ok(selector) = Selector::parse(&string) else {
		return Result::InvalidQuery.into();
	};
	let Some(item) = env.data_mut().store.get_mut(rid) else {
		return Result::InvalidDescriptor.into();
	};
	if let Some(html) = item.as_html() {
		let Some(result) = html
			.select(&selector)
			.map(|element| HtmlElement {
				html: html.clone(),
				id: element.id(),
			})
			.next()
		else {
			return Result::NoResult.into();
		};
		env.data_mut().store.store(StoreItem::HtmlElement(result))
	} else if let Some(element) = item.as_html_element() {
		let Some(result) = element.select_first(&selector) else {
			return Result::NoResult.into();
		};
		env.data_mut().store.store(StoreItem::HtmlElement(result))
	} else if let Some(elements) = item.as_html_element_list() {
		let Some(result) = elements.select_first(&selector) else {
			return Result::NoResult.into();
		};
		env.data_mut().store.store(StoreItem::HtmlElement(result))
	} else {
		Result::InvalidDescriptor.into()
	}
}
pub fn attr(mut env: FunctionEnvMut<WasmEnv>, rid: Rid, key_ptr: u32, key_len: u32) -> FFIResult {
	let Ok(key) = env.data().read_string(&env, key_ptr, key_len) else {
		return Result::InvalidString.into();
	};
	let Some(item) = env.data_mut().store.get_mut(rid) else {
		return Result::InvalidDescriptor.into();
	};
	let attr = if let Some(element) = item.as_html_element() {
		element.attr(&key)
	} else if let Some(elements) = item.as_html_element_list() {
		elements.attr(&key)
	} else {
		return Result::InvalidDescriptor.into();
	};
	if let Some(attr) = attr {
		env.data_mut().store.store(StoreItem::String(attr))
	} else {
		Result::NoResult.into()
	}
}
pub fn text(mut env: FunctionEnvMut<WasmEnv>, rid: Rid) -> FFIResult {
	let Some(item) = env.data_mut().store.get_mut(rid) else {
		return Result::InvalidDescriptor.into();
	};
	let attr = if let Some(element) = item.as_html_element() {
		element.text(true)
	} else if let Some(elements) = item.as_html_element_list() {
		elements.text(true)
	} else {
		return Result::InvalidDescriptor.into();
	};
	if let Some(attr) = attr {
		env.data_mut().store.store(StoreItem::String(attr))
	} else {
		Result::NoResult.into()
	}
}
pub fn untrimmed_text(mut env: FunctionEnvMut<WasmEnv>, rid: Rid) -> FFIResult {
	let Some(item) = env.data_mut().store.get_mut(rid) else {
		return Result::InvalidDescriptor.into();
	};
	let attr = if let Some(element) = item.as_html_element() {
		element.text(false)
	} else if let Some(elements) = item.as_html_element_list() {
		elements.text(false)
	} else {
		return Result::InvalidDescriptor.into();
	};
	if let Some(attr) = attr {
		env.data_mut().store.store(StoreItem::String(attr))
	} else {
		Result::NoResult.into()
	}
}
pub fn html(mut env: FunctionEnvMut<WasmEnv>, rid: Rid) -> FFIResult {
	let Some(item) = env.data_mut().store.get_mut(rid) else {
		return Result::InvalidDescriptor.into();
	};
	let attr = if let Some(element) = item.as_html_element() {
		element.html()
	} else if let Some(elements) = item.as_html_element_list() {
		elements.html()
	} else {
		return Result::InvalidDescriptor.into();
	};
	if let Some(attr) = attr {
		env.data_mut().store.store(StoreItem::String(attr))
	} else {
		Result::NoResult.into()
	}
}
pub fn outer_html(mut env: FunctionEnvMut<WasmEnv>, rid: Rid) -> FFIResult {
	let Some(item) = env.data_mut().store.get_mut(rid) else {
		return Result::InvalidDescriptor.into();
	};
	let attr = if let Some(element) = item.as_html_element() {
		element.outer_html()
	} else if let Some(elements) = item.as_html_element_list() {
		elements.outer_html()
	} else {
		return Result::InvalidDescriptor.into();
	};
	if let Some(attr) = attr {
		env.data_mut().store.store(StoreItem::String(attr))
	} else {
		Result::NoResult.into()
	}
}

pub fn set_text(_env: FunctionEnvMut<WasmEnv>, _rid: Rid, _text: u32, _text_len: u32) -> FFIResult {
	-1
}
pub fn set_html(_env: FunctionEnvMut<WasmEnv>, _rid: Rid, _html: u32, _html_len: u32) -> FFIResult {
	-1
}
pub fn prepend(_env: FunctionEnvMut<WasmEnv>, _rid: Rid, _html: u32, _html_len: u32) -> FFIResult {
	-1
}
pub fn append(_env: FunctionEnvMut<WasmEnv>, _rid: Rid, _html: u32, _html_len: u32) -> FFIResult {
	-1
}
pub fn next(mut env: FunctionEnvMut<WasmEnv>, rid: Rid) -> FFIResult {
	let Some(item) = env.data_mut().store.get_mut(rid) else {
		return Result::InvalidDescriptor.into();
	};
	if let Some(element) = item.as_html_element() {
		let Some(next_element) = element.next_sibling() else {
			return Result::NoResult.into();
		};
		env.data_mut()
			.store
			.store(StoreItem::HtmlElement(next_element))
	} else {
		Result::InvalidDescriptor.into()
	}
}
pub fn previous(mut env: FunctionEnvMut<WasmEnv>, rid: Rid) -> FFIResult {
	let Some(item) = env.data_mut().store.get_mut(rid) else {
		return Result::InvalidDescriptor.into();
	};
	if let Some(element) = item.as_html_element() {
		let Some(prev_element) = element.prev_sibling() else {
			return Result::NoResult.into();
		};
		env.data_mut()
			.store
			.store(StoreItem::HtmlElement(prev_element))
	} else {
		Result::InvalidDescriptor.into()
	}
}
pub fn base_uri(_env: FunctionEnvMut<WasmEnv>, _rid: Rid) -> FFIResult {
	-1
}
pub fn own_text(mut env: FunctionEnvMut<WasmEnv>, rid: Rid) -> FFIResult {
	let Some(item) = env.data_mut().store.get_mut(rid) else {
		return Result::InvalidDescriptor.into();
	};
	if let Some(element) = item.as_html_element() {
		let Some(text) = element.own_text() else {
			return Result::NoResult.into();
		};
		env.data_mut().store.store(StoreItem::String(text))
	} else {
		Result::InvalidDescriptor.into()
	}
}
pub fn data(_env: FunctionEnvMut<WasmEnv>, _rid: Rid) -> FFIResult {
	// i don't think scraper supports this?
	-1
}
pub fn id(mut env: FunctionEnvMut<WasmEnv>, rid: Rid) -> FFIResult {
	let Some(item) = env.data_mut().store.get_mut(rid) else {
		return Result::InvalidDescriptor.into();
	};
	if let Some(element) = item.as_html_element() {
		let Some(id) = element.id() else {
			return Result::NoResult.into();
		};
		env.data_mut().store.store(StoreItem::String(id))
	} else {
		Result::InvalidDescriptor.into()
	}
}
pub fn tag_name(mut env: FunctionEnvMut<WasmEnv>, rid: Rid) -> FFIResult {
	let Some(item) = env.data_mut().store.get_mut(rid) else {
		return Result::InvalidDescriptor.into();
	};
	if let Some(element) = item.as_html_element() {
		let Some(tag) = element.tag_name() else {
			return Result::NoResult.into();
		};
		env.data_mut().store.store(StoreItem::String(tag))
	} else {
		Result::InvalidDescriptor.into()
	}
}
pub fn class_name(mut env: FunctionEnvMut<WasmEnv>, rid: Rid) -> FFIResult {
	let Some(item) = env.data_mut().store.get_mut(rid) else {
		return Result::InvalidDescriptor.into();
	};
	if let Some(element) = item.as_html_element() {
		let Some(class) = element.class_name() else {
			return Result::NoResult.into();
		};
		env.data_mut().store.store(StoreItem::String(class))
	} else {
		Result::InvalidDescriptor.into()
	}
}
pub fn has_class(
	mut env: FunctionEnvMut<WasmEnv>,
	rid: Rid,
	class_ptr: Ptr,
	class_len: u32,
) -> i32 {
	let Ok(class) = env.data().read_string(&env, class_ptr, class_len) else {
		return Result::InvalidString.into();
	};
	let Some(item) = env.data_mut().store.get_mut(rid) else {
		return Result::InvalidDescriptor.into();
	};
	if let Some(element) = item.as_html_element() {
		if element.has_class(&class) {
			1
		} else {
			0
		}
	} else {
		Result::InvalidDescriptor.into()
	}
}
pub fn has_attr(mut env: FunctionEnvMut<WasmEnv>, rid: Rid, attr_ptr: Ptr, attr_len: u32) -> i32 {
	let Ok(attr) = env.data().read_string(&env, attr_ptr, attr_len) else {
		return Result::InvalidString.into();
	};
	let Some(item) = env.data_mut().store.get_mut(rid) else {
		return Result::InvalidDescriptor.into();
	};
	if let Some(element) = item.as_html_element() {
		if element.has_attr(&attr) {
			1
		} else {
			0
		}
	} else {
		Result::InvalidDescriptor.into()
	}
}

pub fn first(mut env: FunctionEnvMut<WasmEnv>, rid: Rid) -> FFIResult {
	let Some(item) = env.data_mut().store.get_mut(rid) else {
		return Result::InvalidDescriptor.into();
	};
	if let Some(elements) = item.as_html_element_list() {
		let Some(element) = elements.0.first().cloned() else {
			return Result::InvalidDescriptor.into();
		};
		env.data_mut().store.store(StoreItem::HtmlElement(element))
	} else {
		Result::InvalidDescriptor.into()
	}
}
pub fn last(mut env: FunctionEnvMut<WasmEnv>, rid: Rid) -> FFIResult {
	let Some(item) = env.data_mut().store.get_mut(rid) else {
		return Result::InvalidDescriptor.into();
	};
	if let Some(elements) = item.as_html_element_list() {
		let Some(element) = elements.0.last().cloned() else {
			return Result::InvalidDescriptor.into();
		};
		env.data_mut().store.store(StoreItem::HtmlElement(element))
	} else {
		Result::InvalidDescriptor.into()
	}
}
pub fn get(mut env: FunctionEnvMut<WasmEnv>, rid: Rid, index: u32) -> FFIResult {
	let Some(item) = env.data_mut().store.get_mut(rid) else {
		return Result::InvalidDescriptor.into();
	};
	if let Some(elements) = item.as_html_element_list() {
		let Some(element) = elements.0.get(index as usize).cloned() else {
			return Result::InvalidDescriptor.into();
		};
		env.data_mut().store.store(StoreItem::HtmlElement(element))
	} else {
		Result::InvalidDescriptor.into()
	}
}
pub fn size(mut env: FunctionEnvMut<WasmEnv>, rid: Rid) -> FFIResult {
	let Some(item) = env.data_mut().store.get_mut(rid) else {
		return Result::InvalidDescriptor.into();
	};
	if let Some(elements) = item.as_html_element_list() {
		elements.0.len() as i32
	} else {
		Result::InvalidDescriptor.into()
	}
}
