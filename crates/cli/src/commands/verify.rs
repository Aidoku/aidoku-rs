//! Verify an Aidoku source to ensure it's ready to be published.
use crate::models::SourceJson;
use anyhow::anyhow;
use colored::Colorize;
use image::GenericImageView;
use semver::Version;
use serde_json::Value;
use std::io::prelude::*;
use wasmparser::{Parser, Payload};

// include json schemas as static strings
const SOURCE_JSON_SCHEMA: &str = include_str!("../supporting/schema/source.schema.json");
const FILTERS_JSON_SCHEMA: &str = include_str!("../supporting/schema/filters.schema.json");
const SETTINGS_JSON_SCHEMA: &str = include_str!("../supporting/schema/settings.schema.json");

pub fn run(files: Vec<std::path::PathBuf>) -> anyhow::Result<()> {
	// ensure files were provided
	if files.is_empty() {
		return Err(anyhow!("no files provided"));
	}

	let mut found_error = false;

	// load json schemas
	let source_json_schema: Value = serde_json::from_str(SOURCE_JSON_SCHEMA)?;
	let source_json_validator = jsonschema::validator_for(&source_json_schema)?;
	let filters_json_schema: Value = serde_json::from_str(FILTERS_JSON_SCHEMA)?;
	let filters_json_validator = jsonschema::validator_for(&filters_json_schema)?;
	let settings_json_schema: Value = serde_json::from_str(SETTINGS_JSON_SCHEMA)?;
	let settings_json_validator = jsonschema::validator_for(&settings_json_schema)?;

	// validate each file
	for (idx, path) in files.iter().enumerate() {
		if idx > 0 {
			// add a new line between verification logs for different packages
			println!();
		}
		println!("* Testing {}", path.display());

		let Some(mut archive) = std::fs::File::open(path)
			.ok()
			.and_then(|file| zip::read::ZipArchive::new(std::io::BufReader::new(file)).ok())
		else {
			println!("  {}", "* failed to read file".red());
			continue;
		};

		let source_json: Option<SourceJson> = {
			let mut contents = String::new();
			if archive
				.by_name("Payload/source.json")
				.map(|mut file| file.read_to_string(&mut contents).is_ok())
				.unwrap_or(false)
			{
				serde_json::from_str(&contents).ok()
			} else {
				None
			}
		};

		// main.wasm
		let has_main_wasm = if let Ok(file) = archive.by_name("Payload/main.wasm") {
			println!("  * main.wasm");
			validate_wasm(file, source_json)
		} else {
			println!("  {}", "* missing main.wasm".red());
			false
		};

		// icon.png
		let icon_valid = if let Ok(mut file) = archive.by_name("Payload/icon.png") {
			println!("  * icon.png");

			let img = {
				let mut buffer = Vec::new();
				file.read_to_end(&mut buffer).ok().and_then(|_| {
					image::ImageReader::new(std::io::Cursor::new(buffer))
						.with_guessed_format()
						.ok()
						.and_then(|i| i.decode().ok())
				})
			};

			if let Some(img) = img {
				print!("    * dimensions are 128x128... ");

				let (width, height) = img.dimensions();
				let valid_dimensions = if width != 128 || height != 128 {
					println!("{}", format!("no, found {width}x{height}").red());
					false
				} else {
					println!("{}", "yes".green());
					true
				};

				fn is_fully_opaque(img: &image::DynamicImage) -> bool {
					let rgba_img = img.to_rgba8();
					for pixel in rgba_img.pixels() {
						if pixel[3] < 255 {
							return false; // alpha channel < 255 is transparent
						}
					}
					true
				}
				let opaque = is_fully_opaque(&img);
				println!(
					"    * is fully opaque... {}",
					if opaque { "yes".green() } else { "no".red() }
				);

				valid_dimensions && opaque
			} else {
				println!("    {}", "* failed to read file".red());
				false
			}
		} else {
			println!("  {}", "* missing icon.png".red());
			false
		};

		// source.json
		let source_json_valid = validate_json(
			&mut archive,
			"Payload/source.json",
			&source_json_validator,
			true,
		);

		// filters.json
		let filters_json_valid = validate_json(
			&mut archive,
			"Payload/filters.json",
			&filters_json_validator,
			false,
		);

		// settings.json
		let settings_json_valid = validate_json(
			&mut archive,
			"Payload/settings.json",
			&settings_json_validator,
			false,
		);

		if !(has_main_wasm
			&& icon_valid
			&& source_json_valid
			&& filters_json_valid
			&& settings_json_valid)
		{
			found_error = true;
		}
	}

	// if any errors were found with any of the files
	if found_error {
		if files.len() == 1 {
			Err(anyhow!("validation failed"))
		} else {
			Err(anyhow!("one or more packages failed validation, see above"))
		}
	} else {
		Ok(())
	}
}

// ensure a json file is valid against a schema
fn validate_json(
	archive: &mut zip::ZipArchive<std::io::BufReader<std::fs::File>>,
	path: &str,
	validator: &jsonschema::Validator,
	required: bool,
) -> bool {
	let file_name = path.split('/').next_back().unwrap_or_default();

	if let Ok(mut source_json) = archive.by_name(path) {
		println!("  * {file_name}");

		let mut contents = String::new();

		if let Some(result) = source_json
			.read_to_string(&mut contents)
			.ok()
			.and_then(|_| serde_json::from_str::<Value>(&contents).ok())
		{
			let valid = validator.is_valid(&result);
			println!(
				"    * is valid against schema... {}",
				if valid { "yes".green() } else { "no".red() }
			);
			valid
		} else {
			println!("    {}", "* failed to read file".red());
			false
		}
	} else if required {
		let msg = format!("* missing {file_name}");
		println!("  {}", msg.red());
		false
	} else {
		true
	}
}

// the mimimum exports required for a source to work
#[derive(Default)]
struct RequiredExports {
	start: bool,
	free_result: bool,
	get_search_manga_list: bool,
	get_manga_update: bool,
	get_page_list: bool,
}

impl RequiredExports {
	fn new() -> Self {
		Self::default()
	}

	fn all_satisfied(&self) -> bool {
		self.start
			&& self.free_result
			&& self.get_search_manga_list
			&& self.get_manga_update
			&& self.get_page_list
	}

	fn mark(&mut self, name: &str) {
		match name {
			"start" => self.start = true,
			"free_result" => self.free_result = true,
			"get_search_manga_list" => self.get_search_manga_list = true,
			"get_manga_update" => self.get_manga_update = true,
			"get_page_list" => self.get_page_list = true,
			_ => {}
		}
	}
}

fn validate_wasm(
	mut wasm_file: zip::read::ZipFile<'_, std::io::BufReader<std::fs::File>>,
	source_json: Option<SourceJson>,
) -> bool {
	let mut wasm_bytes = Vec::new();
	if !wasm_file.read_to_end(&mut wasm_bytes).is_ok() {
		return false;
	}

	let mut exports = RequiredExports::new();
	let mut api_min_version = Version::new(0, 7, 0);

	let mut exports_checked = false;
	let mut imports_checked = false;

	for payload in Parser::new(0).parse_all(&wasm_bytes) {
		let Ok(payload) = payload else {
			continue;
		};
		match payload {
			Payload::ImportSection(s) => {
				for import in s.into_imports() {
					let Ok(import) = import else {
						continue;
					};
					use wasmparser::Import;
					let v = match import {
						Import {
							module: "html",
							name: "kind" | "child_nodes",
							..
						} => Version::new(0, 8, 3),
						Import {
							module: "net",
							name: "get_url" | "set_timeout",
							..
						} => Version::new(0, 8, 3),
						Import {
							module: "html",
							name:
								"remove" | "add_class" | "remove_class" | "set_attr" | "remove_attr",
							..
						} => Version::new(0, 8, 0),
						Import {
							module: "std",
							name: "parse_date",
							..
						} => Version::new(0, 7, 1),
						_ => continue,
					};
					if v > api_min_version {
						api_min_version = v;
					}
				}
				imports_checked = true;
			}
			Payload::ExportSection(s) => {
				for export in s {
					let Ok(export) = export else {
						continue;
					};
					exports.mark(export.name);
				}
				exports_checked = true;
			}
			_ => continue,
		}
		if exports_checked && imports_checked {
			break;
		}
	}

	let defined_min_version = source_json
		.and_then(|json| json.info.min_app_version)
		.and_then(|v| Version::parse(&v).ok())
		.unwrap_or(Version::new(0, 7, 0));

	let api_valid = api_min_version <= defined_min_version;
	println!(
		"    * defined minimum version accepts api version... {}",
		if api_valid {
			"yes".green()
		} else {
			format!("no (api version: {api_min_version})").red()
		}
	);

	let exports_valid = exports.all_satisfied();
	println!(
		"    * minimum functions exported... {}",
		if exports_valid {
			"yes".green()
		} else {
			"no".red()
		}
	);

	api_valid && exports_valid
}
