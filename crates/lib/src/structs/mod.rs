//! Serializable data structures that are sent between Buny and sources.

use super::alloc::{String, Vec};
use serde::{Deserialize, Serialize};

pub use hashbrown::HashMap;

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
pub type ContentURLContext = HashMap<String, String>;

/// The publishing status of a novel.
#[derive(Default, PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum NovelStatus {
	#[default]
	Unknown = 0,
	Ongoing,
	Completed,
	Cancelled,
	Hiatus,
}

/// The content rating of a novel.
#[derive(Default, PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum ContentRating {
	#[default]
	Unknown = 0,
	Safe,
	Suggestive,
	NSFW,
}

/// The proper reading viewer for a novel.
///
/// This is used for automatic selection of the reader view in Buny.
/// `RightToLeft` is used for novel, `LeftToRight` is for western comics,
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

/// The preferred update strategy for a novel.
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

/// A novel, comic, webtoon, or other type of content for Buny to read.
#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Novel {
	/// Unique identifier for the novel.
	pub key: String,
	/// Title of the novel.
	pub title: String,
	/// Link to the novel cover image.
	pub cover: Option<String>,
	// /// Optional list of artists.
	// pub artists: Option<Vec<String>>,
	/// Optional list of authors.
	pub authors: Option<Vec<String>>,
	/// Description of the novel.
	pub description: Option<String>,
	/// Link to the novel on the source website.
	pub url: Option<String>,
	/// Optional list of genres or tags (max: 255).
	pub tags: Option<Vec<String>>,
	/// Publishing status of the novel.
	pub status: NovelStatus,
	/// Content rating of the novel.
	pub content_rating: ContentRating,
	// /// Preferred viewer type of the novel.
	// pub viewer: Viewer,
	/// Ideal update strategy for the novel.
	pub update_strategy: UpdateStrategy,
	/// Optional date for when the novel should next be updated.
	pub next_update_time: Option<i64>,
	/// List of chapters.
	pub chapters: Option<Vec<Chapter>>,
    /// Has more chapters.
    pub has_more_chapters: Option<bool>,
}

impl Novel {
	/// Copy the values from another novel into this one.
	pub fn copy_from(&mut self, novel: Novel) {
		self.key = novel.key;
		self.title = novel.title;
		if let Some(cover) = novel.cover {
			self.cover = Some(cover);
		}
		// if let Some(artists) = novel.artists {
		// 	self.artists = Some(artists);
		// }
		if let Some(authors) = novel.authors {
			self.authors = Some(authors);
		}
		if let Some(description) = novel.description {
			self.description = Some(description);
		}
		if let Some(url) = novel.url {
			self.url = Some(url);
		}
		if let Some(tags) = novel.tags {
			self.tags = Some(tags);
		}
		self.status = novel.status;
		self.content_rating = novel.content_rating;
		// self.viewer = novel.viewer;
		self.update_strategy = novel.update_strategy;
		if let Some(next_update_time) = novel.next_update_time {
			self.next_update_time = Some(next_update_time);
		}
		if let Some(chapters) = novel.chapters {
			self.chapters = Some(chapters);
            self.has_more_chapters = novel.has_more_chapters;
		}
	}
}

/// A page of novel entries.
#[derive(Default, Clone, Debug, PartialEq, Serialize)]
pub struct NovelPageResult {
	/// List of novel entries.
	pub entries: Vec<Novel>,
	/// Whether the next page is available or not.
	pub has_next_page: bool,
}

/// A chapter of a novel.
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

/// The an element of content of the novel chapter page.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ContentBlock {
	// /// Optional open a url in browser action.
	// Url(String, Option<ContentURLContext>),
	/// Block/Banner quoted text.
	BlockQuote(String),
	/// Markdown text content.
	Paragraph(String, Option<String>),
	/// HTML Table content.
	Table(Vec<Vec<String>>),
    /// Section break or divider.
    Divider,
}

impl ContentBlock {
	/// Create a new `PageContent` with a url.
	pub fn divider() -> Self {
		Self::Divider
	}
	/// Create a new `PageContent` with a markdown text string.
	pub fn block_quote<T: Into<String>>(text: T) -> Self {
		Self::BlockQuote(text.into())
	}

	/// Create a new `PageContent` with a markdown text string and optional style.
	pub fn paragraph<T: Into<String>>(text: T, font_size: Option<String>) -> Self {
		Self::Paragraph(text.into(), font_size)
	}

	/// Create a new `PageContent` with a table.
	pub fn table(data: Vec<Vec<String>>) -> Self {
		Self::Table(data)
	}
}

impl Default for ContentBlock {
	fn default() -> Self {
		ContentBlock::Paragraph(String::default(), None)
	}
}

/// The display type of a listing.
#[derive(Default, PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum ListingKind {
	#[default]
	Default,
	List,
}

/// A listing of novel.
#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Listing {
	/// Unique identifier for the listing.
	pub id: String,
	/// Title of the listing.
	pub name: String,
	/// Type of listing.
	pub kind: ListingKind,
}
