## Resource Files

Unique to Aidoku source projects is the `res` directory, which contains additional files that are
packaged with the source when compiled. There are three different JSON files that are used for
configuration, each of which has a corresponding JSON schema that can be used for type hints and
autocompletion in IDEs.

To configure this for [Visual Studio Code](https://code.visualstudio.com/), you can set the
`json.schemas` key in `settings.json`:

```json
"json.schemas": [
	{
		"fileMatch": ["*/res/source.json"],
		"url": "https://raw.githubusercontent.com/Aidoku/aidoku-rs/refs/heads/main/crates/cli/src/supporting/schema/source.schema.json"
	},
	{
		"fileMatch": ["*/res/filters.json"],
		"url": "https://raw.githubusercontent.com/Aidoku/aidoku-rs/refs/heads/main/crates/cli/src/supporting/schema/filters.schema.json"
	},
	{
		"fileMatch": ["*/res/settings.json"],
		"url": "https://raw.githubusercontent.com/Aidoku/aidoku-rs/refs/heads/main/crates/cli/src/supporting/schema/settings.schema.json"
	}
]
```

Similarly for [Zed](https://zed.dev/) (recommended for Rust development), you can configure the
`json-language-server` parameters with the same array of schemas:

```json
"lsp": {
	"json-language-server": {
		"settings": {
			"json": {
				"schemas": [
					...
				]
			}
		}
	}
}
```

Note that these schemas can also be checked against packaged sources by using `aidoku`:

```sh
aidoku verify package.aix
```

### source.json

The `source.json` file is the only required configuration file, which has three components: "info"
(required), "listings" (optional), and "config" (optional). Here is an example of how the file
should be structured:

```json
{
	"info": {
		"id": "en.example-source",
		"name": "Example Source",
		"version": 1,
		"url": "https://aidoku.app",
		"contentRating": 0,
		"languages": ["en"]
	},
	"listings": [
		{
			"id": "test",
			"name": "Test"
		}
	],
	"config": {
		"supportsTagSearch": true
	}
}
```

| Field                | Description                                                                                                                        |
|----------------------|------------------------------------------------------------------------------------------------------------------------------------|
| `info.id`            | A unique identifier for the source. Conventionally, this should be the language and source name, concatenated with a period.       |
| `info.name`          | The displayed name of the source.                                                                                                  |
| `info.altNames`      | An array of additional names that the source should be searchable by.                                                              |
| `info.version`       | The source's version number. It must be a positive integer and incremented with any notable changes.                               |
| `info.url`           | The source's main URL, which can be used for deep linking.                                                                         |
| `info.urls`          | An array of the source's URLs, If the source has multiple domains, this should be used instead of `info.url`.                      |
| `info.contentRating` | The NSFW level of the source. `0` for sources with no NSFW content at all, `1` for some NSFW, and `2` for majority NSFW sources.   |
| `info.languages`     | An array of ISO 639 language codes that the source supports.                                                                       |
| `info.minAppVersion` | Optional minimum Aidoku app version supported by the source.                                                                       |
| `info.maxAppVersion` | Optional maximum Aidoku app version supported by the source.                                                                       |
| `listings` | An array of listings that the source supports. Listings can have an `id`, `name`, and `kind` being a grid (0) or list (1).                   |
| `config.languageSelectType`         | Either "single" or "multi", defaulting to "single". If the app should allow selecting multiple enabled languages.   |
| `config.supportsArtistSearch`       | If the source should allow filtering by series artists without having provided an "artist" text filter.             |
| `config.supportsAuthorSearch`       | `config.supportsArtistSearch` but for authors.                                                                      |
| `config.supportsTagSearch`          | If the source should allow filtering by pressing on a tag, even if it doesn't exist in the source filters.          |
| `config.allowsBaseUrlSelect`        | If selecting a URL from `info.urls` should be enabled in the source settings.                                       |
| `config.breakingChangeVersion`      | The source version at which a breaking change that requires migration was made.                                     |
| `config.hidesFiltersWhileSearching` | If filters should be disabled with an active search query.                                                          |

### filters.json (optional)

The `filters.json` file allows you to configure static filters for Aidoku to display in the UI. In
addition to providing static filters, you can also programatically define "dynamic" filters using
the `DynamicFilters` trait in your source code, which will be appended to whatever static filters
you have defined. However, static filters are preferred whenever possible due to quicker loading and
better inference for various features.

Filters are defined as JSON objects with varying attributes depending on the "type":

| Filter Type    | Description                                                                                                                              |
|----------------|------------------------------------------------------------------------------------------------------------------------------------------|
| `text`         | An input field for text content (e.g. author or artist searching).                                                                       |
| `sort`         | A sort filter, optionally supporting both ascending and descending directions.                                                           |
| `check`        | A single toggle option.                                                                                                                  |
| `select`       | A list allowing selection of a single value.                                                                                             |
| `multi-select` | A list allowing selection of multiple values, optionally supporting exclusion as well as inclusion.                                      |
| `note`         | Non-editable text for information or explanation.                                                                                        |
| `range`        | An editable "from" and "to" range of numeric values.                                                                                     |

### settings.json (optional)

Similar to the `filters.json` file, the `settings.json` file allows you to define static settings
for your source, with dynamic ones able to be provided through the `DynamicSettings` trait.

Like filters, settings are JSON objects with varying attributes depending on the "type":

| Setting Type    | Description                                                                                                                              |
|-----------------|------------------------------------------------------------------------------------------------------------------------------------------|
| `group`         | Used to group other settings into sections with optional header and footer text.                                                         |
| `select`        | A list allowing selection of a single value.                                                                                             |
| `multi-select`  | A list allowing selection of multiple values.                                                                                            |
| `switch`        | A toggle with an on and off state.                                                                                                       |
| `stepper`       | A number stepper                                                                                                                         |
| `segment`       | A segmented control for selecting a single value from a few options.                                                                     |
| `text`          | An input field for text content.                                                                                                         |
| `button`        | A button that can trigger an action for the source to execute programmatically.                                                          |
| `link`          | A button that links to a web page.                                                                                                       |
| `login`         | A login setting, allowing for simple username/password entry or oauth flow handling.                                                     |
| `page`          | Used to group settings into a nested page.                                                                                               |
| `editable-list` | An editable list of items, allowing the user to add and remove arbitrary text entries.                                                   |

Settings values can be fetched and set programmatically using the
[defaults](https://aidoku.github.io/aidoku-rs/aidoku/imports/defaults/index.html) functions provided
by aidoku-rs.

### icon.png

Aidoku expects a PNG image file to display as the source icon. The image should be square, and
ideally 128x128 with no transparency.
