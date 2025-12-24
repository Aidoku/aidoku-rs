//! Module for working with HTML.
//!
//! It provides a convenient API for extracting data, using HTML5
//! DOM methods and CSS selectors.
//!
//! The backend of this module is [SwiftSoup](https://github.com/scinfu/SwiftSoup).
use super::{
	FFIResult, Rid,
	std::{destroy, read_string_and_destroy},
};
use crate::alloc::String;
use core::fmt::Display;

#[link(wasm_import_module = "html")]
unsafe extern "C" {
	fn parse(
		html: *const u8,
		html_len: usize,
		base_url: *const u8,
		base_url_len: usize,
	) -> FFIResult;
	fn parse_fragment(
		html: *const u8,
		html_len: usize,
		base_url: *const u8,
		base_url_len: usize,
	) -> FFIResult;
	fn escape(text: *const u8, text_len: usize) -> FFIResult;
	fn unescape(text: *const u8, text_len: usize) -> FFIResult;

	fn select(rid: Rid, query: *const u8, query_len: usize) -> FFIResult;
	fn select_first(rid: Rid, query: *const u8, query_len: usize) -> FFIResult;
	fn attr(rid: Rid, key: *const u8, key_len: usize) -> FFIResult;
	fn text(rid: Rid) -> FFIResult;
	fn untrimmed_text(rid: Rid) -> FFIResult;
	fn html(rid: Rid) -> FFIResult;
	fn outer_html(rid: Rid) -> FFIResult;

	fn set_text(rid: Rid, text: *const u8, text_len: usize) -> FFIResult;
	fn set_html(rid: Rid, html: *const u8, html_len: usize) -> FFIResult;
	fn prepend(rid: Rid, html: *const u8, html_len: usize) -> FFIResult;
	fn append(rid: Rid, html: *const u8, html_len: usize) -> FFIResult;
	fn parent(rid: Rid) -> FFIResult;
	fn children(rid: Rid) -> FFIResult;
	fn siblings(rid: Rid) -> FFIResult;
	fn next(rid: Rid) -> FFIResult;
	fn previous(rid: Rid) -> FFIResult;
	fn base_uri(rid: Rid) -> FFIResult;
	fn own_text(rid: Rid) -> FFIResult;
	fn data(rid: Rid) -> FFIResult;
	fn id(rid: Rid) -> FFIResult;
	fn tag_name(rid: Rid) -> FFIResult;
	fn class_name(rid: Rid) -> FFIResult;
	fn has_class(rid: Rid, class: *const u8, class_len: usize) -> bool;
	fn has_attr(rid: Rid, attr: *const u8, attr_len: usize) -> bool;

	fn first(rid: Rid) -> FFIResult;
	fn last(rid: Rid) -> FFIResult;
	#[allow(clashing_extern_declarations)]
	#[link_name = "get"]
	fn html_get(rid: Rid, index: usize) -> FFIResult;
	fn size(rid: Rid) -> FFIResult;
}

/// Error type for HTML operations.
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum HtmlError {
	InvalidDescriptor,
	InvalidString,
	InvalidHtml,
	InvalidQuery,
	NoResult,
	SwiftSoupError,
}

impl HtmlError {
	fn from(value: FFIResult) -> Option<Self> {
		match value {
			-1 => Some(Self::InvalidDescriptor),
			-2 => Some(Self::InvalidString),
			-3 => Some(Self::InvalidHtml),
			-4 => Some(Self::InvalidQuery),
			-5 => Some(Self::NoResult),
			-6 => Some(Self::SwiftSoupError),
			_ => None,
		}
	}
}

/// Namespace for HTML-related functions.
#[derive(Debug)]
pub struct Html;

impl Html {
	/// Parse HTML into a Document.
	///
	/// As there is no base URL specified, absolute URL resolution requires the
	/// HTML to have a `<base href>` tag.
	pub fn parse<T: AsRef<[u8]>>(html: T) -> Result<Document, HtmlError> {
		let buf = html.as_ref();
		let rid = unsafe { parse(buf.as_ptr(), buf.len(), "".as_ptr(), 0) };
		if let Some(error) = HtmlError::from(rid) {
			Err(error)
		} else {
			Ok(Document(unsafe { Element::from(rid) }))
		}
	}

	/// Parse HTML into a Document, with a base URL.
	///
	/// The given `base_url` will be used for any URLs that occurs before a
	/// `<base href>` tag is defined.
	pub fn parse_with_url<T: AsRef<[u8]>, B: AsRef<str>>(
		html: T,
		base_url: B,
	) -> Result<Document, HtmlError> {
		let buf = html.as_ref();
		let url = base_url.as_ref();
		let rid = unsafe { parse(buf.as_ptr(), buf.len(), url.as_ptr(), url.len()) };
		if let Some(error) = HtmlError::from(rid) {
			Err(error)
		} else {
			Ok(Document(unsafe { Element::from(rid) }))
		}
	}

	/// Parse a HTML fragment, assuming that it forms the `body` of the HTML.
	///
	/// Similar to [Html::parse], relative URLs will not be resolved unless
	/// there is a `<base href>` tag.
	pub fn parse_fragment<T: AsRef<[u8]>>(html: T) -> Result<Document, HtmlError> {
		let buf = html.as_ref();
		let rid = unsafe { parse_fragment(buf.as_ptr(), buf.len(), "".as_ptr(), 0) };
		if let Some(error) = HtmlError::from(rid) {
			Err(error)
		} else {
			Ok(Document(unsafe { Element::from(rid) }))
		}
	}

	/// Parse a HTML fragment, assuming that it forms the `body` of the HTML, with a base URL.
	///
	/// Similar to [Html::parse_with_url], URL resolution occurs for any that appears
	/// before a `<base href>` tag.
	pub fn parse_fragment_with_url<T: AsRef<[u8]>, B: AsRef<str>>(
		html: T,
		base_url: B,
	) -> Result<Document, HtmlError> {
		let buf = html.as_ref();
		let url = base_url.as_ref();
		let rid = unsafe { parse_fragment(buf.as_ptr(), buf.len(), url.as_ptr(), url.len()) };
		if let Some(error) = HtmlError::from(rid) {
			Err(error)
		} else {
			Ok(Document(unsafe { Element::from(rid) }))
		}
	}

	/// Escape any HTML-reserved characters to HTML entities.
	///
	/// # Examples
	/// ```ignore
	/// use aidoku::imports::html::Html;
	/// assert_eq!(
	///     Html::escape("Hello &<> Å å π 新 there ¾ © »"),
	///     "Hello &amp;&lt;&gt; Å å π 新 there ¾ © »",
	/// );
	/// ```
	pub fn escape<T: AsRef<str>>(text: T) -> String {
		let text = text.as_ref();
		let rid = unsafe { escape(text.as_ptr(), text.len()) };
		read_string_and_destroy(rid).unwrap_or_default()
	}

	/// Unescape any HTML entities to their original characters.
	///
	/// # Examples
	/// ```ignore
	/// use aidoku::imports::html::Html;
	/// assert_eq!(
	///     Html::unescape("Hello &amp;&lt;&gt; Å å π 新 there ¾ © »"),
	///     Some("Hello &<> Å å π 新 there ¾ © »".into()),
	/// );
	/// ```
	pub fn unescape<T: AsRef<str>>(text: T) -> Option<String> {
		let text = text.as_ref();
		let rid = unsafe { unescape(text.as_ptr(), text.len()) };
		if HtmlError::from(rid).is_some() {
			return None;
		}
		read_string_and_destroy(rid)
	}
}

/// A complete HTML document.
pub struct Document(pub(crate) Element);

impl Document {
	/// Get an instance from a [Rid].
	pub(crate) unsafe fn from(rid: Rid) -> Self {
		Self(unsafe { Element::from(rid) })
	}

	/// Find elements that match the given CSS (or JQuery) selector.
	///
	/// <details>
	///     <summary>Supported selectors</summary>
	///
	/// | Pattern                 | Matches                                                                                              | Example                                                           |
	/// |-------------------------|------------------------------------------------------------------------------------------------------|-------------------------------------------------------------------|
	/// | `*`                     | any element                                                                                          | `*`                                                               |
	/// | `tag`                   | elements with the given tag name                                                                     | `div`                                                             |
	/// | <code>*\|E</code>       | elements of type E in any namespace (including non-namespaced)                                       | <code>*\|name</code> finds `<fb:name>` and `<name>` elements      |
	/// | <code>ns\|E</code>      | elements of type E in the namespace ns                                                               | <code>fb\|name</code> finds `<fb:name>` elements                  |
	/// | `#id`                   | elements with attribute ID of "id"                                                                   | `div#wrap`, `#logo`                                               |
	/// | `.class`                | elements with a class name of "class"                                                                | `div.left`, `.result`                                             |
	/// | `[attr]`                | elements with an attribute named "attr" (with any value)                                             | `a[href]`, `[title]`                                              |
	/// | `[^attrPrefix]`         | elements with an attribute name starting with "attrPrefix". Use to find elements with HTML5 datasets | `[^data-]`, `div[^data-]`                                         |
	/// | `[attr=val]`            | elements with an attribute named "attr", and value equal to "val"                                    | `img[width=500]`, `a[rel=nofollow]`                               |
	/// | `[attr="val"]`          | elements with an attribute named "attr", and value equal to "val"                                    | `span[hello="Cleveland"][goodbye="Columbus"]`, `a[rel="nofollow"]`|
	/// | `[attr^=valPrefix]`     | elements with an attribute named "attr", and value starting with "valPrefix"                         | `a[href^=http:]`                                                  |
	/// | `[attr$=valSuffix]`     | elements with an attribute named "attr", and value ending with "valSuffix"                           | `img[src$=.png]`                                                  |
	/// | `[attr*=valContaining]` | elements with an attribute named "attr", and value containing "valContaining"                        | `a[href*=/search/]`                                               |
	/// | `[attr~=regex]`         | elements with an attribute named "attr", and value matching the regular expression                   | `img[src~=(?i)\\.(png\|jpe?g)]`                                   |
	/// |                         | The above may be combined in any order                                                               | `div.header[title]`                                               |
	///
	/// ## Combinators
	/// | Pattern   | Matches                                         | Example                     |
	/// |-----------|-------------------------------------------------|-----------------------------|
	/// | `E F`     | an F element descended from an E element        | `div a`, `.logo h1`         |
	/// | `E > F`   | an F direct child of E                          | `ol > li`                   |
	/// | `E + F`   | an F element immediately preceded by sibling E  | `li + li`, `div.head + div` |
	/// | `E ~ F`   | an F element preceded by sibling E              | `h1 ~ p`                    |
	/// | `E, F, G` | all matching elements E, F, or G                | `a[href], div, h3`          |
	///
	/// ## Pseudo selectors
	/// | Pattern              | Matches                                                                                                                                                   | Example                                                                                                                                                      |
	/// |----------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------|--------------------------------------------------------------------------------------------------------------------------------------------------------------|
	/// | `:lt(n)`             | elements whose sibling index is less than n                                                                                                               | `td:lt(3)` finds the first 3 cells of each row                                                                                                               |
	/// | `:gt(n)`             | elements whose sibling index is greater than n                                                                                                            | `td:gt(1)` finds cells after skipping the first two                                                                                                          |
	/// | `:eq(n)`             | elements whose sibling index is equal to n                                                                                                                | `td:eq(0)` finds the first cell of each row                                                                                                                  |
	/// | `:has(selector)`     | elements that contains at least one element matching the selector                                                                                         | `div:has(p)` finds divs that contain p elements; `div:has(> a)` selects div elements that have at least one direct child a element.                          |
	/// | `:not(selector)`     | elements that do not match the selector.                                                                                                                  | `div:not(.logo)` finds all divs that do not have the "logo" class; `div:not(:has(div))` finds divs that do not contain divs.                                 |
	/// | `:contains(text)`    | elements that contains the specified text. The search is case insensitive. The text may appear in the found element, or any of its descendants.           | `p:contains(SwiftSoup)` finds p elements containing the text "SwiftSoup"; `p:contains(hello \(there\))` finds p elements containing the text "Hello (There)" |
	/// | `:matches(regex)`    | elements whose text matches the specified regular expression. The text may appear in the found element, or any of its descendants.                        | `td:matches(\\d+)` finds table cells containing digits. div:matches((?i)login) finds divs containing the text, case insensitively.                           |
	/// | `:containsOwn(text)` | elements that directly contain the specified text. The search is case insensitive. The text must appear in the found element, not any of its descendants. | `p:containsOwn(SwiftSoup)` finds p elements with own text "SwiftSoup".                                                                                       |
	/// | `:matchesOwn(regex)` | elements whose own text matches the specified regular expression. The text must appear in the found element, not any of its descendants.                  | `td:matchesOwn(\\d+)` finds table cells directly containing digits. div:matchesOwn((?i)login) finds divs containing the text, case insensitively.            |
	///
	/// ## Structural pseudo-selectors
	/// | Pattern                   | Matches                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           | Example                                                |
	/// |---------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|--------------------------------------------------------|
	/// | `:root`                   | The element that is the root of the document. In HTML, this is the html element                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |                                                        |                                                                                                                                                                                                 |
	/// | `:nth-child(an+b)`        | elements that have an+b-1 siblings before it in the document tree, for any positive integer or zero value of n, and has a parent element. For values of a and b greater than zero, this effectively divides the element's children into groups of a elements (the last group taking the remainder), and selecting the bth element of each group. For example, this allows the selectors to address every other row in a table, and could be used to alternate the color of paragraph text in a cycle of four. The a and b values must be integers (positive, negative, or zero). The index of the first child of an element is 1. |                                                        |
	/// | `:nth-last-child(an+b)`   | elements that have an+b-1 siblings after it in the document tree. Otherwise like `:nth-child()`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   | `tr:nth-last-child(-n+2)` the last two rows of a table |
	/// | `:nth-of-type(an+b)`      | pseudo-class notation represents an element that has an+b-1 siblings with the same expanded element name before it in the document tree, for any zero or positive integer value of n, and has a parent element                                                                                                                                                                                                                                                                                                                                                                                                                    | `img:nth-of-type(2n+1)`                                |
	/// | `:nth-last-of-type(an+b)` | pseudo-class notation represents an element that has an+b-1 siblings with the same expanded element name after it in the document tree, for any zero or positive integer value of n, and has a parent element                                                                                                                                                                                                                                                                                                                                                                                                                     | `img:nth-last-of-type(2n+1)`                           |
	/// | `:first-child`            | elements that are the first child of some other element.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          | `div > p:first-child`                                  |
	/// | `:last-child`             | elements that are the last child of some other element.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           | `ol > li:last-child`                                   |
	/// | `:first-of-type`          | elements that are the first sibling of its type in the list of children of its parent element                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     | `dl dt:first-of-type`                                  |
	/// | `:last-of-type`           | elements that are the last sibling of its type in the list of children of its parent element                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      | `tr > td:last-of-type`                                 |
	/// | `:only-child`             | elements that have a parent element and whose parent element hasve no other element children                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |                                                        |
	/// | `:only-of-type`           |  an element that has a parent element and whose parent element has no other element children with the same expanded element name                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |                                                        |
	/// | `:empty`                  | elements that have no children at all                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |                                                        |
	/// </details>
	pub fn select<T: AsRef<str>>(&self, css_query: T) -> Option<ElementList> {
		self.0.select(css_query)
	}

	/// Find the first element that matches the given CSS (or JQuery) selector.
	pub fn select_first<T: AsRef<str>>(&self, css_query: T) -> Option<Element> {
		self.0.select_first(css_query)
	}
}

/// A single HTML element.
pub struct Element {
	rid: Rid,
}

impl Element {
	/// Get an instance from a [Rid].
	unsafe fn from(rid: Rid) -> Self {
		Self { rid }
	}

	/// Find elements that match the given CSS (or JQuery) selector.
	pub fn select<T: AsRef<str>>(&self, css_query: T) -> Option<ElementList> {
		let query = css_query.as_ref();
		let rid = unsafe { select(self.rid, query.as_ptr(), query.len()) };
		if HtmlError::from(rid).is_some() {
			return None;
		}
		Some(unsafe { ElementList::from(rid) })
	}

	/// Find the first element that matches the given CSS (or JQuery) selector.
	pub fn select_first<T: AsRef<str>>(&self, css_query: T) -> Option<Element> {
		let query = css_query.as_ref();
		let rid = unsafe { select_first(self.rid, query.as_ptr(), query.len()) };
		if HtmlError::from(rid).is_some() {
			return None;
		}
		Some(unsafe { Element::from(rid) })
	}

	/// Get an attribute value by its key.
	///
	/// To get an absolute URL from an attribute that may be a relative URL,
	/// prefix the key with `abs:`.
	///
	/// # Examples
	/// ```ignore
	/// use aidoku::imports::html::Html;
	/// let html = Html::parse_with_url("<img src=\"/image.jpg\" />", "https://example.com").unwrap();
	/// let el = html.select_first("img").unwrap();
	/// assert_eq!(
	///     el.attr("abs:src"),
	///     Some("https://example.com/image.jpg".into())
	/// );
	/// ```
	pub fn attr<T: AsRef<str>>(&self, attr_name: T) -> Option<String> {
		let attr_name = attr_name.as_ref();
		let rid = unsafe { attr(self.rid, attr_name.as_ptr(), attr_name.len()) };
		if HtmlError::from(rid).is_some() {
			return None;
		}
		read_string_and_destroy(rid)
	}

	/// Get the normalized, combined text of this element and its children.
	///
	/// Whitespace is normalized and trimmed.
	///
	/// Note that this method returns text that would be presented to a reader.
	/// The contents of data nodes (e.g. `<script>` tags) are not considered text,
	/// and instead, [Element::html] or [Element::data] can be used for them.
	///
	/// # Examples
	/// ```ignore
	/// use aidoku::imports::html::Html;
	/// let html = Html::parse("<p>Hello <b>there</b> now! </p>").unwrap();
	/// let el = html.select_first("p").unwrap();
	/// assert_eq!(el.text(), Some("Hello there now!".into()));
	/// ```
	pub fn text(&self) -> Option<String> {
		let rid = unsafe { text(self.rid) };
		if HtmlError::from(rid).is_some() {
			return None;
		}
		read_string_and_destroy(rid)
	}

	/// Get the text of this element and its children.
	///
	/// Whitespace is *not* normalized and trimmed.
	///
	/// Notices from [Element::text] apply.
	///
	/// # Examples
	/// ```ignore
	/// use aidoku::imports::html::Html;
	/// let html = Html::parse("<p>Hello <b>there</b> now! </p>").unwrap();
	/// let el = html.select_first("p").unwrap();
	/// assert_eq!(el.untrimmed_text(), Some("Hello there now! ".into()));
	/// ```
	pub fn untrimmed_text(&self) -> Option<String> {
		let rid = unsafe { untrimmed_text(self.rid) };
		if HtmlError::from(rid).is_some() {
			return None;
		}
		read_string_and_destroy(rid)
	}

	/// Get the element's inner HTML.
	///
	/// # Examples
	/// ```ignore
	/// use aidoku::imports::html::Html;
	/// let html = Html::parse("<div><p></p></div>").unwrap();
	/// let div = html.select_first("div").unwrap();
	/// assert_eq!(div.html(), Some("<p></p>".into()));
	/// ```
	pub fn html(&self) -> Option<String> {
		let rid = unsafe { html(self.rid) };
		if HtmlError::from(rid).is_some() {
			return None;
		}
		read_string_and_destroy(rid)
	}

	/// Get the element's outer HTML.
	///
	/// # Examples
	/// ```ignore
	/// use aidoku::imports::html::Html;
	/// let html = Html::parse("<div><p></p></div>").unwrap();
	/// let div = html.select_first("div").unwrap();
	/// assert_eq!(div.outer_html(), Some("<div><p></p></div>".into()));
	/// ```
	pub fn outer_html(&self) -> Option<String> {
		let rid = unsafe { outer_html(self.rid) };
		if HtmlError::from(rid).is_some() {
			return None;
		}
		read_string_and_destroy(rid)
	}

	/// Get the element's parent element, returning `None` if there isn't one.
	pub fn parent(&self) -> Option<Element> {
		let rid = unsafe { parent(self.rid) };
		if HtmlError::from(rid).is_some() {
			return None;
		}
		Some(unsafe { Element::from(rid) })
	}

	/// Get the element's children elements.
	pub fn children(&self) -> ElementList {
		let rid = unsafe { children(self.rid) };
		unsafe { ElementList::from(rid) }
	}

	/// Get the sibling elements of the element.
	pub fn siblings(&self) -> ElementList {
		let rid = unsafe { siblings(self.rid) };
		unsafe { ElementList::from(rid) }
	}

	/// Get the next sibling of the element, returning `None` if there isn't one.
	pub fn next(&self) -> Option<Element> {
		let rid = unsafe { next(self.rid) };
		if HtmlError::from(rid).is_some() {
			return None;
		}
		Some(unsafe { Element::from(rid) })
	}

	/// Get the previous sibling of the element, returning `None` if there isn't one.
	pub fn prev(&self) -> Option<Element> {
		let rid = unsafe { previous(self.rid) };
		if HtmlError::from(rid).is_some() {
			return None;
		}
		Some(unsafe { Element::from(rid) })
	}

	/// Set the element's text content, clearing any existing content.
	pub fn set_text<T: AsRef<str>>(&mut self, text: T) -> Result<(), HtmlError> {
		let text = text.as_ref();
		let result = unsafe { set_text(self.rid, text.as_ptr(), text.len()) };

		if let Some(error) = HtmlError::from(result) {
			Err(error)
		} else {
			Ok(())
		}
	}

	/// Set the element's inner HTML, clearing the existing HTML.
	pub fn set_html<T: AsRef<str>>(&mut self, text: T) -> Result<(), HtmlError> {
		let text = text.as_ref();
		let result = unsafe { set_html(self.rid, text.as_ptr(), text.len()) };

		if let Some(error) = HtmlError::from(result) {
			Err(error)
		} else {
			Ok(())
		}
	}

	/// Prepend inner HTML into this element.
	///
	/// The given HTML will be parsed, and each node prepended to the start
	/// of the element's children.
	pub fn prepend<T: AsRef<str>>(&mut self, text: T) -> Result<(), HtmlError> {
		let text = text.as_ref();
		let result = unsafe { prepend(self.rid, text.as_ptr(), text.len()) };

		if let Some(error) = HtmlError::from(result) {
			Err(error)
		} else {
			Ok(())
		}
	}

	/// Append inner HTML into this element.
	///
	/// The given HTML will be parsed, and each node appended to the end
	/// of the element's children.
	pub fn append<T: AsRef<str>>(&mut self, text: T) -> Result<(), HtmlError> {
		let text = text.as_ref();
		let result = unsafe { append(self.rid, text.as_ptr(), text.len()) };

		if let Some(error) = HtmlError::from(result) {
			Err(error)
		} else {
			Ok(())
		}
	}

	/// Get the base URI of this Element.
	pub fn base_uri(&self) -> Option<String> {
		let rid = unsafe { base_uri(self.rid) };
		if HtmlError::from(rid).is_some() {
			return None;
		}
		read_string_and_destroy(rid)
	}

	/// Gets the (normalized) text owned by this element.
	pub fn own_text(&self) -> Option<String> {
		let rid = unsafe { own_text(self.rid) };
		if HtmlError::from(rid).is_some() {
			return None;
		}
		read_string_and_destroy(rid)
	}

	/// Get the combined data (e.g. the inside of a `<script>` tag) of this element.
	///
	/// Note that data is NOT the text of the element. Use [Element::text]
	/// to get the text that would be visible to a user, and [Element::data]
	/// for the contents of scripts, comments, CSS styles, etc.
	pub fn data(&self) -> Option<String> {
		let rid = unsafe { data(self.rid) };
		if HtmlError::from(rid).is_some() {
			return None;
		}
		read_string_and_destroy(rid)
	}

	/// Get the `id` attribute of this element.
	pub fn id(&self) -> Option<String> {
		let rid = unsafe { id(self.rid) };
		if HtmlError::from(rid).is_some() {
			return None;
		}
		read_string_and_destroy(rid)
	}

	/// Get the name of the tag for this element.
	///
	/// This will always be the lowercased version. For example, `<DIV>` and
	/// `<div>` would both return `div`.
	pub fn tag_name(&self) -> Option<String> {
		let rid = unsafe { tag_name(self.rid) };
		if HtmlError::from(rid).is_some() {
			return None;
		}
		read_string_and_destroy(rid)
	}

	/// Get the literal value of this node's `class` attribute.
	///
	/// For example, on `<div class="header gray">` this would return `header gray`.
	pub fn class_name(&self) -> Option<String> {
		let rid = unsafe { class_name(self.rid) };
		if HtmlError::from(rid).is_some() {
			return None;
		}
		read_string_and_destroy(rid)
	}

	/// Test if this element has a class. Case insensitive.
	pub fn has_class<T: AsRef<str>>(&self, class_name: T) -> bool {
		let class_name = class_name.as_ref();
		unsafe { has_class(self.rid, class_name.as_ptr(), class_name.len()) }
	}

	/// Test if this element has an attribute. Case insensitive.
	pub fn has_attr<T: AsRef<str>>(&self, attr_name: T) -> bool {
		let attr_name = attr_name.as_ref();
		unsafe { has_attr(self.rid, attr_name.as_ptr(), attr_name.len()) }
	}
}

impl Drop for Element {
	fn drop(&mut self) {
		unsafe { destroy(self.rid) }
	}
}

impl Display for Element {
	/// Returns the outer HTML of the node.
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		write!(f, "{}", self.outer_html().unwrap_or_default())
	}
}

/// A collection of HTML elements.
pub struct ElementList {
	rid: Rid,
	lower_bound: usize,
	upper_bound: usize,
	size: usize,
}

impl ElementList {
	/// Get an instance from a [Rid].
	unsafe fn from(rid: Rid) -> Self {
		let size = unsafe { size(rid) as usize };
		Self {
			rid,
			lower_bound: 0,
			upper_bound: size.wrapping_sub(1),
			size,
		}
	}

	/// Find elements that match the given CSS (or JQuery) selector.
	pub fn select<T: AsRef<str>>(&self, css_query: T) -> Option<ElementList> {
		let query = css_query.as_ref();
		let rid = unsafe { select(self.rid, query.as_ptr(), query.len()) };
		if HtmlError::from(rid).is_some() {
			return None;
		}
		Some(unsafe { ElementList::from(rid) })
	}

	/// Find the first element that matches the given CSS (or JQuery) selector.
	pub fn select_first<T: AsRef<str>>(&self, css_query: T) -> Option<Element> {
		let query = css_query.as_ref();
		let rid = unsafe { select_first(self.rid, query.as_ptr(), query.len()) };
		if HtmlError::from(rid).is_some() {
			return None;
		}
		Some(unsafe { Element::from(rid) })
	}

	/// Get the normalized, combined text of these elements and their children.
	///
	/// See [Element::text].
	pub fn text(&self) -> Option<String> {
		let rid = unsafe { text(self.rid) };
		if HtmlError::from(rid).is_some() {
			return None;
		}
		read_string_and_destroy(rid)
	}

	/// Get the text of these elements and their children.
	///
	/// See [Element::untrimmed_text].
	pub fn untrimmed_text(&self) -> Option<String> {
		let rid = unsafe { untrimmed_text(self.rid) };
		if HtmlError::from(rid).is_some() {
			return None;
		}
		read_string_and_destroy(rid)
	}

	/// Get the combined elements' inner HTML.
	///
	/// See [Element::html].
	pub fn html(&self) -> Option<String> {
		let rid = unsafe { html(self.rid) };
		if HtmlError::from(rid).is_some() {
			return None;
		}
		read_string_and_destroy(rid)
	}

	/// Get the combined elements' outer HTML.
	///
	/// See [Element::outer_html].
	pub fn outer_html(&self) -> Option<String> {
		let rid = unsafe { outer_html(self.rid) };
		if HtmlError::from(rid).is_some() {
			return None;
		}
		read_string_and_destroy(rid)
	}

	/// Get the first element of this element list.
	pub fn first(&self) -> Option<Element> {
		let rid = unsafe { first(self.rid) };
		if HtmlError::from(rid).is_some() {
			return None;
		}
		Some(unsafe { Element::from(rid) })
	}

	/// Get the last element of this element list.
	pub fn last(&self) -> Option<Element> {
		let rid = unsafe { last(self.rid) };
		if HtmlError::from(rid).is_some() {
			return None;
		}
		Some(unsafe { Element::from(rid) })
	}

	/// Get the element at the given index.
	pub fn get(&self, index: usize) -> Option<Element> {
		let rid = unsafe { html_get(self.rid, index) };
		if HtmlError::from(rid).is_some() {
			return None;
		}
		Some(unsafe { Element::from(rid) })
	}

	/// Get the size of this element list.
	pub fn size(&self) -> usize {
		self.size
	}

	/// Check if this element list is empty.
	pub fn is_empty(&self) -> bool {
		self.size() == 0
	}
}

impl Iterator for ElementList {
	type Item = Element;

	fn next(&mut self) -> Option<Self::Item> {
		if self.lower_bound > self.upper_bound || self.upper_bound == usize::MAX {
			return None;
		}
		let value_ref = self.get(self.lower_bound);
		self.lower_bound += 1;
		value_ref
	}
}

impl DoubleEndedIterator for ElementList {
	fn next_back(&mut self) -> Option<Self::Item> {
		if self.lower_bound > self.upper_bound || self.upper_bound == usize::MAX {
			return None;
		}
		let value_ref = self.get(self.upper_bound);
		self.upper_bound = self.upper_bound.wrapping_sub(1);
		value_ref
	}
}

impl Drop for ElementList {
	fn drop(&mut self) {
		unsafe { destroy(self.rid) }
	}
}
