use serde::{ser::SerializeStruct, Deserialize, Serialize};

extern crate alloc;
use alloc::{borrow::Cow, string::String, vec::Vec};

/// A filter that can be used in a source search.
///
/// This struct shouldn't be constructed directly. Instead, use the individual
/// filter structs and call [into](Into::into).
#[derive(Debug, Clone, PartialEq)]
pub struct Filter {
	pub id: Cow<'static, str>,
	pub title: Option<Cow<'static, str>>,
	pub hide_from_header: Option<bool>,
	pub kind: FilterKind,
}
/// A default value for a sort filter.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SortFilterDefault {
	pub index: i32,
	pub ascending: bool,
}

/// The kind of filter.
#[derive(Debug, Clone, PartialEq)]
pub enum FilterKind {
	/// A text field.
	Text {
		placeholder: Option<Cow<'static, str>>,
	},
	/// A list of sort options.
	Sort {
		can_ascend: bool,
		options: Vec<Cow<'static, str>>,
		default: Option<SortFilterDefault>,
	},
	/// A checkbox.
	Check {
		name: Option<Cow<'static, str>>,
		can_exclude: bool,
		default: Option<bool>,
	},
	/// A list of values that allows a single selection.
	Select {
		is_genre: bool,
		uses_tag_style: bool,
		options: Vec<Cow<'static, str>>,
		ids: Option<Vec<Cow<'static, str>>>,
		default: Option<Cow<'static, str>>,
	},
	/// A list of values that allows multiple selections.
	MultiSelect {
		is_genre: bool,
		can_exclude: bool,
		uses_tag_style: bool,
		options: Vec<Cow<'static, str>>,
		ids: Option<Vec<Cow<'static, str>>>,
		default_included: Option<Vec<Cow<'static, str>>>,
		default_excluded: Option<Vec<Cow<'static, str>>>,
	},
	/// A block of text displayed in the filter menu.
	Note(Cow<'static, str>),
	/// A range filter.
	Range {
		min: Option<f32>,
		max: Option<f32>,
		decimal: bool,
	},
}

impl Filter {
	/// Creates a new note filter with the given text.
	pub fn note<A: Into<Cow<'static, str>>>(text: A) -> Self {
		Self {
			id: Cow::Borrowed("note"),
			title: None,
			hide_from_header: None,
			kind: FilterKind::Note(text.into()),
		}
	}
}

macro_rules! create_filter_struct {
	(
		$struct_name:ident,
		$filter_kind:ident,
		$doc_comment:expr,
		{ $($(#[$field_meta:meta])* $field_name:ident: $field_type:ty),* $(,)? },
		{ $($def_field_name:ident: $default_value:expr),* $(,)? }
	) => {
		#[doc = $doc_comment]
		///
		/// This filter can be converted into a generic [Filter] using the [Into] trait.
		#[derive(Debug, Clone, PartialEq)]
		pub struct $struct_name {
			/// The identifier for this filter.
			pub id: Cow<'static, str>,
			/// The display title for this filter.
			pub title: Option<Cow<'static, str>>,
			/// Whether this filter should be hidden from the filters list header.
			pub hide_from_header: Option<bool>,
			$(
				$(#[$field_meta])*
				pub $field_name: $field_type
			),*
		}

		impl From<$struct_name> for Filter {
			fn from(value: $struct_name) -> Filter {
				Filter {
					id: value.id,
					title: value.title,
					hide_from_header: value.hide_from_header,
					kind: FilterKind::$filter_kind {
						$($field_name: value.$field_name),*
					},
				}
			}
		}

		impl Default for $struct_name {
			fn default() -> Self {
				Self {
					id: Cow::Borrowed(stringify!($struct_name)),
					title: None,
					hide_from_header: None,
					$($def_field_name: $default_value),*
				}
			}
		}
	};
}

create_filter_struct!(
	TextFilter,
	Text,
	"A text field.",
	{
		/// Optional placeholder text to display when the field is empty.
		placeholder: Option<Cow<'static, str>>,
	},
	{
		placeholder: None,
	}
);

create_filter_struct!(
	SortFilter,
	Sort,
	"A list of sort options.",
	{
		/// Whether the sort can be ascending.
		can_ascend: bool,
		/// The list of available sort options.
		options: Vec<Cow<'static, str>>,
		/// The default sort option.
		default: Option<SortFilterDefault>,
	},
	{
		can_ascend: true,
		options: Vec::new(),
		default: None,
	}
);

create_filter_struct!(
	CheckFilter,
	Check,
	"A checkbox.",
	{
		/// Optional display name for the checkbox. If `None`, the title is used.
		name: Option<Cow<'static, str>>,
		/// Whether the checkbox can be excluded (tristate).
		can_exclude: bool,
		/// The default state of the checkbox.
		default: Option<bool>,
	},
	{
		name: None,
		can_exclude: false,
		default: None,
	}
);

create_filter_struct!(
	SelectFilter,
	Select,
	"A list of values that allows a single selection.",
	{
		/// Indicates if the filter is for genres.
		is_genre: bool,
		/// Whether to display the options as tags.
		uses_tag_style: bool,
		/// The list of options to display.
		options: Vec<Cow<'static, str>>,
		/// Optional IDs for each option. If not provided, the options are used.
		ids: Option<Vec<Cow<'static, str>>>,
		/// The default selected option.
		default: Option<Cow<'static, str>>,
	},
	{
		is_genre: false,
		uses_tag_style: false,
		options: Vec::new(),
		ids: None,
		default: None,
	}
);

create_filter_struct!(
	MultiSelectFilter,
	MultiSelect,
	"A list of values that allows multiple selections.",
	{
		/// Indicates if the filter is for genres.
		is_genre: bool,
		/// Whether options can be excluded as well as included.
		can_exclude: bool,
		/// Whether to display the options as tags.
		uses_tag_style: bool,
		/// The list of options to display.
		options: Vec<Cow<'static, str>>,
		/// Optional IDs for each option. If not provided, the options are used.
		ids: Option<Vec<Cow<'static, str>>>,
		default_included: Option<Vec<Cow<'static, str>>>,
		default_excluded: Option<Vec<Cow<'static, str>>>,
	},
	{
		is_genre: false,
		can_exclude: false,
		uses_tag_style: false,
		options: Vec::new(),
		ids: None,
		default_included: None,
		default_excluded: None,
	}
);

create_filter_struct!(
	RangeFilter,
	Range,
	"A range filter.",
	{
		/// The minimum value of the range.
		min: Option<f32>,
		/// The maximum value of the range.
		max: Option<f32>,
		/// Whether the range can be decimal.
		decimal: bool,
	},
	{
		min: None,
		max: None,
		decimal: false,
	}
);

impl FilterKind {
	fn raw_value(&self) -> &str {
		match self {
			FilterKind::Text { .. } => "text",
			FilterKind::Sort { .. } => "sort",
			FilterKind::Check { .. } => "check",
			FilterKind::Select { .. } => "select",
			FilterKind::MultiSelect { .. } => "multi-select",
			FilterKind::Note(_) => "note",
			FilterKind::Range { .. } => "range",
		}
	}
}

impl Serialize for Filter {
	fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		let mut state = serializer.serialize_struct("Filter", 0)?;
		state.serialize_field("id", &Some(self.id.clone()))?;
		state.serialize_field("title", &self.title)?;
		state.serialize_field("hide_from_header", &self.hide_from_header)?;
		state.serialize_field("type", &self.kind.raw_value())?;
		match &self.kind {
			FilterKind::Text { placeholder } => {
				state.serialize_field("placeholder", &placeholder)?
			}
			FilterKind::Sort {
				can_ascend,
				options,
				default,
			} => {
				state.serialize_field("can_ascend", &Some(can_ascend))?;
				state.serialize_field("options", &options)?;
				state.serialize_field("default", &default)?
			}
			FilterKind::Check {
				name,
				can_exclude,
				default,
			} => {
				state.serialize_field("name", &name)?;
				state.serialize_field("can_exclude", &Some(can_exclude))?;
				state.serialize_field("default", &default)?
			}
			FilterKind::Select {
				is_genre,
				uses_tag_style,
				options,
				ids,
				default,
			} => {
				state.serialize_field("is_genre", &Some(is_genre))?;
				state.serialize_field("uses_tag_style", &Some(uses_tag_style))?;
				state.serialize_field("options", &options)?;
				state.serialize_field("ids", &ids)?;
				state.serialize_field("default", &default)?
			}
			FilterKind::MultiSelect {
				is_genre,
				can_exclude,
				uses_tag_style,
				options,
				ids,
				default_included,
				default_excluded,
			} => {
				state.serialize_field("is_genre", &Some(is_genre))?;
				state.serialize_field("can_exclude", &Some(can_exclude))?;
				state.serialize_field("uses_tag_style", &Some(uses_tag_style))?;
				state.serialize_field("options", &options)?;
				state.serialize_field("ids", &ids)?;
				state.serialize_field("default_included", &default_included)?;
				state.serialize_field("default_excluded", &default_excluded)?;
			}
			FilterKind::Note(text) => state.serialize_field("text", &text)?,
			FilterKind::Range { min, max, decimal } => {
				state.serialize_field("min", &min)?;
				state.serialize_field("max", &max)?;
				state.serialize_field("decimal", &Some(decimal))?;
			}
		};
		state.end()
	}
}

/// A configured filter value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FilterValue {
	/// A string from a text field.
	Text {
		/// The id of the filter.
		id: String,
		/// The value of the text field.
		value: String,
	},
	/// A value from a sort filter.
	Sort {
		/// The id of the filter.
		id: String,
		/// The index of the sort option.
		index: i32,
		/// Whether the sort is ascending.
		ascending: bool,
	},
	/// A value from a check filter.
	Check {
		/// The id of the filter.
		id: String,
		/// The value of the check filter.
		value: i32,
	},
	/// A value from a select filter.
	Select {
		/// The id of the filter.
		id: String,
		/// The value of the select filter.
		value: String,
	},
	/// A list of values from a multi-select filter.
	MultiSelect {
		/// The id of the filter.
		id: String,
		/// The list of included values.
		included: Vec<String>,
		/// The list of excluded values.
		excluded: Vec<String>,
	},
	/// A range of values from a range filter.
	Range {
		/// The id of the filter.
		id: String,
		/// The starting value of the range.
		from: Option<f32>,
		/// The ending value of the range.
		to: Option<f32>,
	},
}
