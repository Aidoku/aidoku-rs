type Rid = i32;

use super::{StringRef, ArrayRef, ValueRef, destroy};

#[link(wasm_import_module = "html")]
extern "C" {
    #[link_name = "parse"]
    fn scraper_parse(string: *const u8, len: usize) -> i32;
    #[link_name = "parse_fragment"]
    fn scraper_parse_fragment(string: *const u8, len: usize) -> i32;

    #[link_name = "select"]
    fn scraper_select(rid: i32, selector: *const u8, selector_len: usize) -> i32;
    #[link_name = "attr"]
    fn scraper_attr(rid: i32, selector: *const u8, selector_len: usize) -> i32;
    
    #[link_name = "first"]
    fn scraper_first(rid: i32) -> i32;
    #[link_name = "last"]
    fn scraper_last(rid: i32) -> i32;
    #[link_name = "array"]
    fn scraper_array(rid: i32) -> i32;

    #[link_name = "base_uri"]
    fn scraper_base_uri(rid: i32) -> i32;
    #[link_name = "body"]
    fn scraper_body(rid: i32) -> i32;
    #[link_name = "text"]
    fn scraper_text(rid: i32) -> i32;
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
    pub fn new(buf: &[u8]) -> Self {
        let rid = unsafe { scraper_parse(buf.as_ptr(), buf.len()) };
        Self(rid)
    }

    pub fn new_fragment(buf: &[u8]) -> Self {
        let rid = unsafe { scraper_parse_fragment(buf.as_ptr(), buf.len()) };
        Self(rid)
    }

    pub fn from(rid: Rid) -> Self {
        Self(rid)
    }

    pub fn close(&mut self) {
        drop(self)
    }

    pub fn select(&self, selector: &str) -> Self {
        let rid = unsafe { scraper_select(self.0, selector.as_ptr(), selector.len()) };
        Self(rid)
    }

    pub fn attr(&self, selector: &str) -> StringRef {
        let rid = unsafe { scraper_attr(self.0, selector.as_ptr(), selector.len()) };
        StringRef(ValueRef::new(rid))
    }

    pub fn first(&self) -> Self {
        let rid = unsafe { scraper_first(self.0) };
        Self(rid)
    }

    pub fn last(&self) -> Self {
        let rid = unsafe { scraper_last(self.0) };
        Self(rid)
    }

    pub fn array(&self) -> ArrayRef {
        let rid = unsafe { scraper_array(self.0) };
        ArrayRef(ValueRef::new(rid), 0)
    }

    pub fn base_uri(&self) -> StringRef {
        let rid = unsafe { scraper_base_uri(self.0) };
        StringRef(ValueRef::new(rid))
    }

    pub fn body(&self) -> StringRef {
        let rid = unsafe { scraper_body(self.0) };
        StringRef(ValueRef::new(rid))
    }

    pub fn text(&self) -> StringRef {
        let rid = unsafe { scraper_text(self.0) };
        StringRef(ValueRef::new(rid))
    }

    pub fn html(&self) -> StringRef {
        let rid = unsafe { scraper_html(self.0) };
        StringRef(ValueRef::new(rid))
    }

    pub fn outer_html(&self) -> StringRef {
        let rid = unsafe { scraper_outer_html(self.0) };
        StringRef(ValueRef::new(rid))
    }

    pub fn id(&self) -> StringRef {
        let rid = unsafe { scraper_id(self.0) };
        StringRef(ValueRef::new(rid))
    }

    pub fn tag_name(&self) -> StringRef {
        let rid = unsafe { scraper_tag_name(self.0) };
        StringRef(ValueRef::new(rid))
    }

    pub fn class_name(&self) -> StringRef {
        let rid = unsafe { scraper_class_name(self.0) };
        StringRef(ValueRef::new(rid))
    }

    pub fn has_class(&self, class_name: &str) -> bool {
        unsafe { scraper_has_class(self.0, class_name.as_ptr(), class_name.len()) }
    }

    pub fn has_attr(&self, attr_name: &str) -> bool {
        unsafe { scraper_has_attr(self.0, attr_name.as_ptr(), attr_name.len()) }
    }
}

impl Drop for Node {
    fn drop(&mut self) {
        unsafe { destroy(self.0) }
    }
}
