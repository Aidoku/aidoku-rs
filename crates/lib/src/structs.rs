use crate::std::{ObjectRef, Rid, String, ValueRef, Vec};

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

impl From<i32> for FilterType {
    fn from(value: i32) -> Self {
        Self::from(value as i64)
    }
}

impl From<i64> for FilterType {
    fn from(value: i64) -> Self {
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
}

impl FilterType {
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

/// An enum representing the various statuses a manga can have.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub enum MangaStatus {
    /// The manga status cannot be determined.
    #[default]
    Unknown = 0,

    /// A manga that is still releasing chapters/being translated.
    Ongoing = 1,

    /// A manga that has completed production/translation.
    Completed = 2,

    /// A manga that has been cancelled. This could convey the manga
    /// being dropped, or the translation team has stopped working on the manga,
    /// even though the manga itself is still ongoing.
    Cancelled = 3,

    /// The manga is on hiatus. Could happen because the author decided
    /// to get a PS5 and then leave people on a cliffhanger for two years
    /// straight.
    Hiatus = 4,
}

/// An enumeration representing the manga's content rating.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub enum MangaContentRating {
    #[default]
    Safe = 0,
    Suggestive = 1,
    Nsfw = 2,
}

/// An enumeration representing different manga viewers, used to indicate
/// the preferred reading method for this manga.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub enum MangaViewer {
    #[deprecated(
        since = "0.2.0",
        note = "MangaViewer::Default is ignored in the app, and defaults to the RTL viewer."
    )]
    Default = 0,
    #[default]
    Rtl = 1,
    Ltr = 2,
    Vertical = 3,
    Scroll = 4,
}

/// Struct representing a search filter.
#[derive(Clone, Default)]
pub struct Filter {
    /// The filter variant.
    pub kind: FilterType,

    /// The filter's name, which matches the name of the filter in `filters.json`.
    pub name: String,

    /// The filter's value. This is dependent on what the filter type is.
    pub value: ValueRef,

    /// The raw filter object. You can add extra metadata to the JSON object, and then
    /// use this attribute to obtain it.
    pub object: ObjectRef,
}

/// The Manga struct contains information about a manga. Different mangas
/// are differentiated by their ID, and so changing the ID will result in
/// a different manga. Thus, developers should decide on the ID format
/// before publishing their source.
/// 
/// The ID must be unique at the source-level. 
#[derive(Clone, Debug, Default)]
pub struct Manga {
    /// The given identifier of this manga, which can be anything, from a number
    /// to the entire URL.
    pub id: String,

    /// A URL pointing to a thumbnail which can be used to display the manga.
    pub cover: String,

    /// The title of the manga. It can be either the official title, or the localized
    /// title, depending on which one the developer thinks fits best for the source.
    pub title: String,

    /// The name of the manga's author. Multiple authors should be concatenated into
    /// a single string using a delimiter such as a comma.
    pub author: String,

    /// The name of the manga's artist. Multiple artists should be concatenated into
    /// a single string using a delimiter such as a comma.
    pub artist: String,

    /// A description for this manga.
    pub description: String,

    /// The URL for this manga. Will be used for sharing and for opening the in-app
    /// browser.
    pub url: String,

    /// A vector containing all the manga's tags/categories.
    pub categories: Vec<String>,

    /// The status of the manga (completed, ongoing, hiatus, cancelled...).
    pub status: MangaStatus,

    /// The manga's content rating (safe, suggestive or NSFW).
    pub nsfw: MangaContentRating,

    /// The viewer to use for this manga.
    pub viewer: MangaViewer,
}

/// A struct representing a "page" of mangas. There is no limit on how many mangas
/// can a page have, that is up to the source to decide.
#[derive(Clone, Debug, Default)]
pub struct MangaPageResult {
    /// The mangas that were found in the page.
    pub manga: Vec<Manga>,

    /// Whether there are any more pages after this one. Used to determine if
    /// the app should make another request when the user scrolls to the bottom.
    pub has_more: bool,
}

/// Struct containing information about a listing.
#[derive(Clone, Debug)]
pub struct Listing {
    /// The name of the listing.
    pub name: String,
}

/// Struct containing metadata about a chapter. Different chapters are differentiated
/// by their ID. Thus, changing the ID will result in a different chapter, even if the
/// chapters have the same volume/chapter number.
/// 
/// The ID must be unique at the manga level.
#[derive(Clone, Debug)]
pub struct Chapter {
    /// The given identifier of this manga, which can be anything, from a number
    /// to the entire URL.
    pub id: String,

    /// The title of the chapter.
    pub title: String,

    /// The volume that the chapter belongs to.
    pub volume: f32,

    /// The chapter number of the chapter.
    pub chapter: f32,

    /// The publishing date of the chapter.
    pub date_updated: f64,

    /// The scanlator/scanlation group that posted the chapter.
    pub scanlator: String,

    /// The chapter URL, which will be used for sharing in the future.
    pub url: String,
    
    /// The chapter's language. It should be a valid language code.
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

/// Struct representing a manga page.
#[derive(Clone, Debug, Default)]
pub struct Page {
    /// The index of the page, starting from 0.
    pub index: i32,

    /// The URL to the image file representing the page.
    pub url: String,

    /// The base64-encoded data of the page. If you got it from a data URI,
    /// remove everything but the actual base64 data.
    pub base64: String,

    /// The page's text, mostly used for light novels. Aidoku does not support
    /// this feature yet.
    pub text: String,
}

/// Struct representing a deep link. This deep link is used to open a manga
/// from a webpage URL using the `aidoku://` URL scheme.
#[derive(Clone, Debug, Default)]
pub struct DeepLink {
    /// The manga to link to.
    pub manga: Option<Manga>,

    /// The chapter to link to. Currently, this is not implemented, but should
    /// still be provided for futureproofing.
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
