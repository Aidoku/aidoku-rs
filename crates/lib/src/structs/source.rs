use super::{
	Chapter, Filter, FilterValue, HashMap, HomeLayout, Listing, Manga, MangaPageResult, Page,
	PageContext, Setting,
};
use crate::alloc::{String, Vec};
use crate::imports::{canvas::ImageRef, net::Request};
use serde::{ser::SerializeStruct, Deserialize, Serialize};

pub use crate::imports::error::{AidokuError, Result};

/// The required functions an Aidoku source must implement.
pub trait Source {
	/// Called to initialize a source.
	///
	/// If a source requires any setup before other functions are called, it should happen here.
	fn new() -> Self;

	/// Returns the manga for the provided listing.
	fn get_manga_list(&self, listing: Listing, page: i32) -> Result<MangaPageResult>;

	/// Returns the manga for a search query with filters.
	fn get_search_manga_list(
		&self,
		query: Option<String>,
		page: i32,
		filters: Vec<FilterValue>,
	) -> Result<MangaPageResult>;

	/// Updates a given manga with new details and chapters, as requested.
	fn get_manga_update(
		&self,
		manga: Manga,
		needs_details: bool,
		needs_chapters: bool,
	) -> Result<Manga>;

	/// Returns the pages for a given manga chapter.
	fn get_page_list(&self, manga: Manga, chapter: Chapter) -> Result<Vec<Page>>;
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

/// A source that processes page image data after being fetched.
pub trait PageImageProcessor: Source {
	fn process_page_image(
		&self,
		response: Response,
		context: Option<PageContext>,
	) -> Result<ImageRef>;
}

/// A source that provides requests for images.
///
/// By default, Aidoku will request covers, thumbnails, and pages without headers.
/// This trait can be used to override the requests for source images.
pub trait ImageRequestProvider: Source {
	fn get_image_request(&self, url: String, context: Option<PageContext>) -> Result<Request>;
}

/// A source that provides dynamic descriptions for pages.
pub trait PageDescriptionProvider: Source {
	fn get_page_description(&self, page: Page) -> Result<String>;
}

/// A source that provides multiple cover images.
pub trait AlternateCoverProvider: Source {
	fn get_alternate_covers(&self, manga: Manga) -> Result<Vec<String>>;
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
/// in Aidoku, it will be sent to the given source to handle.
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

/// A source that handles id migration.
///
/// If a source provides a "breakingChangeVersion" in its configuration, this function will be
/// called with all of a user's manga and chapter ids to migrate them after updating.
/// This function should return the new id to replace the old one.
pub trait MigrationHandler: Source {
	fn handle_id_migration(&self, id: String, id_kind: IdKind) -> Result<String>;
}

/// The kind of id string.
#[derive(Debug, Deserialize)]
pub enum IdKind {
	Manga = 0,
	Chapter = 1,
}

impl IdKind {
	pub fn from(value: i32) -> Option<Self> {
		match value {
			0 => Some(Self::Manga),
			1 => Some(Self::Chapter),
			_ => None,
		}
	}
}

/// A result of a deep link handling.
#[derive(Debug)]
pub enum DeepLinkResult {
	Manga { key: String },
	Chapter { manga_key: String, key: String },
	Listing(Listing),
}

impl Serialize for DeepLinkResult {
	fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		let mut state = serializer.serialize_struct("DeepLinkResult", 3)?;
		match self {
			DeepLinkResult::Manga { key } => {
				state.serialize_field("manga_key", &Some(key))?;
				state.serialize_field("chapter_key", &Option::<String>::None)?;
				state.serialize_field("listing", &Option::<Listing>::None)?;
			}
			DeepLinkResult::Chapter { manga_key, key } => {
				state.serialize_field("manga_key", &Some(manga_key))?;
				state.serialize_field("chapter_key", &Some(key))?;
				state.serialize_field("listing", &Option::<Listing>::None)?;
			}
			DeepLinkResult::Listing(listing) => {
				state.serialize_field("manga_key", &Option::<String>::None)?;
				state.serialize_field("chapter_key", &Option::<String>::None)?;
				state.serialize_field("listing", &Some(listing))?;
			}
		}
		state.end()
	}
}

/// The details of a HTTP request.
#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseRequest {
	pub url: Option<String>,
	pub headers: HashMap<String, String>,
}

/// A response from a network image request.
#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
	/// The HTTP status code.
	pub code: u16,
	/// The HTTP response headers.
	pub headers: HashMap<String, String>,
	/// The HTTP request details.
	pub request: ResponseRequest,
	/// A reference to image data.
	pub image: ImageRef,
}
