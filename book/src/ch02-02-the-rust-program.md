## The Rust Program

Aidoku sources are fundamentally [no_std](https://docs.rust-embedded.org/book/intro/no-std.html)
Rust programs that compile to WebAssembly. Thus, any Rust crates that support `no_std` (or `no_std`
with an allocator) can be utilized as dependencies.

aidoku-rs expects sources to be written by implementing traits that it provides to a struct, then
passing that struct to the `register_source!` macro.

### Traits

All sources must implement the one required
[Source](https://aidoku.github.io/aidoku-rs/aidoku/trait.Source.html) trait, and then can optionally
implement any other traits for more features. Check
[here](https://aidoku.github.io/aidoku-rs/aidoku/index.html#traits) for a complete list of traits
that aidoku-rs provides.

The `Source` trait has four required functions:
- [new](https://aidoku.github.io/aidoku-rs/aidoku/trait.Source.html#tymethod.new): performs any
  setup required for the source to function (e.g. setting global rate limit) and initializes the
  source struct.
- [get_search_manga_list](https://aidoku.github.io/aidoku-rs/aidoku/trait.Source.html#tymethod.get_search_manga_list):
  returns a single page of results, with a query and list of filters if input by the user. Filter
  values will correspond to any of the configured filters in `filters.json`, and page numbers will
  start at `1`. If the `MangaPageResult` you return has `has_next_page` set to `true`, then the
  function will be called again with the page number incremented if the user scroll down to load
  another page of results. `Manga` returned in the result entries only need to have the `id`,
  `title`, and `cover` attributes populated.
- [get_manga_update](https://aidoku.github.io/aidoku-rs/aidoku/trait.Source.html#tymethod.get_manga_update):
  if the `needs_details` parameter is `true`, then as many of the available `manga` struct
  attributes should be filled as possible to provide the UI with information to display.
  If `needs_chapters` is `true`, the `chapters` attribute should be set. This function may be called
  with either or both of the booleans set to `true`, but not neither.
- [get_page_list](https://aidoku.github.io/aidoku-rs/aidoku/trait.Source.html#tymethod.get_page_list):
  returns a list of pages for a given manga and chapter.

Most sources should implement the
[DeepLinkHandler](https://aidoku.github.io/aidoku-rs/aidoku/trait.DeepLinkHandler.html) trait, to
allow users to replace the "http" or "https" in a url with "aidoku" to open the app directly to
whatever series or chapter the web page was displaying. This may also be used in the future by the
app to open urls in the app directly.

After implementing a basic source that supports searching and filtering, your next step would be
considering if you want to implement listings with the
[ListingProvider](https://aidoku.github.io/aidoku-rs/aidoku/trait.ListingProvider.html) trait and
a home page with the [Home](https://aidoku.github.io/aidoku-rs/aidoku/trait.Home.html) trait.

### Registering

The `register_source!` macro should be called with the source struct name as the first parameter,
and then all the other traits the struct implements apart from the `Source` trait. For example:

```rust,noplayground
struct MySource;

impl Source for MySource { ... }

impl ImageRequestProvider for MySource { ... }

impl DeepLinkHandler for MySource { ... }

register_source!(MySource, ImageRequestProvider, DeepLinkHandler);
```

Under the hood, this macro generates the actual functions that are exposed for Aidoku to call
directly, and converts the raw data sent from the app to the source into Rust structs for the source
to manipulate, then converts the structs the source returns in its implemented functions back into
raw data for the app to read. If you implement a trait for your source, the app won't take it into
account unless it is properly registered.

### Error Handling

All aidoku-rs trait functions expect a `Result<..., AidokuError>` to be returned, and most library
functions return `Result`s with an `AidokuError`, or errors that can be converted into an
`AidokuError`. This means that the `?` operator can be used liberally, allowing the app to handle
any errors. In addition, there are two helper macros to assist with creating errors yourself:
`error!` and `bail!`. Both macros allow you to provide a message (that supports formatting like
`format!` and `println!`) that the app will display in the UI if encountered.

`error!` simply constructs an `AidokuError`, and is typically used for something like this:

```rust,noplayground
let img_element = html.select("img").ok_or_else(|| error!("img element not found"))?;
```

Whereas `bail!` will return immediately. It can be used like this:

```rust,noplayground
let Some(img_element) = html.select("img") else {
	bail!("img element not found");
};
```

### Network Requests

Nearly every Aidoku source will need to use network requests to fetch data from its target website,
be it HTML content or API responses, such as JSON data. As a simple example:

```rust,noplayground
let response: Response = Request::get("http://example.com")?
	.header("User-Agent", "Aidoku")
	.send()?;
assert_eq!(response.status_code(), 200);
```

The other commonly used functions that "send" the network request and return a result are
[data](https://aidoku.github.io/aidoku-rs/aidoku/imports/net/struct.Request.html#method.data) to get
the raw byte data,
[string](https://aidoku.github.io/aidoku-rs/aidoku/imports/net/struct.Request.html#method.string) to
get the text content, and
[html](https://aidoku.github.io/aidoku-rs/aidoku/imports/net/struct.Request.html#method.html) to
automatically parse as HTML. If the "json" feature is enabled for the aidoku-rs dependency, the
`json_owned` function can be used to automatically parse the response data using
[serde_json](https://docs.rs/serde_json/latest/serde_json/).

Note that all function calls that send the network request are blocking, and will pause your source
execution until the data is available to return. When performing multiple network requests that
don't depend on each other, you can consider parallelizing them with
[Request\:\:send_all](https://aidoku.github.io/aidoku-rs/aidoku/imports/net/struct.Request.html#method.send_all).

### HTML Parsing

Aidoku provides APIs for interacting with the [SwiftSoup](https://github.com/scinfu/SwiftSoup)
library, which allows you to extract data from HTML elements using CSS selectors. The main functions
you should be aware of are
[Element\:\:select](https://aidoku.github.io/aidoku-rs/aidoku/imports/html/struct.Element.html#method.select),
[Element\:\:select_first](https://aidoku.github.io/aidoku-rs/aidoku/imports/html/struct.Element.html#method.select_first),
[Element\:\:text](https://aidoku.github.io/aidoku-rs/aidoku/imports/html/struct.Element.html#method.text),
and [Element\:\:attr](https://aidoku.github.io/aidoku-rs/aidoku/imports/html/struct.Element.html#method.attr).

```rust,noplayground
let html = Request::get("http://aidoku.app")?.html()?;
let text = html
	.select_first("main.home .features > .feature h2")
	.and_then(|element| element.text());
let image = html
	.select_first(".image > img")
	.and_then(|element| element.attr("abs:src"));
```
