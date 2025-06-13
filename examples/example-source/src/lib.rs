#![no_std]
use aidoku::{
	alloc::{vec, String, Vec},
	imports::{defaults::defaults_get, net::Request},
	prelude::*,
	AlternateCoverProvider, Chapter, CheckFilter, ContentRating, DeepLinkHandler, DeepLinkResult,
	DynamicFilters, DynamicListings, DynamicSettings, Filter, FilterValue, Home, HomeComponent,
	HomeLayout, Listing, Manga, MangaPageResult, MangaStatus, MangaWithChapter, MultiSelectFilter,
	NotificationHandler, Page, PageContent, PageDescriptionProvider, Result, SelectFilter, Setting,
	SortFilter, Source, TextFilter, ToggleSetting,
};

const PAGE_SIZE: i32 = 20;

// to create a source, you need a struct that implements the Source trait
// the struct can contain properties that are initialized with the new() method
struct ExampleSource;

impl Source for ExampleSource {
	// this method is called once when the source is initialized
	// perform any necessary setup here
	fn new() -> Self {
		Self
	}

	// this method will be called when a listing or a home section with an associated listing is opened
	fn get_manga_list(&self, listing: Listing, _page: i32) -> Result<MangaPageResult> {
		if listing.id == "test" {
			bail!("Not supported");
		}
		Ok(MangaPageResult {
			entries: vec![Manga {
				key: String::from("1"),
				title: String::from("Manga 1"),
				cover: Some(String::from("https://aidoku.app/images/icon.png")),
				..Default::default()
			}],
			has_next_page: false,
		})
	}

	// this method will be called first without a query when the search page is opened,
	// then when a search query is entered or filters are changed
	fn get_search_manga_list(
		&self,
		query: Option<String>,
		page: i32,
		_filters: Vec<FilterValue>,
	) -> Result<MangaPageResult> {
		let mut entries: Vec<Manga> = Vec::new();
		let start = (page - 1) * PAGE_SIZE + 1;
		for i in start..start + PAGE_SIZE {
			let title = format!("Manga {i}");
			if let Some(query) = query.as_ref() {
				if !title.contains(query) {
					continue;
				}
			}
			entries.push(Manga {
				key: format!("{i}"),
				title,
				cover: Some(String::from("https://aidoku.app/images/icon.png")),
				authors: Some(vec![String::from("Author")]),
				..Default::default()
			})
		}
		Ok(MangaPageResult {
			entries,
			has_next_page: start < 40,
		})
	}

	// this method will be called when a manga page is opened
	fn get_manga_update(
		&self,
		manga: Manga,
		needs_details: bool,
		needs_chapters: bool,
	) -> Result<Manga> {
		let mut new_manga = manga.clone();
		if needs_details {
			new_manga.authors = Some(vec![String::from("Author")]);
			new_manga.description = ExampleSource::get_latest_aidoku_version();
			new_manga.status = MangaStatus::Ongoing;
			new_manga.content_rating = ContentRating::Safe;
			new_manga.tags = Some(vec![String::from("Tag 1"), String::from("Tag 2")]);
			new_manga.url = Some(String::from("https://aidoku.app"));
		}
		if needs_chapters {
			new_manga.chapters = Some(vec![
				Chapter {
					key: String::from("8"),
					chapter_number: Some(8.0),
					..Default::default()
				},
				Chapter {
					key: String::from("7"),
					chapter_number: Some(7.0),
					title: Some(String::from("Title")),
					..Default::default()
				},
				Chapter {
					key: String::from("6"),
					chapter_number: Some(6.0),
					title: Some(String::from("Title")),
					date_uploaded: Some(1692318525),
					..Default::default()
				},
				Chapter {
					key: String::from("5"),
					chapter_number: Some(5.0),
					..Default::default()
				},
				Chapter {
					key: String::from("4"),
					chapter_number: Some(4.0),
					..Default::default()
				},
				Chapter {
					key: String::from("3"),
					chapter_number: Some(3.0),
					..Default::default()
				},
				Chapter {
					key: String::from("2"),
					chapter_number: Some(2.0),
					..Default::default()
				},
				Chapter {
					key: String::from("1"),
					chapter_number: Some(1.0),
					..Default::default()
				},
			]);
		}
		Ok(new_manga)
	}

	fn get_page_list(&self, _manga: Manga, _chapter: Chapter) -> Result<Vec<Page>> {
		Ok(vec![
			Page {
				content: PageContent::url("https://aidoku.app/images/icon.png"),
				has_description: true,
				description: Some("Description".into()),
				..Default::default()
			},
			Page {
				content: PageContent::text(
					"# Title\n\nThis is some description\n\n## Section\n\nThis is a section.",
				),
				has_description: true,
				description: None,
				..Default::default()
			},
		])
	}
}

impl ExampleSource {
	// gets the latest version of aidoku from the github releases page
	fn get_latest_aidoku_version() -> Option<String> {
		Request::get("https://github.com/aidoku/aidoku/releases")
			.ok()?
			.html()
			.ok()?
			.select_first(".repository-content .Box a")?
			.text()
	}
}

// use the home trait to implement a home page for a source
// where possible, try to replicate the associated web page's layout
impl Home for ExampleSource {
	fn get_home(&self) -> Result<HomeLayout> {
		let entries = self.get_search_manga_list(None, 1, Vec::new())?.entries;
		let chapter = Chapter {
			key: String::from("1"),
			chapter_number: Some(1.0),
			title: Some(String::from("Chapter")),
			date_uploaded: Some(1692318525),
			..Default::default()
		};
		let manga_chapters = entries
			.iter()
			.map(|manga| MangaWithChapter {
				manga: manga.clone(),
				chapter: chapter.clone(),
			})
			.take(3)
			.collect::<Vec<_>>();
		Ok(HomeLayout {
			components: vec![
				HomeComponent {
					title: Some(String::from("Big Scroller")),
					subtitle: None,
					value: aidoku::HomeComponentValue::BigScroller {
						entries: entries.clone(),
						auto_scroll_interval: Some(10.0),
					},
				},
				HomeComponent {
					title: Some(String::from("Manga Chapter List")),
					subtitle: None,
					value: aidoku::HomeComponentValue::MangaChapterList {
						page_size: None,
						entries: manga_chapters,
						listing: None,
					},
				},
				HomeComponent {
					title: Some(String::from("Manga List")),
					subtitle: None,
					value: aidoku::HomeComponentValue::MangaList {
						ranking: false,
						page_size: None,
						entries: entries.iter().take(2).cloned().map(|m| m.into()).collect(),
						listing: None,
					},
				},
				HomeComponent {
					title: Some(String::from("Manga List (Paged, Ranking)")),
					subtitle: None,
					value: aidoku::HomeComponentValue::MangaList {
						ranking: true,
						page_size: Some(3),
						entries: entries.iter().take(8).cloned().map(|m| m.into()).collect(),
						listing: None,
					},
				},
				HomeComponent {
					title: Some(String::from("Scroller")),
					subtitle: None,
					value: aidoku::HomeComponentValue::Scroller {
						entries: entries.clone().into_iter().map(|m| m.into()).collect(),
						listing: None,
					},
				},
				HomeComponent {
					title: Some("Filters".into()),
					subtitle: None,
					value: aidoku::HomeComponentValue::Filters(vec![
						aidoku::FilterItem::from(String::from("Action")),
						"Adventure".into(),
						"Fantasy".into(),
						"Horror".into(),
						"Slice of Life".into(),
						"Magic".into(),
						"Adaptation".into(),
					]),
				},
				HomeComponent {
					title: Some(String::from("Links")),
					subtitle: None,
					value: aidoku::HomeComponentValue::Links(vec![
						aidoku::Link {
							title: String::from("Website Link"),
							value: Some(aidoku::LinkValue::Url(String::from("https://aidoku.app"))),
							..Default::default()
						},
						aidoku::Link {
							title: String::from("Manga Link"),
							value: Some(aidoku::LinkValue::Manga(entries.first().unwrap().clone())),
							..Default::default()
						},
						aidoku::Link {
							title: String::from("Listing Link"),
							value: Some(aidoku::LinkValue::Listing(Listing {
								id: String::from("listing"),
								name: String::from("Listing"),
								kind: aidoku::ListingKind::List,
							})),
							..Default::default()
						},
					]),
				},
			],
		})
	}
}

// to provide page descriptions asynchronously, use the PageDescriptionProvider trait
// if fetching a page description requires an additional request, use this trait,
// otherwise just provide it when fetching the page list
impl PageDescriptionProvider for ExampleSource {
	fn get_page_description(&self, _page: Page) -> Result<String> {
		Ok("# Title\n\nThis is some description\n\n## Section\n\nThis is a section.".into())
	}
}

// if your source changes filters frequently or only has some filters available conditionally, use the DynamicFilters trait
// where possible, static filters are preferred
impl DynamicFilters for ExampleSource {
	fn get_dynamic_filters(&self) -> Result<Vec<Filter>> {
		Ok(vec![
			TextFilter {
				id: "text".into(),
				title: Some("Text".into()),
				placeholder: Some("Search".into()),
				..Default::default()
			}
			.into(),
			SortFilter {
				id: "sort".into(),
				title: Some("Sort".into()),
				can_ascend: true,
				options: vec!["Popular".into(), "Recent".into()],
				..Default::default()
			}
			.into(),
			CheckFilter {
				id: "check".into(),
				title: Some("Check".into()),
				can_exclude: true,
				..Default::default()
			}
			.into(),
			SelectFilter {
				id: "select".into(),
				title: Some("Select".into()),
				uses_tag_style: true,
				options: vec!["One".into(), "Two".into()],
				..Default::default()
			}
			.into(),
			MultiSelectFilter {
				id: "mselect".into(),
				title: Some("Multi-Select".into()),
				can_exclude: true,
				uses_tag_style: false,
				options: vec!["One".into(), "Two".into()],
				..Default::default()
			}
			.into(),
			Filter::note("Testing note"),
		])
	}
}

// if you need to serve settings dynamically, use the DynamicSettings trait
// again, this shouldn't be used for static settings
impl DynamicSettings for ExampleSource {
	fn get_dynamic_settings(&self) -> Result<Vec<Setting>> {
		let toggle_value = defaults_get::<bool>("setting");
		let mut settings = vec![ToggleSetting {
			key: "setting".into(),
			title: "Toggle".into(),
			notification: Some("test".into()),
			refreshes: Some(vec!["settings".into()]),
			..Default::default()
		}
		.into()];
		if let Some(value) = toggle_value {
			if value {
				settings.push(
					ToggleSetting {
						key: "setting2".into(),
						title: "Toggle 2".into(),
						..Default::default()
					}
					.into(),
				);
			}
		}
		Ok(settings)
	}
}

// if you need to serve listings dynamically, use the DynamicListings trait
// again, this shouldn't be used for static listings
// for example, you could fetch listings from an API, or show one if a certain setting is enabled
impl DynamicListings for ExampleSource {
	fn get_dynamic_listings(&self) -> Result<Vec<Listing>> {
		Ok(vec![Listing {
			id: String::from("listing"),
			name: String::from("Listing"),
			kind: aidoku::ListingKind::List,
		}])
	}
}

// if you need to perform any actions when settings change, use the NotificationHandler trait
// for example, you could update different defaults values
impl NotificationHandler for ExampleSource {
	fn handle_notification(&self, key: String) {
		println!("Notification: {key}");
	}
}

// if your source supports displaying multiple covers for a title, use the AlternateCoverProvider trait
impl AlternateCoverProvider for ExampleSource {
	fn get_alternate_covers(&self, _manga: Manga) -> Result<Vec<String>> {
		Ok(vec!["https://aidoku.app/images/icon.png".into()])
	}
}

// it's recommended for all sources to implement the DeepLinkHandler trait
// the url that is passed in will have the base of any of the source's urls
// the source should determine if the url is a link to a manga, a chapter, or a listing page,
// then return the appropriate DeepLinkResult to handle it.
impl DeepLinkHandler for ExampleSource {
	fn handle_deep_link(&self, _url: String) -> Result<Option<DeepLinkResult>> {
		Ok(Some(DeepLinkResult::Manga {
			key: String::from("manga_key"),
		}))
	}
}

// the register_source! macro generates the necessary wasm functions for aidoku
register_source!(
	ExampleSource,
	// after the name of the source struct, list all the extra traits it implements
	Home,
	PageDescriptionProvider,
	DynamicFilters,
	DynamicSettings,
	DynamicListings,
	NotificationHandler,
	AlternateCoverProvider,
	DeepLinkHandler
);

// you can also implement tests via our custom test runner!
#[cfg(test)]
mod test {
	use super::*;
	use aidoku_test::aidoku_test;

	// all tests need to be annotated with the #[aidoku_test] attribute instead of #[test]
	#[aidoku_test]
	fn test_request() {
		let version = ExampleSource::get_latest_aidoku_version();
		println!("{:?}", version); // if the test fails (or you pass --nocapture), you can see this in the log,
		assert!(version.is_some());
		assert!(version.unwrap().chars().next().unwrap() == 'v');
	}

	#[aidoku_test]
	fn test_js_execution() {
		// most aidoku imports you'd want to use should also work
		use aidoku::imports::js::JsContext;
		let context = JsContext::new();
		let result = context.eval("1 + 2");
		assert_eq!(result, Ok(String::from("3")));
	}
}
