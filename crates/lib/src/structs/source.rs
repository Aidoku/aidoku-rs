use super::{
	Chapter, ContentBlock, Filter, FilterValue, HashMap, HomeLayout, Listing, Novel,
	NovelPageResult, Setting,
};
use crate::alloc::{String, Vec};
use serde::{ser::SerializeStruct, Deserialize, Serialize};

pub use crate::imports::error::{BunyError, Result};

/// The required functions an Buny source must implement.
pub trait Source {
	/// Called to initialize a source.
	///
	/// If a source requires any setup before other functions are called, it should happen here.
	fn new() -> Self;

	/// Returns the novel for a search query with filters.
	fn get_search_novel_list(
		&self,
		query: Option<String>,
		page: i32,
		filters: Vec<FilterValue>,
	) -> Result<NovelPageResult>;

	/// Updates a given novel with new details and chapters, as requested.
	fn get_novel_update(
		&self,
		novel: Novel,
		needs_details: bool,
		needs_chapters: bool,
        page: i32,
	) -> Result<Novel>;

	/// Returns the content array for a given novel chapter.
	fn get_chapter_content_list(&self, novel: Novel, chapter: Chapter)
		-> Result<Vec<ContentBlock>>;
}

/// A source that provides listings.
pub trait ListingProvider: Source {
	/// Returns the novel for the provided listing.
	fn get_novel_list(&self, listing: Listing, page: i32) -> Result<NovelPageResult>;
}

/// A source that provides a home layout.
pub trait Home: Source {
	fn get_home(&self) -> Result<HomeLayout>;
}

/// A source that provides dynamic listings.
pub trait DynamicListings: Source {
	fn get_dynamic_listings(&self) -> Result<Vec<Listing>>;
}

/// A source that provides dynamic filters.
pub trait DynamicFilters: Source {
	fn get_dynamic_filters(&self) -> Result<Vec<Filter>>;
}

/// A source that provides dynamic settings.
pub trait DynamicSettings: Source {
	fn get_dynamic_settings(&self) -> Result<Vec<Setting>>;
}

/// A source that provides multiple cover images.
pub trait AlternateCoverProvider: Source {
	fn get_alternate_covers(&self, novel: Novel) -> Result<Vec<String>>;
}

/// A source that provides a programmatic base url.
///
/// The use of this trait is discouraged in favor of providing the source url statically.
pub trait BaseUrlProvider: Source {
	fn get_base_url(&self) -> Result<String>;
}

/// A source that handles notification callbacks.
///
/// Notifications can be sent on source setting changes.
pub trait NotificationHandler: Source {
	fn handle_notification(&self, notification: String);
}

/// A source that handles deep links.
///
/// If a url that is contained in one of the source's provided base urls is opened
/// in Buny, it will be sent to the given source to handle.
pub trait DeepLinkHandler: Source {
	fn handle_deep_link(&self, url: String) -> Result<Option<DeepLinkResult>>;
}

/// A source that handles basic login with username and password.
///
/// This function should return true if the login was successful.
pub trait BasicLoginHandler: Source {
	fn handle_basic_login(&self, key: String, username: String, password: String) -> Result<bool>;
}

/// A source that handles custom webview login.
///
/// This function will be called whenever cookies are updated, and should return true if the login was successful.
pub trait WebLoginHandler: Source {
	fn handle_web_login(&self, key: String, cookies: HashMap<String, String>) -> Result<bool>;
}

/// A source that handles key migration.
///
/// If a source provides a "breakingChangeVersion" in its configuration, these functions will be
/// called with all of a user's local novel and chapter keys to migrate them after updating.
/// These functions should return the new key to replace the old one.
pub trait MigrationHandler: Source {
	fn handle_novel_migration(&self, key: String) -> Result<String>;
	fn handle_chapter_migration(&self, novel_key: String, chapter_key: String) -> Result<String>;
}

/// A result of a deep link handling.
#[derive(Debug, Clone, PartialEq)]
pub enum DeepLinkResult {
	Novel { key: String },
	Chapter { novel_key: String, key: String },
	Listing(Listing),
}

impl Serialize for DeepLinkResult {
	fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		let mut state = serializer.serialize_struct("DeepLinkResult", 3)?;
		match self {
			DeepLinkResult::Novel { key } => {
				state.serialize_field("novel_key", &Some(key))?;
				state.serialize_field("chapter_key", &Option::<String>::None)?;
				state.serialize_field("listing", &Option::<Listing>::None)?;
			}
			DeepLinkResult::Chapter { novel_key, key } => {
				state.serialize_field("novel_key", &Some(novel_key))?;
				state.serialize_field("chapter_key", &Some(key))?;
				state.serialize_field("listing", &Option::<Listing>::None)?;
			}
			DeepLinkResult::Listing(listing) => {
				state.serialize_field("novel_key", &Option::<String>::None)?;
				state.serialize_field("chapter_key", &Option::<String>::None)?;
				state.serialize_field("listing", &Some(listing))?;
			}
		}
		state.end()
	}
}

/// The details of a HTTP request.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImageRequest {
	pub url: Option<String>,
	pub headers: HashMap<String, String>,
}
