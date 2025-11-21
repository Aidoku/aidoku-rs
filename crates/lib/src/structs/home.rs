use super::{Chapter, FilterValue, Listing, Novel};
use serde::{Deserialize, Serialize};

extern crate alloc;
use alloc::{string::String, vec::Vec};

/// A partial result for the home page.
///
/// This should only be used with [send_partial_result](crate::imports::std::send_partial_result)
/// in the [get_home](super::Home::get_home) function.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum HomePartialResult {
	Layout(HomeLayout),
	Component(HomeComponent),
}

/// A home layout for a source.
#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HomeLayout {
	/// The components of the layout.
	pub components: Vec<HomeComponent>,
}

/// A component for a home layout.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HomeComponent {
	/// The title of the component.
	pub title: Option<String>,
	/// The subtitle of the component.
	pub subtitle: Option<String>,
	/// The component value.
	pub value: HomeComponentValue,
}

impl Default for HomeComponent {
	fn default() -> Self {
		Self {
			title: None,
			subtitle: None,
			value: HomeComponentValue::empty_scroller(),
		}
	}
}

/// The value of a component for a home layout.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum HomeComponentValue {
	ImageScroller {
		links: Vec<Link>,
		auto_scroll_interval: Option<f32>,
		width: Option<i32>,
		height: Option<i32>,
	},
	Details {
		entries: Vec<Novel>,
		auto_scroll_interval: Option<f32>,
		listing: Option<Listing>,
	},
	Scroller {
		entries: Vec<Novel>,
		auto_scroll_interval: Option<f32>,
		listing: Option<Listing>,
		size: i32,
	},
	Stack {
		entries: Vec<Novel>,
		auto_scroll_interval: Option<f32>,
		listing: Option<Listing>,
	},
	Vertical {
		entries: Vec<Novel>,
		auto_scroll_interval: Option<f32>,
		listing: Option<Listing>,
	},
	/// A collection of links to filtered listings.
	Filters(Vec<FilterItem>),
	/// A list of links.
	///
	/// Only the link title and values are used.
	Links(Vec<Link>),
}

impl HomeComponentValue {
	/// Creates an empty image scroller component.
	pub fn empty_image_scroller() -> Self {
		Self::ImageScroller {
			links: Vec::new(),
			auto_scroll_interval: None,
			width: None,
			height: None,
		}
	}

	pub fn empty_details() -> Self {
		Self::Details {
			entries: Vec::new(),
			auto_scroll_interval: None,
			listing: None,
		}
	}

	/// Creates an empty scroller component.
	pub fn empty_scroller() -> Self {
		Self::Scroller {
			entries: Vec::new(),
			auto_scroll_interval: None,
			listing: None,
			size: 0,
		}
	}

	/// Creates an empty scroller component.
	pub fn empty_scroller_small() -> Self {
		Self::Scroller {
			entries: Vec::new(),
			auto_scroll_interval: None,
			listing: None,
			size: 0,
		}
	}

	/// Creates an empty scroller component.
	pub fn empty_scroller_medium() -> Self {
		Self::Scroller {
			entries: Vec::new(),
			auto_scroll_interval: None,
			listing: None,
			size: 1,
		}
	}

	/// Creates an empty scroller component.
	pub fn empty_scroller_large() -> Self {
		Self::Scroller {
			entries: Vec::new(),
			auto_scroll_interval: None,
			listing: None,
			size: 2,
		}
	}

	/// Creates an empty manga list component.
	pub fn empty_stack() -> Self {
		Self::Stack {
			entries: Vec::new(),
			auto_scroll_interval: None,
			listing: None,
		}
	}

	/// Creates an empty manga list component.
	pub fn empty_vertical() -> Self {
		Self::Vertical {
			entries: Vec::new(),
			auto_scroll_interval: None,
			listing: None,
		}
	}

	/// Creates an empty filters component.
	pub fn empty_filters() -> Self {
		Self::Filters(Vec::new())
	}

	/// Creates an empty links component.
	pub fn empty_links() -> Self {
		Self::Links(Vec::new())
	}
}

/// A paired manga and chapter.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NovelWithChapter {
	pub novel: Novel,
	pub chapter: Chapter,
}

/// A link to a listing that uses the provided filters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FilterItem {
	pub title: String,
	pub values: Option<Vec<FilterValue>>,
}

impl From<String> for FilterItem {
	fn from(title: String) -> Self {
		Self {
			title,
			values: None,
		}
	}
}

impl From<&str> for FilterItem {
	fn from(title: &str) -> Self {
		Self {
			title: String::from(title),
			values: None,
		}
	}
}

/// A link used in home components.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Link {
	pub title: String,
	pub subtitle: Option<String>,
	pub image_url: Option<String>,
	pub value: Option<LinkValue>,
}

/// A link value that can be opened by the Aidoku app.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LinkValue {
	Url(String),
	Listing(Listing),
	Novel(Novel),
}

impl Default for LinkValue {
	fn default() -> Self {
		Self::Url(String::new())
	}
}

impl From<Novel> for Link {
	fn from(value: Novel) -> Self {
		Link {
			title: value.title.clone(),
			subtitle: value
				.authors
				.as_ref()
				.map(|a| a.join(", "))
				.or(value.description.clone()),
			image_url: value.cover.clone(),
			value: Some(LinkValue::Novel(value)),
		}
	}
}
