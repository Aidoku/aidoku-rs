type Rid = i32;

use super::{copy, destroy, ArrayRef, StringRef, ValueRef, value_kind};

#[link(wasm_import_module = "html")]
extern "C" {
    #[link_name = "parse"]
    fn scraper_parse(string: *const u8, len: usize) -> i32;
    #[link_name = "parse_with_uri"]
    fn scraper_parse_with_uri(string: *const u8, len: usize, base_uri: *const u8, base_uri_len: usize) -> i32;
    #[link_name = "parse_fragment"]
    fn scraper_parse_fragment(string: *const u8, len: usize) -> i32;
    #[link_name = "parse_fragment_with_uri"]
    fn scraper_parse_fragment_with_uri(string: *const u8, len: usize, base_uri: *const u8, base_uri_len: usize) -> i32;

    #[link_name = "select"]
    fn scraper_select(rid: i32, selector: *const u8, selector_len: usize) -> i32;
    #[link_name = "attr"]
    fn scraper_attr(rid: i32, selector: *const u8, selector_len: usize) -> i32;

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
    #[link_name = "array"]
    fn scraper_array(rid: i32) -> i32;
    #[link_name = "html"]
    fn scraper_html(rid: i32) -> i32;
    #[link_name = "outer_html"]
    fn scraper_outer_html(rid: i32) -> i32;

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

pub struct Node(Rid);

impl Node {
    /// Parse HTML into a Node. As there is no base URI specified, absolute URL 
    /// resolution requires the HTML to have a `<base href>` tag.
    pub fn new<T: AsRef<[u8]>>(buf: T) -> Self {
        let buf = buf.as_ref();
        let rid = unsafe { scraper_parse(buf.as_ptr(), buf.len()) };
        Self(rid)
    }

    /// Parse HTML into a Node. The given `base_uri` will be used for any URLs that 
    /// occurs before a `<base href>` tag is defined.
    pub fn new_with_uri<A: AsRef<[u8]>, B: AsRef<str>>(buf: A, base_uri: B) -> Self {
        let buf = buf.as_ref();
        let base_uri = base_uri.as_ref();
        let rid = unsafe { 
            scraper_parse_with_uri(
                buf.as_ptr(),
                buf.len(),
                base_uri.as_ptr(),
                base_uri.len(),
            ) 
        };
        Self(rid)
    }

    /// Parse a HTML fragment, assuming that it forms the `body` of the HTML. Similar to 
    /// [Node::new](aidoku_imports::std::html::Node::new), relative URLs will not be
    /// resolved unless there is a `<base href>` tag.
    pub fn new_fragment<T: AsRef<[u8]>>(buf: T) -> Self {
        let buf = buf.as_ref();
        let rid = unsafe { scraper_parse_fragment(buf.as_ptr(), buf.len()) };
        Self(rid)
    }

    /// Parse a HTML fragment, assuming that it forms the `body` of the HTML. Similar to
    /// [Node::new_with_uri](aidoku_imports::std::html::Node::new_with_uri), URL resolution
    /// occurs for any that appears before a `<base href>` tag.
    pub fn new_fragment_with_uri<A: AsRef<[u8]>, B: AsRef<str>>(buf: A, base_uri: B) -> Self {
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
        Self(rid)
    }

    /// Get an instance from a [Rid](aidoku_imports::std::Rid).
    pub fn from(rid: Rid) -> Self {
        Self(rid)
    }

    pub fn close(self) {
        drop(self)
    }

    /// Find elements that matches the given CSS selector.
    /// Supported selectors can be found [here](https://github.com/scinfu/SwiftSoup#selector-overview).
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

    /// Get the next sibling of the element.
    /// 
    /// # Returns
    /// Returns None if there is no next sibling, else Some(Node).
    pub fn next(&self) -> Option<Node> {
        let rid = unsafe { scraper_next(self.0) };
        match unsafe { value_kind(rid) } {
            crate::Kind::Node => Some(Node(rid)),
            _ => None
        }
    }

    /// Get the previous sibling of the element.
    /// 
    /// # Returns
    /// Returns None if there is no next sibling, else Some(Node).
    pub fn previous(&self) -> Option<Node> {
        let rid = unsafe { scraper_previous(self.0) };
        match unsafe { value_kind(rid) } {
            crate::Kind::Node => Some(Node(rid)),
            _ => None
        }
    }

    /// Get the base URI of this Node
    pub fn base_uri(&self) -> StringRef {
        let rid = unsafe { scraper_base_uri(self.0) };
        StringRef(ValueRef::new(rid))
    }

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
    /// Use [html()](aidoku_imports::html::Node::html) to retrieve that content.
    pub fn text(&self) -> StringRef {
        let rid = unsafe { scraper_text(self.0) };
        StringRef(ValueRef::new(rid))
    }

    /// Get an array of Node
    pub fn array(&self) -> ArrayRef {
        let rid = unsafe { scraper_array(self.0) };
        ArrayRef(ValueRef::new(rid), 0)
    }

    /// Get the node's inner HTML.
    /// For example, on `<div><p></p></div>`, `div.html()` would return `<p></p>`.
    pub fn html(&self) -> StringRef {
        let rid = unsafe { scraper_html(self.0) };
        StringRef(ValueRef::new(rid))
    }

    /// Get the node's outer HTML.
    /// For example, on `<div><p></p></div>`, `div.outer_html()` would return
    /// `<div><p></p></div>`.
    pub fn outer_html(&self) -> StringRef {
        let rid = unsafe { scraper_outer_html(self.0) };
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
