use super::std::{String, Vec};
use aidoku_imports::{ObjectRef, Rid, ValueRef};

#[link(wasm_import_module = "aidoku")]
extern "C" {
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
        url: *const u8,
        url_len: usize,
        categories: *const *const u8,
        category_str_lens: *const usize,
        category_count: usize,
        status: MangaStatus,
        nsfw: MangaContentRating,
        viewer: MangaViewer,
    ) -> i32;

    fn create_manga_result(manga_array: Rid, has_more: bool) -> i32;

    fn create_chapter(
        id: *const u8,
        id_len: usize,
        title: *const u8,
        title_len: usize,
        volume: f32,
        chapter: f32,
        date_updated: f64,
        scanlator: *const u8,
        scanlator_len: usize,
        url: *const u8,
        url_len: usize,
        lang: *const u8,
        lang_len: usize,
    ) -> i32;

    fn create_page(
        index: i32,
        image_url: *const u8,
        image_url_len: usize,
        base64: *const u8,
        base64_len: usize,
        text: *const u8,
        text_len: usize,
    ) -> i32;

    fn create_deeplink(manga: i32, chapter: i32) -> i32;
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub enum FilterType {
    #[default]
    Base = 0,
    Group = 1,
    Text = 2,
    Check = 3,
    Select = 4,
    Sort = 5,
    SortSelection = 6,
    Title = 7,
    Author = 8,
    Genre = 9,
}

impl FilterType {
    pub fn from(value: i32) -> FilterType {
        match value {
            0 => FilterType::Base,
            1 => FilterType::Group,
            2 => FilterType::Text,
            3 => FilterType::Check,
            4 => FilterType::Select,
            5 => FilterType::Sort,
            6 => FilterType::SortSelection,
            7 => FilterType::Title,
            8 => FilterType::Author,
            9 => FilterType::Genre,
            _ => FilterType::Base,
        }
    }

    pub fn to_int(&self) -> i32 {
        match self {
            FilterType::Base => 0,
            FilterType::Group => 1,
            FilterType::Text => 2,
            FilterType::Check => 3,
            FilterType::Select => 4,
            FilterType::Sort => 5,
            FilterType::SortSelection => 6,
            FilterType::Title => 7,
            FilterType::Author => 8,
            FilterType::Genre => 9,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub enum MangaStatus {
    #[default]
    Unknown = 0,
    Ongoing = 1,
    Completed = 2,
    Cancelled = 3,
    Hiatus = 4,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub enum MangaContentRating {
    #[default]
    Safe = 0,
    Suggestive = 1,
    Nsfw = 2,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub enum MangaViewer {
    #[default]
    Default = 0,
    Rtl = 1,
    Ltr = 2,
    Vertical = 3,
    Scroll = 4,
}

#[derive(Clone, Default)]
pub struct Filter {
    pub kind: FilterType,
    pub name: String,
    pub value: ValueRef,
    pub object: ObjectRef,
}

#[derive(Clone, Debug, Default)]
pub struct Manga {
    pub id: String,
    pub cover: String,
    pub title: String,
    pub author: String,
    pub artist: String,
    pub description: String,
    pub url: String,
    pub categories: Vec<String>,
    pub status: MangaStatus,
    pub nsfw: MangaContentRating,
    pub viewer: MangaViewer,
}

#[derive(Clone, Debug, Default)]
pub struct MangaPageResult {
    pub manga: Vec<Manga>,
    pub has_more: bool,
}

#[derive(Clone, Debug)]
pub struct Listing {
    pub name: String,
}

#[derive(Clone, Debug)]
pub struct Chapter {
    pub id: String,
    pub title: String,
    pub volume: f32,
    pub chapter: f32,
    pub date_updated: f64,
    pub scanlator: String,
    pub url: String,
    pub lang: String,
}

impl Default for Chapter {
    fn default() -> Self {
        Chapter {
            id: String::new(),
            title: String::new(),
            volume: -1.0,
            chapter: -1.0,
            date_updated: -1.0,
            scanlator: String::new(),
            url: String::new(),
            lang: String::new(),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Page {
    pub index: i32,
    pub url: String,
    pub base64: String,
    pub text: String,
}

#[derive(Clone, Debug, Default)]
pub struct DeepLink {
    pub manga: Option<Manga>,
    pub chapter: Option<Chapter>,
}

impl Manga {
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
                self.url.as_ptr(),
                self.url.len(),
                categories_ptr.as_ptr(),
                category_lens.as_ptr(),
                self.categories.len(),
                self.status,
                self.nsfw,
                self.viewer,
            )
        }
    }
}

impl MangaPageResult {
    pub fn create(self) -> i32 {
        let mut arr = aidoku_imports::ArrayRef::new();
        for manga in self.manga {
            let manga_descriptor = manga.create();
            arr.insert(ValueRef::new(manga_descriptor));
        }
        unsafe { create_manga_result(arr.0 .0, self.has_more) }
    }
}

impl Chapter {
    #[inline]
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

impl Page {
    #[inline]
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

impl DeepLink {
    pub fn create(self) -> i32 {
        let manga = match self.manga {
            Some(manga) => manga.create(),
            None => -1,
        };
        let chapter = match self.chapter {
            Some(chapter) => chapter.create(),
            None => -1,
        };
        unsafe { create_deeplink(manga, chapter) }
    }
}
