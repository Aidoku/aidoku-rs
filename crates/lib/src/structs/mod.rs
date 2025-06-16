//! Serializable data structures that are sent between Aidoku and sources.

use super::alloc::{String, Vec};
use serde::{Deserialize, Serialize};

pub use hashbrown::HashMap;

pub mod canvas;
mod filter;
mod home;
mod setting;

pub use filter::*;
pub use home::*;
pub use setting::*;

#[cfg(feature = "imports")]
mod source;

#[cfg(feature = "imports")]
pub use source::*;

/// Context associated with a page.
pub type PageContext = HashMap<String, String>;

/// The publishing status of a manga.
#[derive(Default, PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum MangaStatus {
	#[default]
	Unknown = 0,
	Ongoing,
	Completed,
	Cancelled,
	Hiatus,
}

/// The content rating of a manga.
#[derive(Default, PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum ContentRating {
	#[default]
	Unknown = 0,
	Safe,
	Suggestive,
	NSFW,
}

/// The proper reading viewer for a manga.
///
/// This is used for automatic selection of the reader view in Aidoku.
/// `RightToLeft` is used for manga, `LeftToRight` is for western comics,
/// and `Webtoon` is used for manhwa and manhua.
#[derive(Default, PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Viewer {
	#[default]
	Unknown = 0,
	LeftToRight,
	RightToLeft,
	Vertical,
	Webtoon,
}

/// The preferred update strategy for a manga.
///
/// Titles marked as `Always` will be included in library refreshes by default,
/// while `Never` will be excluded. Useful for titles that are known to be fully
/// completed or have a static chapter list that won't change after the initial fetch.
#[derive(Default, PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum UpdateStrategy {
	#[default]
	Always,
	Never,
}

/// A manga, comic, webtoon, or other type of content for Aidoku to read.
#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Manga {
	/// Unique identifier for the manga.
	pub key: String,
	/// Title of the manga.
	pub title: String,
	/// Link to the manga cover image.
	pub cover: Option<String>,
	/// Optional list of artists.
	pub artists: Option<Vec<String>>,
	/// Optional list of authors.
	pub authors: Option<Vec<String>>,
	/// Description of the manga.
	pub description: Option<String>,
	/// Link to the manga on the source website.
	pub url: Option<String>,
	/// Optional list of genres or tags (max: 255).
	pub tags: Option<Vec<String>>,
	/// Publishing status of the manga.
	pub status: MangaStatus,
	/// Content rating of the manga.
	pub content_rating: ContentRating,
	/// Preferred viewer type of the manga.
	pub viewer: Viewer,
	/// Ideal update strategy for the manga.
	pub update_strategy: UpdateStrategy,
	/// Optional date for when the manga should next be updated.
	pub next_update_time: Option<i64>,
	/// List of chapters.
	pub chapters: Option<Vec<Chapter>>,
}

impl Manga {
	/// Copy the values from another manga into this one.
	pub fn copy_from(&mut self, manga: Manga) {
		self.key = manga.key;
		self.title = manga.title;
		if let Some(cover) = manga.cover {
			self.cover = Some(cover);
		}
		if let Some(artists) = manga.artists {
			self.artists = Some(artists);
		}
		if let Some(authors) = manga.authors {
			self.authors = Some(authors);
		}
		if let Some(description) = manga.description {
			self.description = Some(description);
		}
		if let Some(url) = manga.url {
			self.url = Some(url);
		}
		if let Some(tags) = manga.tags {
			self.tags = Some(tags);
		}
		self.status = manga.status;
		self.content_rating = manga.content_rating;
		self.viewer = manga.viewer;
		self.update_strategy = manga.update_strategy;
		if let Some(next_update_time) = manga.next_update_time {
			self.next_update_time = Some(next_update_time);
		}
		if let Some(chapters) = manga.chapters {
			self.chapters = Some(chapters);
		}
	}
}

/// A page of manga entries.
#[derive(Default, Clone, Debug, PartialEq, Serialize)]
pub struct MangaPageResult {
	/// List of manga entries.
	pub entries: Vec<Manga>,
	/// Whether the next page is available or not.
	pub has_next_page: bool,
}

/// A chapter of a manga.
#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Chapter {
	/// Unique identifier for the chapter.
	pub key: String,
	/// Title of the chapter (excluding volume and chapter number).
	pub title: Option<String>,
	/// Chapter number.
	pub chapter_number: Option<f32>,
	/// Volume number.
	pub volume_number: Option<f32>,
	/// Date the chapter was uploaded.
	pub date_uploaded: Option<i64>,
	/// Optional list of groups that scanlated or published the chapter.
	pub scanlators: Option<Vec<String>>,
	/// Link to the chapter on the source website.
	pub url: Option<String>,
	/// Language of the chapter.
	pub language: Option<String>,
	/// Optional thumbnail image url for the chapter.
	pub thumbnail: Option<String>,
	/// Boolean indicating if the chapter is locked.
	pub locked: bool,
}

#[cfg(feature = "imports")]
mod __private {
	use crate::imports::canvas::ImageRef;

	#[derive(Debug, PartialEq)]
	pub struct ImageRefPriv(pub(crate) ImageRef);

	impl serde::Serialize for ImageRefPriv {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: serde::Serializer,
		{
			self.0.serialize(serializer)
		}
	}

	impl<'de> serde::Deserialize<'de> for ImageRefPriv {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: serde::Deserializer<'de>,
		{
			ImageRef::deserialize(deserializer).map(|inner| ImageRefPriv(inner))
		}
	}
}

/// The content of a page.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum PageContent {
	/// A url to an image, with associated context.
	///
	/// The context is sent to the page processor and/or image request modifier
	/// if the source implements either.
	Url(String, Option<PageContext>),
	/// A markdown text string.
	Text(String),
	/// A raw image.
	#[cfg(feature = "imports")]
	Image(__private::ImageRefPriv),
	/// A url to zip archive and a file path to an image inside the archive.
	Zip(String, String),
}

impl PageContent {
	/// Create a new `PageContent` with a url.
	pub fn url<T: Into<String>>(url: T) -> Self {
		Self::Url(url.into(), None)
	}

	/// Create a new `PageContent` with a url and context.
	pub fn url_context<T: Into<String>>(url: T, context: PageContext) -> Self {
		Self::Url(url.into(), Some(context))
	}

	/// Create a new `PageContent` with a markdown text string.
	pub fn text<T: Into<String>>(text: T) -> Self {
		Self::Text(text.into())
	}

	/// Create a new `PageContent` with a raw image.
	#[cfg(feature = "imports")]
	pub fn image(image: crate::imports::canvas::ImageRef) -> Self {
		Self::Image(__private::ImageRefPriv(image))
	}
}

/// A page for a chapter.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Page {
	/// The page content.
	pub content: PageContent,
	/// Optional thumbnail image url for the page.
	pub thumbnail: Option<String>,
	/// If the page has a description.
	pub has_description: bool,
	/// Optional description for the page.
	///
	/// If `has_description` is `true` and this is `None`, [PageDescriptionProvider] will be used.
	/// If `has_description` is `false`, this field will be ignored.
	pub description: Option<String>,
}

impl Page {
	/// Set the page content to be externally managed if it is an image.
	///
	/// This property is exposed for the functions that the [register_source](crate::register_source)
	/// macro generates and should not be used directly.
	#[cfg(feature = "imports")]
	pub fn ensure_externally_managed(&mut self) {
		if let PageContent::Image(ref mut image) = self.content {
			image.0.externally_managed = true;
		}
	}
}

impl Default for Page {
	fn default() -> Self {
		Self {
			content: PageContent::Text(String::default()),
			thumbnail: None,
			has_description: false,
			description: None,
		}
	}
}

/// The display type of a listing.
#[derive(Default, PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum ListingKind {
	#[default]
	Default,
	List,
}

/// A listing of manga.
#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Listing {
	/// Unique identifier for the listing.
	pub id: String,
	/// Title of the listing.
	pub name: String,
	/// Type of listing.
	pub kind: ListingKind,
}
