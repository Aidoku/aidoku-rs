use serde::{ser::SerializeStruct, Serialize};

extern crate alloc;
use alloc::{borrow::Cow, string::String, vec::Vec};

/// A setting that is shown in the source settings page.
///
/// This struct shouldn't be constructed directly. Instead, use the individual
/// settings structs and call [into](Into::into).
#[derive(Debug, Clone, PartialEq)]
pub struct Setting {
	pub key: Cow<'static, str>,
	pub title: Cow<'static, str>,
	pub notification: Option<Cow<'static, str>>,
	pub requires: Option<Cow<'static, str>>,
	pub requires_false: Option<Cow<'static, str>>,
	pub refreshes: Option<Vec<Cow<'static, str>>>,
	pub value: SettingValue,
}

impl Serialize for Setting {
	fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		let mut state = serializer.serialize_struct("Setting", 8)?;
		state.serialize_field("type", &self.value.raw_value())?;
		state.serialize_field("key", &self.key)?;
		state.serialize_field("title", &self.title)?;
		state.serialize_field("notification", &self.notification)?;
		state.serialize_field("requires", &self.requires)?;
		state.serialize_field("requires_false", &self.requires_false)?;
		state.serialize_field("refreshes", &self.refreshes)?;
		state.serialize_field("value", &self.value)?;
		state.end()
	}
}

/// A login method.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoginMethod {
	/// Basic authentication with username and password.
	Basic,
	/// OAuth authentication.
	OAuth,
	/// Authentication via a web view.
	Web,
}

impl Serialize for LoginMethod {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		match self {
			Self::Basic => serializer.serialize_str("basic"),
			Self::OAuth => serializer.serialize_str("oauth"),
			Self::Web => serializer.serialize_str("web"),
		}
	}
}

/// The kind of setting.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum SettingValue {
	/// A group of settings.
	Group {
		/// Optional footer text for the group.
		footer: Option<Cow<'static, str>>,
		/// The settings contained in this group.
		items: Vec<Setting>,
	},
	/// A page that allows selection of a single value.
	Select {
		/// The values of the options.
		values: Vec<Cow<'static, str>>,
		/// Optional display titles for the options. If not provided, the values will be used as titles.
		titles: Option<Vec<Cow<'static, str>>>,
		/// Whether to require authentication to open the page for this setting.
		auth_to_open: Option<bool>,
		/// The default selected value.
		default: Option<String>,
	},
	/// A page that allows selection of multiple values.
	MultiSelect {
		/// The values of the options.
		values: Vec<Cow<'static, str>>,
		/// Optional display titles for the options. If not provided, the values will be used as titles.
		titles: Option<Vec<Cow<'static, str>>>,
		/// Whether to require authentication to open the page for this setting.
		auth_to_open: Option<bool>,
		/// The default selected value(s).
		default: Option<Vec<String>>,
	},
	/// A toggle switch.
	Toggle {
		/// Optional subtitle text.
		subtitle: Option<Cow<'static, str>>,
		/// Whether to require authentication to turn the toggle off.
		auth_to_disable: Option<bool>,
		/// The default state of the toggle.
		default: bool,
	},
	/// A numeric stepper control.
	Stepper {
		/// The minimum allowed value.
		minimum_value: f64,
		/// The maximum allowed value.
		maximum_value: f64,
		/// Optional step increment value.
		step_value: Option<f64>,
		/// The default value.
		default: Option<f64>,
	},
	/// A segmented control.
	Segment {
		/// The options shown on the segments.
		options: Vec<Cow<'static, str>>,
		/// The default selected segment index.
		default: Option<i32>,
	},
	/// A text input field.
	Text {
		/// Optional placeholder text when the field is empty.
		placeholder: Option<Cow<'static, str>>,
		/// The autocapitalization type.
		autocapitalization_type: Option<i32>,
		/// Whether autocorrection should be disabled.
		autocorrection_disabled: Option<bool>,
		/// The keyboard type.
		keyboard_type: Option<i32>,
		/// The return key type.
		return_key_type: Option<i32>,
		/// Whether the text field is for secure entry (password).
		secure: Option<bool>,
		/// The default text value.
		default: Option<Cow<'static, str>>,
	},
	/// A clickable button.
	Button,
	/// A link to a URL.
	Link {
		/// The URL to open on press.
		url: Cow<'static, str>,
		/// Whether the link should open in an external browser.
		external: Option<bool>,
	},
	/// A login control.
	Login {
		/// The authentication method to use.
		method: LoginMethod,
		/// The authentication URL.
		url: Option<Cow<'static, str>>,
		/// An optional defaults key to fetch the URL from.
		url_key: Option<Cow<'static, str>>,
		/// The title for the logout button. If not provided, the title will be "Log Out".
		logout_title: Option<Cow<'static, str>>,
		/// Whether to use PKCE for the OAuth flow.
		pkce: bool,
		/// The token URL for OAuth.
		token_url: Option<Cow<'static, str>>,
		/// The callback scheme for OAuth.
		callback_scheme: Option<Cow<'static, str>>,
		/// Whether to prompt for an email instead of username for basic authentication.
		use_email: bool,
		/// An array of localStorage keys to extract after login.
		local_storage_keys: Option<Vec<String>>,
	},
	/// A page of settings.
	Page {
		/// The settings contained in this page.
		items: Vec<Setting>,
		/// Whether to display the title inline.
		inline_title: Option<bool>,
		/// Whether to require authentication to open the page.
		auth_to_open: Option<bool>,
		/// An icon to be displayed along with the page title.
		icon: Option<PageIcon>,
		/// An optional string to display under the title in a header view (an icon must also be provided).
		info: Option<String>,
	},
	/// A list that can be edited by the user.
	EditableList {
		/// Optional maximum number of lines.
		line_limit: Option<i32>,
		/// Whether to display the list inline.
		inline: bool,
		/// Optional placeholder text for new items.
		placeholder: Option<Cow<'static, str>>,
		/// The default list items.
		default: Option<Vec<Cow<'static, str>>>,
	},
}

impl SettingValue {
	fn raw_value(&self) -> &str {
		match self {
			Self::Group { .. } => "group",
			Self::Select { .. } => "select",
			Self::MultiSelect { .. } => "multi-select",
			Self::Toggle { .. } => "switch",
			Self::Stepper { .. } => "stepper",
			Self::Segment { .. } => "segment",
			Self::Text { .. } => "text",
			Self::Button => "button",
			Self::Link { .. } => "link",
			Self::Login { .. } => "login",
			Self::Page { .. } => "page",
			Self::EditableList { .. } => "editable-list",
		}
	}
}

#[derive(Debug, Clone, PartialEq)]
pub enum PageIcon {
	System {
		name: String,
		color: String,
		inset: Option<i32>,
	},
	Url(String),
}

impl Serialize for PageIcon {
	fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		let mut state = serializer.serialize_struct(
			"Setting",
			match self {
				Self::System { .. } => 2,
				Self::Url(_) => 1,
			},
		)?;
		match self {
			Self::System { name, color, inset } => {
				state.serialize_field("type", "system")?;
				state.serialize_field("name", name)?;
				state.serialize_field("color", color)?;
				state.serialize_field("inset", inset)?;
			}
			Self::Url(url) => {
				state.serialize_field("type", "url")?;
				state.serialize_field("url", url)?;
			}
		}
		state.end()
	}
}

macro_rules! create_setting_struct {
	(
		$struct_name:ident,
		$setting_kind:ident,
		$doc_comment:expr,
		{ $($(#[$field_meta:meta])* $field_name:ident: $field_type:ty),* $(,)? },
		{ $($def_field_name:ident: $default_value:expr),* $(,)? }
	) => {
		#[doc = $doc_comment]
		///
		/// This setting can be converted into a generic [Setting] using the [Into] trait.
		#[derive(Debug, Clone, PartialEq)]
		pub struct $struct_name {
			/// The unique key that identifies this setting.
			pub key: Cow<'static, str>,
			/// The display title for this setting.
			pub title: Cow<'static, str>,
			/// Optional notification to send to the source when this setting is changed.
			pub notification: Option<Cow<'static, str>>,
			/// Optional key of another setting that must be enabled for this setting to be enabled.
			pub requires: Option<Cow<'static, str>>,
			/// Optional key of another setting that must be disabled for this setting to be enabled.
			pub requires_false: Option<Cow<'static, str>>,
			/// Optional list of items that should be refreshed when this setting changes.
			///
			/// The valid options are:
			/// - `content`
			/// - `listings`
			/// - `settings`
			pub refreshes: Option<Vec<Cow<'static, str>>>,
			$(
				$(#[$field_meta])*
				pub $field_name: $field_type
			),*
		}

		impl From<$struct_name> for Setting {
			fn from(source: $struct_name) -> Self {
				Setting {
					key: source.key,
					title: source.title,
					notification: source.notification,
					requires: source.requires,
					requires_false: source.requires_false,
					refreshes: source.refreshes,
					value: SettingValue::$setting_kind {
						$($field_name: source.$field_name),*
					},
				}
			}
		}

		impl Default for $struct_name {
			fn default() -> Self {
				Self {
					key: Cow::Borrowed(stringify!($struct_name)),
					title: Cow::Borrowed(stringify!($struct_name)),
					notification: None,
					requires: None,
					requires_false: None,
					refreshes: None,
					$($def_field_name: $default_value),*
				}
			}
		}
	};
}

create_setting_struct!(
	GroupSetting,
	Group,
	"A group of settings.",
	{
		/// Optional footer text for the group.
		footer: Option<Cow<'static, str>>,
		/// The settings contained in this group.
		items: Vec<Setting>,
	},
	{
		footer: None,
		items: Vec::new(),
	}
);

create_setting_struct!(
	SelectSetting,
	Select,
	"A page that allows selection of a single value.",
	{
		/// The values of the options.
		values: Vec<Cow<'static, str>>,
		/// Optional display titles for the options. If not provided, the values will be used as titles.
		titles: Option<Vec<Cow<'static, str>>>,
		/// Whether to require authentication to open the page for this setting.
		auth_to_open: Option<bool>,
		/// The default selected value. If not provided, the first value will be selected.
		default: Option<String>,
	},
	{
		values: Vec::new(),
		titles: None,
		auth_to_open: None,
		default: None,
	}
);

create_setting_struct!(
	MultiSelectSetting,
	MultiSelect,
	"A page that allows selection of multiple values.",
	{
		/// The values of the options.
		values: Vec<Cow<'static, str>>,
		/// Optional display titles for the options. If not provided, the values will be used as titles.
		titles: Option<Vec<Cow<'static, str>>>,
		/// Whether to require authentication to open the page for this setting.
		auth_to_open: Option<bool>,
		/// The default selected value(s).
		default: Option<Vec<String>>,
	},
	{
		values: Vec::new(),
		titles: None,
		auth_to_open: None,
		default: None,
	}
);

create_setting_struct!(
	ToggleSetting,
	Toggle,
	"A toggle switch.",
	{
		/// Optional subtitle text.
		subtitle: Option<Cow<'static, str>>,
		/// Whether to require authentication to turn the toggle off.
		auth_to_disable: Option<bool>,
		/// The default state of the toggle.
		default: bool,
	},
	{
		subtitle: None,
		auth_to_disable: None,
		default: false,
	}
);

create_setting_struct!(
	StepperSetting,
	Stepper,
	"A numeric stepper control.",
	{
		/// The minimum allowed value.
		minimum_value: f64,
		/// The maximum allowed value.
		maximum_value: f64,
		/// Optional step increment value.
		step_value: Option<f64>,
		/// The default value.
		default: Option<f64>,
	},
	{
		minimum_value: 1.0,
		maximum_value: 10.0,
		step_value: None,
		default: None,
	}
);

create_setting_struct!(
	SegmentSetting,
	Segment,
	"A segmented control.",
	{
		/// The options show on the segments.
		options: Vec<Cow<'static, str>>,
		/// The default selected segment index.
		default: Option<i32>,
	},
	{
		options: Vec::new(),
		default: None,
	}
);

create_setting_struct!(
	TextSetting,
	Text,
	"A text input field.",
	{
		/// Optional placeholder text when the field is empty.
		placeholder: Option<Cow<'static, str>>,
		/// The autocapitalization type.
		autocapitalization_type: Option<i32>,
		/// The keyboard type.
		keyboard_type: Option<i32>,
		/// The return key type.
		return_key_type: Option<i32>,
		/// Whether autocorrection should be disabled.
		autocorrection_disabled: Option<bool>,
		/// Whether the text field is for secure entry (password).
		secure: Option<bool>,
		/// The default text value.
		default: Option<Cow<'static, str>>,
	},
	{
		placeholder: None,
		autocapitalization_type: None,
		keyboard_type: None,
		return_key_type: None,
		autocorrection_disabled: None,
		secure: None,
		default: None,
	}
);

create_setting_struct!(
	LinkSetting,
	Link,
	"A link to a URL.",
	{
		/// The URL to open on press.
		url: Cow<'static, str>,
		/// Whether the link should open in an external browser.
		external: Option<bool>,
	},
	{
		url: "".into(),
		external: None,
	}
);

create_setting_struct!(
	LoginSetting,
	Login,
	"A login control.",
	{
		/// The authentication method to use.
		method: LoginMethod,
		/// The authentication URL.
		url: Option<Cow<'static, str>>,
		/// An optional defaults key to fetch the URL from.
		url_key: Option<Cow<'static, str>>,
		/// The title for the logout button. If not provided, the title will be "Log Out".
		logout_title: Option<Cow<'static, str>>,
		/// Whether to use PKCE for the OAuth flow.
		pkce: bool,
		/// The token URL for OAuth.
		token_url: Option<Cow<'static, str>>,
		/// The callback scheme for OAuth.
		callback_scheme: Option<Cow<'static, str>>,
		/// Whether to prompt for an email instead of username for basic authentication.
		use_email: bool,
		/// An array of localStorage keys to extract after login.
		local_storage_keys: Option<Vec<String>>,
	},
	{
		method: LoginMethod::OAuth,
		url: None,
		url_key: None,
		logout_title: None,
		pkce: false,
		token_url: None,
		callback_scheme: None,
		use_email: false,
		local_storage_keys: None,
	}
);

create_setting_struct!(
	PageSetting,
	Page,
	"A page of settings.",
	{
		/// The settings contained in this page.
		items: Vec<Setting>,
		/// Whether to display the title inline.
		inline_title: Option<bool>,
		/// Whether to require authentication to open the page.
		auth_to_open: Option<bool>,
		/// An icon to be displayed along with the page title.
		icon: Option<PageIcon>,
		/// An optional string to display under the title in a header view (an icon must also be provided).
		info: Option<String>,
	},
	{
		items: Vec::new(),
		inline_title: None,
		auth_to_open: None,
		icon: None,
		info: None,
	}
);

create_setting_struct!(
	EditableListSetting,
	EditableList,
	"A list that can be edited by the user.",
	{
		/// Optional maximum number of lines.
		line_limit: Option<i32>,
		/// Whether to display the list inline instead of in a separate page.
		inline: bool,
		/// Optional placeholder text for new items.
		placeholder: Option<Cow<'static, str>>,
		/// The default list items.
		default: Option<Vec<Cow<'static, str>>>,
	},
	{
		line_limit: None,
		inline: false,
		placeholder: None,
		default: None,
	}
);

/// A button that notifies the source when pressed.
///
/// This setting can be converted into a generic [Setting] using the [Into] trait.
#[derive(Debug, Clone, PartialEq)]
pub struct ButtonSetting {
	/// The unique key that identifies this setting.
	pub key: Cow<'static, str>,
	/// The display title for this setting.
	pub title: Cow<'static, str>,
	/// Optional notification text to display when this setting is changed.
	pub notification: Option<Cow<'static, str>>,
	/// Optional key of another setting that must be enabled for this setting to be enabled.
	pub requires: Option<Cow<'static, str>>,
	/// Optional key of another setting that must be disabled for this setting to be enabled.
	pub requires_false: Option<Cow<'static, str>>,
	/// Optional list of setting keys that should be refreshed when this setting changes.
	pub refreshes: Option<Vec<Cow<'static, str>>>,
}

impl From<ButtonSetting> for Setting {
	fn from(button: ButtonSetting) -> Self {
		Setting {
			key: button.key,
			title: button.title,
			notification: button.notification,
			requires: button.requires,
			requires_false: button.requires_false,
			refreshes: button.refreshes,
			value: SettingValue::Button,
		}
	}
}

impl Default for ButtonSetting {
	fn default() -> Self {
		Self {
			key: Cow::Borrowed(stringify!($struct_name)),
			title: Cow::Borrowed(stringify!($struct_name)),
			notification: None,
			requires: None,
			requires_false: None,
			refreshes: None,
		}
	}
}
