type Rid = i32;

#[link(wasm_import_module = "html")]
extern "C" {
    fn scraper_parse(data: *const u8, size: usize) -> Rid;
    fn scraper_select(rid: Rid, selector: *const u8, selector_len: usize) -> Rid;
    // ????
    fn scraper_attr(rid: Rid, selector: *const u8, selector_len: usize) -> Rid;
    // Uhh, what?
    fn scraper_text(rid: Rid, buf: *const u8);
    fn scraper_array_size(rid: Rid) -> usize;
    fn scraper_array_get(rid: Rid, index: usize) -> Rid;
    fn scraper_free(rid: Rid);
}

pub struct Node(Rid);

impl Node {
    pub fn new(buf: &[u8]) -> Self {
        let rid = unsafe { scraper_parse(buf.as_ptr(), buf.len()) };
        Self(rid)
    }

    pub fn select(&self, selector: &str) -> Self {
        let rid = unsafe { scraper_select(self.0, selector.as_ptr(), selector.len()) };
        Self(rid)
    }

    pub fn attr<'a>() -> &'a str {
        todo!()
    }

    pub fn text<'a>(&self) -> &'a str {
        todo!()
    }

    pub fn array_len(&self) -> usize {
        unsafe { scraper_array_size(self.0) }
    }

    pub fn array_get(&self, index: usize) -> Option<Self> {
        let rid = unsafe { scraper_array_get(self.0, index) };
        if rid > -1 {
            Some(Self(rid))
        } else {
            None
        }
    }
}

impl Drop for Node {
    fn drop(&mut self) {
        unsafe { scraper_free(self.0) }
    }
}
