use super::std::Vec;
use aidoku_imports::Rid;

#[link(wasm_import_module = "aidoku")]
extern "C" {
    #[link_name = "listing"]
    fn create_listing(name: *const u8, name_len: usize, flags: i32) -> i32;

    #[link_name = "manga"]
    fn create_manga(
        id: *const u8,
        id_len: usize,
        cover_url: *const u8,
        cover_url_len: usize,
        title: *const u8,
        title_len: usize,
        author: *const u8,
        author_len: usize,
        artist: *const u8,
        artist_len: usize,
        description: *const u8,
        description_len: usize,
        status: MangaStatus,
        categories: *const *const u8,
        category_str_lens: *const usize,
        category_count: usize,
        url: *const u8,
        url_len: usize,
        nsfw: MangaContentRating,
        viewer: MangaViewer,
    ) -> i32;

    #[link_name = "manga_result"]
    fn create_manga_result(manga_array: Rid, has_more: bool) -> i32;

    #[link_name = "chapter"]
    fn create_chapter(
        id: *const u8,
        id_len: usize,
        title: *const u8,
        title_len: usize,
        volume: f32,
        chapter: f32,
        date_updated: i64,
        scanlator: *const u8,
        scanlator_len: usize,
        url: *const u8,
        url_len: usize,
        lang: *const u8,
        lang_len: usize,
    ) -> i32;

    #[link_name = "page"]
    fn create_page(
        index: i32,
        image_url: *const u8,
        image_url_len: usize,
        base64: *const u8,
        base64_len: usize,
        text: *const u8,
        text_len: usize,
    ) -> i32;
}

#[repr(C)]
pub enum FilterType {
    Note = 0,
    Text = 1,
    Check = 2,
    Select = 3,
    Sort = 4,
    SortOption = 5,
    Group = 6,
    Genre = 7,
}

#[repr(C)]
pub enum MangaStatus {
    Unknown = 0,
    Ongoing = 1,
    Completed = 2,
    Cancelled = 3,
    Hiatus = 4,
}

#[repr(C)]
pub enum MangaContentRating {
    Safe = 0,
    Suggestive = 1,
    Nsfw = 2,
}

#[repr(C)]
pub enum MangaViewer {
    Default = 0,
    Rtl = 1,
    Ltr = 2,
    Vertical = 3,
    Scroll = 4,
}

pub struct Filter<'a> {
    pub kind: FilterType,
    pub name: &'a str,
    pub value_ptr: *const i32,
}

pub struct Manga<'a> {
    pub id: &'a str,
    pub cover: &'a str,
    pub title: &'a str,
    pub author: &'a str,
    pub artist: &'a str,
    pub description: &'a str,
    pub status: MangaStatus,
    pub categories: Vec<&'a str>,
    pub url: &'a str,
    pub nsfw: MangaContentRating,
    pub viewer: MangaViewer,
}

pub struct MangaPageResult<'a> {
    pub manga: Vec<Manga<'a>>,
    pub has_more: bool,
}

pub struct Listing<'a> {
    pub name: &'a str,
}

pub struct Chapter<'a> {
    pub id: &'a str,
    pub title: &'a str,
    pub volume: f32,
    pub chapter: f32,
    pub date_updated: i64,
    pub scanlator: &'a str,
    pub url: &'a str,
    pub lang: &'a str,
}

pub struct Page<'a> {
    pub index: i32,
    pub url: &'a str,
    pub base64: &'a str,
    pub text: &'a str,
}

impl<'a> Filter<'a> {
    pub fn create(&self) -> i32 {
        unsafe {
            create_filter(
                self.kind,
                self.name.as_ptr(),
                self.name.len(),
                self.value_ptr,
                0 as *const u8,
            )
        }
    }
}

impl<'a> Listing<'a> {
    pub fn create(self) -> i32 {
        unsafe { create_listing(self.name.as_ptr(), self.name.len(), 0) }
    }
}

impl<'a> Manga<'a> {
    pub fn create(self) -> i32 {
        let categories_ptr = &self
            .categories
            .iter()
            .map(|x| x.as_ptr())
            .collect::<Vec<*const u8>>();
        let category_lens = self
            .categories
            .iter()
            .map(|x| x.len())
            .collect::<Vec<usize>>();
        unsafe {
            create_manga(
                self.id.as_ptr(),
                self.id.len(),
                self.cover.as_ptr(),
                self.cover.len(),
                self.title.as_ptr(),
                self.title.len(),
                self.author.as_ptr(),
                self.author.len(),
                self.artist.as_ptr(),
                self.artist.len(),
                self.description.as_ptr(),
                self.description.len(),
                self.status,
                categories_ptr.as_ptr(),
                category_lens.as_ptr(),
                self.categories.len(),
                self.url.as_ptr(),
                self.url.len(),
                self.nsfw,
                self.viewer,
            )
        }
    }
}

impl<'a> MangaPageResult<'a> {
    pub fn create(self) -> i32 {
        let mut arr = aidoku_imports::ArrayRef::new();
        for manga in self.manga {
            let manga_descriptor = manga.create();
            arr.insert(manga_descriptor.into());
        }
        unsafe { create_manga_result(arr.0, self.has_more) }
    }
}

impl<'a> Chapter<'a> {
    pub fn create(self) -> i32 {
        unsafe {
            create_chapter(
                self.id.as_ptr(),
                self.id.len(),
                self.title.as_ptr(),
                self.title.len(),
                self.volume,
                self.chapter,
                self.date_updated,
                self.scanlator.as_ptr(),
                self.scanlator.len(),
                self.url.as_ptr(),
                self.url.len(),
                self.lang.as_ptr(),
                self.lang.len(),
            )
        }
    }
}

impl<'a> Page<'a> {
    pub fn create(self) -> i32 {
        unsafe {
            create_page(
                self.index,
                self.url.as_ptr(),
                self.url.len(),
                self.base64.as_ptr(),
                self.base64.len(),
                self.text.as_ptr(),
                self.text.len(),
            )
        }
    }
}
