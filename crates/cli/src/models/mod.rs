use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, Serialize, Deserialize)]
pub struct SourceJson {
	pub info: SourceInfo,
}

#[derive(Debug, clap::ValueEnum, Serialize_repr, Deserialize_repr, Clone, Copy)]
#[repr(u8)]
pub enum SourceContentRating {
	Safe,
	ContainsNsfw,
	PrimarilyNsfw,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceInfo {
	pub id: String,
	pub name: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub alt_names: Option<Vec<String>>,
	pub version: i32,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub url: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub urls: Option<Vec<String>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub content_rating: Option<SourceContentRating>,
	pub languages: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SourceList {
	pub name: String,
	// #[serde(rename = "feedbackURL")]
	// #[serde(skip_serializing_if = "Option::is_none")]
	// pub feedback_url: Option<String>,
	pub sources: Vec<SourcesItem>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SourcesItem {
	pub id: String,
	pub name: String,
	pub version: i32,
	#[serde(rename = "iconURL")]
	pub icon_url: String,
	#[serde(rename = "downloadURL")]
	pub download_url: String,
	pub languages: Vec<String>,
	pub content_rating: SourceContentRating,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub alt_names: Option<Vec<String>>,
	#[serde(rename = "baseURL")]
	pub base_url: Option<String>,
}
