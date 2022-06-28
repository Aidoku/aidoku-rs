//! A module for working with HTML. It provides a somewhat convenient API
//! for extracting data, using HTML5 DOM methods and CSS selectors.
//!
//! The backend of this module is [SwiftSoup](https://github.com/scinfu/SwiftSoup).
use core::fmt::Display;
use alloc::string::String;

use crate::{
    error::{AidokuError, NodeError, Result},
    std::{copy, destroy, value_kind, ArrayRef, Rid, StringRef, ValueRef},
};

#[link(wasm_import_module = "html")]
extern "C" {
    #[link_name = "parse"]
    fn scraper_parse(string: *const u8, len: usize) -> i32;
    #[link_name = "parse_with_uri"]
    fn scraper_parse_with_uri(
        string: *const u8,
        len: usize,
        base_uri: *const u8,
        base_uri_len: usize,
    ) -> i32;
    #[link_name = "parse_fragment"]
    fn scraper_parse_fragment(string: *const u8, len: usize) -> i32;
    #[link_name = "parse_fragment_with_uri"]
    fn scraper_parse_fragment_with_uri(
        string: *const u8,
        len: usize,
        base_uri: *const u8,
        base_uri_len: usize,
    ) -> i32;

    #[link_name = "select"]
    fn scraper_select(rid: i32, selector: *const u8, selector_len: usize) -> i32;
    #[link_name = "attr"]
    fn scraper_attr(rid: i32, selector: *const u8, selector_len: usize) -> i32;

    #[link_name = "set_text"]
    fn scraper_set_text(rid: i32, text: *const u8, text_len: usize) -> i32;
    #[link_name = "set_html"]
    fn scraper_set_html(rid: i32, html: *const u8, html_len: usize) -> i32;
    #[link_name = "prepend"]
    fn scraper_prepend(rid: i32, html: *const u8, html_len: usize) -> i32;
    #[link_name = "append"]
    fn scraper_append(rid: i32, html: *const u8, html_len: usize) -> i32;

    #[link_name = "first"]
    fn scraper_first(rid: i32) -> i32;
    #[link_name = "last"]
    fn scraper_last(rid: i32) -> i32;
    #[link_name = "next"]
    fn scraper_next(rid: i32) -> i32;
    #[link_name = "previous"]
    fn scraper_previous(rid: i32) -> i32;

    #[link_name = "base_uri"]
    fn scraper_base_uri(rid: i32) -> i32;
    #[link_name = "body"]
    fn scraper_body(rid: i32) -> i32;
    #[link_name = "text"]
    fn scraper_text(rid: i32) -> i32;
    #[link_name = "untrimmed_text"]
    fn scraper_untrimmed_text(rid: i32) -> i32;
    #[link_name = "own_text"]
    fn scraper_own_text(rid: i32) -> i32;
    #[link_name = "data"]
    fn scraper_data(rid: i32) -> i32;
    #[link_name = "array"]
    fn scraper_array(rid: i32) -> i32;
    #[link_name = "html"]
    fn scraper_html(rid: i32) -> i32;
    #[link_name = "outer_html"]
    fn scraper_outer_html(rid: i32) -> i32;

    #[link_name = "escape"]
    fn scraper_escape(rid: i32) -> i32;
    #[link_name = "unescape"]
    fn scraper_unescape(rid: i32) -> i32;

    #[link_name = "id"]
    fn scraper_id(rid: i32) -> i32;
    #[link_name = "tag_name"]
    fn scraper_tag_name(rid: i32) -> i32;
    #[link_name = "class_name"]
    fn scraper_class_name(rid: i32) -> i32;
    #[link_name = "has_class"]
    fn scraper_has_class(rid: i32, class_name: *const u8, class_length: usize) -> bool;
    #[link_name = "has_attr"]
    fn scraper_has_attr(rid: i32, attr_name: *const u8, attr_length: usize) -> bool;
}

/// HTML escape an input string.
/// 
/// # Examples
/// ```ignore
/// assert_eq!(escape_html_entities("<"), "&lt;");
/// ```
pub fn escape_html_entities<T: AsRef<str>>(text: T) -> String {
    let str_ref = StringRef::from(text);
    let rid = unsafe { scraper_escape(str_ref.0 .0) };
    StringRef(ValueRef::new(rid)).read()
}

/// Un-escape an HTML escaped string.
/// 
/// # Examples
/// ```ignore
/// assert_eq!(unescape_html_entities("&lt;"), "<");
/// ```
pub fn unescape_html_entities<T: AsRef<str>>(text: T) -> String {
    let str_ref = StringRef::from(text);
    let rid = unsafe { scraper_unescape(str_ref.0 .0) };
    StringRef(ValueRef::new(rid)).read()
}

/// Type which represents a HTML node, which can be a group of elements,
/// an element, or the entire HTML document.
#[derive(Debug)]
pub struct Node(Rid);

impl Node {
    /// Parse HTML into a Node. As there is no base URI specified, absolute URL
    /// resolution requires the HTML to have a `<base href>` tag.
    pub fn new<T: AsRef<[u8]>>(buf: T) -> Result<Self> {
        let buf = buf.as_ref();
        let rid = unsafe { scraper_parse(buf.as_ptr(), buf.len()) };
        match rid {
            -1 => Err(AidokuError::from(NodeError::ParseError)),
            _ => Ok(Self(rid)),
        }
    }

    /// Parse HTML into a Node. The given `base_uri` will be used for any URLs that
    /// occurs before a `<base href>` tag is defined.
    pub fn new_with_uri<A: AsRef<[u8]>, B: AsRef<str>>(buf: A, base_uri: B) -> Result<Self> {
        let buf = buf.as_ref();
        let base_uri = base_uri.as_ref();
        let rid = unsafe {
            scraper_parse_with_uri(buf.as_ptr(), buf.len(), base_uri.as_ptr(), base_uri.len())
        };
        match rid {
            -1 => Err(AidokuError::from(NodeError::ParseError)),
            _ => Ok(Self(rid)),
        }
    }

    /// Parse a HTML fragment, assuming that it forms the `body` of the HTML.
    /// Similar to [Node::new](crate::html::Node::new), relative URLs will not
    /// be resolved unless there is a `<base href>` tag.
    pub fn new_fragment<T: AsRef<[u8]>>(buf: T) -> Result<Self> {
        let buf = buf.as_ref();
        let rid = unsafe { scraper_parse_fragment(buf.as_ptr(), buf.len()) };
        match rid {
            -1 => Err(AidokuError::from(NodeError::ParseError)),
            _ => Ok(Self(rid)),
        }
    }

    /// Parse a HTML fragment, assuming that it forms the `body` of the HTML.
    /// Similar to [Node::new_with_uri](crate::html::Node::new_with_uri), URL
    /// resolution occurs for any that appears before a `<base href>` tag.
    pub fn new_fragment_with_uri<A: AsRef<[u8]>, B: AsRef<str>>(
        buf: A,
        base_uri: B,
    ) -> Result<Self> {
        let buf = buf.as_ref();
        let base_uri = base_uri.as_ref();
        let rid = unsafe {
            scraper_parse_fragment_with_uri(
                buf.as_ptr(),
                buf.len(),
                base_uri.as_ptr(),
                base_uri.len(),
            )
        };
        match rid {
            -1 => Err(AidokuError::from(NodeError::ParseError)),
            _ => Ok(Self(rid)),
        }
    }

    /// Get an instance from a [Rid](crate::Rid)
    ///
    /// # Safety
    /// Ensure that this Rid is of [Kind::Node](crate::Kind) before
    /// converting.
    #[inline]
    pub unsafe fn from(rid: Rid) -> Self {
        Self(rid)
    }

    #[inline]
    pub fn close(self) {
        drop(self)
    }

    /// Find elements that matches the given CSS (or JQuery) selector.
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
    pub fn select<T: AsRef<str>>(&self, selector: T) -> Self {
        let selector = selector.as_ref();
        let rid = unsafe { scraper_select(self.0, selector.as_ptr(), selector.len()) };
        Self(rid)
    }

    /// Get an attribute value by its key.
    /// To get an absolute URL from an attribute that may be a relative URL,
    /// prefix the key with `abs:`.
    ///
    /// # Example
    /// ```ignore
    /// // Assumes that `el` is a Node
    /// let url = el.attr("abs:src");
    /// ```
    pub fn attr<T: AsRef<str>>(&self, attr: T) -> StringRef {
        let attr = attr.as_ref();
        let rid = unsafe { scraper_attr(self.0, attr.as_ptr(), attr.len()) };
        StringRef(ValueRef::new(rid))
    }

    /// Set the element's inner HTML, clearning the existing HTML.
    ///
    /// # Notice
    /// Internally, this operates on SwiftSoup.Element, but
    /// not on SwiftSoup.Elements, which is the type you usually get when using
    /// methods like [Node::select](crate::html::Node::select). Either use
    /// [Node::array](crate::html::Node::array) to iterate through each element,
    /// or use [Node::first](crate::html::Node::first)/[Node::last](crate::html::Node::last)
    /// to select an element before calling this function.
    pub fn set_html<T: AsRef<str>>(&mut self, html: T) -> Result<()> {
        let html = html.as_ref();
        match unsafe { scraper_set_html(self.0, html.as_ptr(), html.len()) } {
            0 => Ok(()),
            _ => Err(AidokuError::from(NodeError::ModifyError)),
        }
    }

    /// Set the element's text content, clearing any existing content.
    ///
    /// # Notice
    /// Internally, this operates on SwiftSoup.Element, but
    /// not on SwiftSoup.Elements, which is the type you usually get when using
    /// methods like [Node::select](crate::html::Node::select). Either use
    /// [Node::array](crate::html::Node::array) to iterate through each element,
    /// or use [Node::first](crate::html::Node::first)/[Node::last](crate::html::Node::last)
    /// to select an element before calling this function.
    pub fn set_text<T: AsRef<str>>(&mut self, text: T) -> Result<()> {
        let text = text.as_ref();
        match unsafe { scraper_set_text(self.0, text.as_ptr(), text.len()) } {
            0 => Ok(()),
            _ => Err(AidokuError::from(NodeError::ModifyError)),
        }
    }

    /// Add inner HTML into this element. The given HTML will be parsed, and
    /// each node prepended to the start of the element's children.
    ///
    /// # Notice
    /// Internally, this operates on SwiftSoup.Element, but
    /// not on SwiftSoup.Elements, which is the type you usually get when using
    /// methods like [Node::select](crate::html::Node::select). Either use
    /// [Node::array](crate::html::Node::array) to iterate through each element,
    /// or use [Node::first](crate::html::Node::first)/[Node::last](crate::html::Node::last)
    /// to select an element before calling this function.
    pub fn prepend<T: AsRef<str>>(&mut self, html: T) -> Result<()> {
        let html = html.as_ref();
        match unsafe { scraper_prepend(self.0, html.as_ptr(), html.len()) } {
            0 => Ok(()),
            _ => Err(AidokuError::from(NodeError::ModifyError)),
        }
    }

    /// Add inner HTML into this element. The given HTML will be parsed, and
    /// each node appended to the end of the element's children.
    ///
    /// # Notice
    /// Internally, this operates on SwiftSoup.Element, but
    /// not on SwiftSoup.Elements, which is the type you usually get when using
    /// methods like [Node::select](crate::html::Node::select). Either use
    /// [Node::array](crate::html::Node::array) to iterate through each element,
    /// or use [Node::first](crate::html::Node::first)/[Node::last](crate::html::Node::last)
    /// to select an element before calling this function.
    pub fn append<T: AsRef<str>>(&mut self, html: T) -> Result<()> {
        let html = html.as_ref();
        match unsafe { scraper_append(self.0, html.as_ptr(), html.len()) } {
            0 => Ok(()),
            _ => Err(AidokuError::from(NodeError::ModifyError)),
        }
    }

    /// Get the first sibling of this element, which can be this element
    pub fn first(&self) -> Self {
        let rid = unsafe { scraper_first(self.0) };
        Self(rid)
    }

    /// Get the last sibling of this element, which can be this element
    pub fn last(&self) -> Self {
        let rid = unsafe { scraper_last(self.0) };
        Self(rid)
    }

    /// Get the next sibling of the element, returning `None` if there isn't
    /// one.
    pub fn next(&self) -> Option<Node> {
        let rid = unsafe { scraper_next(self.0) };
        match unsafe { value_kind(rid) } {
            crate::Kind::Node => Some(Node(rid)),
            _ => None,
        }
    }

    /// Get the previous sibling of the element, returning `None` if there isn't
    /// one.
    pub fn previous(&self) -> Option<Node> {
        let rid = unsafe { scraper_previous(self.0) };
        match unsafe { value_kind(rid) } {
            crate::Kind::Node => Some(Node(rid)),
            _ => None,
        }
    }

    /// Get the base URI of this Node
    pub fn base_uri(&self) -> StringRef {
        let rid = unsafe { scraper_base_uri(self.0) };
        StringRef(ValueRef::new(rid))
    }

    /// Get the document's `body` element.
    pub fn body(&self) -> StringRef {
        let rid = unsafe { scraper_body(self.0) };
        StringRef(ValueRef::new(rid))
    }

    /// Get the **normalized, combined text** of this element and its children.
    /// Whitespace is normalized and trimmed.
    ///
    /// For example, given HTML `<p>Hello <b>there</b> now! </p>`,
    /// p.text() returns "Hello there now!"
    ///
    /// Note that this method returns text that would be presented to a reader.
    /// The contents of data nodes (e.g. `<script>` tags) are not considered text.
    /// Use [Node::html](crate::html::Node::html) or [Node::data](crate::html::Node::data)
    /// to retrieve that content.
    pub fn text(&self) -> StringRef {
        let rid = unsafe { scraper_text(self.0) };
        StringRef(ValueRef::new(rid))
    }

    /// Get the text of this element and its children. Whitespace is **not** normalized
    /// and trimmed.
    ///
    /// Notices from [Node::text](crate::html::Node::text) applies.
    pub fn untrimmed_text(&self) -> StringRef {
        let rid = unsafe { scraper_untrimmed_text(self.0) };
        StringRef(ValueRef::new(rid))
    }

    /// Gets the (normalized) text owned by this element only; does not get the
    /// combined text of all children.
    ///
    /// Node::own_text only operates on a singular element, so calling it after
    /// [Node::select](crate::html::Node::select) will not work. You need to get
    /// a specific element first, through [Node::array](crate::html::Node::array)
    /// and [ArrayRef::get](crate::std::ArrayRef::get), [Node::first](crate::html::Node::first),
    /// or [Node::last](crate::html::Node::last).
    pub fn own_text(&self) -> StringRef {
        let rid = unsafe { scraper_own_text(self.0) };
        StringRef(ValueRef::new(rid))
    }

    /// Get the combined data of this element. Data is e.g. the inside of a `<script>` tag.
    ///
    /// Note that data is NOT the text of the element. Use [Node::text](crate::html::Node::text)
    /// to get the text that would be visible to a user, and [Node::data](crate::html::Node::data)
    /// for the contents of scripts, comments, CSS styles, etc.
    pub fn data(&self) -> StringRef {
        let rid = unsafe { scraper_data(self.0) };
        StringRef(ValueRef::new(rid))
    }

    /// Get an array of Node. This is most commonly used with
    /// [Node::select](crate::html::Node::select) to iterate through elements
    /// that match a selector.
    pub fn array(&self) -> ArrayRef {
        let rid = unsafe { scraper_array(self.0) };
        ArrayRef::from(ValueRef::new(rid))
    }

    /// Get the node's inner HTML.
    ///
    /// For example, on `<div><p></p></div>`, `div.html()` would return `<p></p>`.
    pub fn html(&self) -> StringRef {
        let rid = unsafe { scraper_html(self.0) };
        StringRef(ValueRef::new(rid))
    }

    /// Get the node's outer HTML.
    ///
    /// For example, on `<div><p></p></div>`, `div.outer_html()` would return
    /// `<div><p></p></div>`.
    pub fn outer_html(&self) -> StringRef {
        let rid = unsafe { scraper_outer_html(self.0) };
        StringRef(ValueRef::new(rid))
    }

    /// Get the node's text and escape any HTML-reserved characters to HTML entities.
    /// 
    /// For example, for a node with text `Hello &<> Å å π 新 there ¾ © »`, 
    /// this would return `Hello &amp;&lt;&gt; Å å π 新 there ¾ © »`
    pub fn escape(&self) -> StringRef {
        let rid: i32 = unsafe { scraper_escape(self.0) };
        StringRef(ValueRef::new(rid))
    }

    /// Get the node's text and unescape any HTML entities to their original characters.
    /// 
    /// For example, for a node with text `Hello &amp;&lt;&gt; Å å π 新 there ¾ © »`,
    /// this would return `Hello &<> Å å π 新 there ¾ © »`.
    pub fn unescape(&self) -> StringRef {
        let rid: i32 = unsafe { scraper_unescape(self.0) };
        StringRef(ValueRef::new(rid))
    }

    /// Get the `id` attribute of this element.
    pub fn id(&self) -> StringRef {
        let rid = unsafe { scraper_id(self.0) };
        StringRef(ValueRef::new(rid))
    }

    /// Get the name of the tag for this element. This will always be the
    /// lowercased version. For example, `<DIV>` and `<div>` would both return
    /// `div`.
    pub fn tag_name(&self) -> StringRef {
        let rid = unsafe { scraper_tag_name(self.0) };
        StringRef(ValueRef::new(rid))
    }

    /// Get the literal value of this node's `class` attribute. For example,
    /// on `<div class="header gray">` this would return `header gray`.
    pub fn class_name(&self) -> StringRef {
        let rid = unsafe { scraper_class_name(self.0) };
        StringRef(ValueRef::new(rid))
    }

    /// Test if this element has a class. Case insensitive.
    pub fn has_class<T: AsRef<str>>(&self, class_name: T) -> bool {
        let class_name = class_name.as_ref();
        unsafe { scraper_has_class(self.0, class_name.as_ptr(), class_name.len()) }
    }

    /// Test if this element has an attribute. Case insensitive.
    pub fn has_attr<T: AsRef<str>>(&self, attr_name: T) -> bool {
        let attr_name = attr_name.as_ref();
        unsafe { scraper_has_attr(self.0, attr_name.as_ptr(), attr_name.len()) }
    }
}

impl Display for Node {
    /// Returns the outer HTML of the node.
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.outer_html().read())
    }
}

impl Drop for Node {
    fn drop(&mut self) {
        unsafe { destroy(self.0) }
    }
}

impl Clone for Node {
    fn clone(&self) -> Self {
        let rid: i32 = unsafe { copy(self.0) };
        Self(rid)
    }
}
