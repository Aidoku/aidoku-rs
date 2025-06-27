//! Initialize a new Aidoku source.
use crate::models::{SourceContentRating, SourceInfo, SourceJson};
use anyhow::{anyhow, Context};
use dialoguer::{Input, MultiSelect, Select};
use std::io::prelude::*;
use std::path::PathBuf;

// iso 639-2 language codes
// technically 639-3 codes are also valid for the app
static VALID_LANGUAGE_CODES: &[&str] = &[
	"ab", "aa", "af", "ak", "sq", "am", "ar", "an", "hy", "as", "av", "ae", "ay", "az", "bm", "ba",
	"eu", "be", "bn", "bi", "bs", "br", "bg", "my", "ca", "ch", "ce", "ny", "zh", "cu", "cv", "kw",
	"co", "cr", "hr", "cs", "da", "dv", "nl", "dz", "en", "eo", "et", "ee", "fo", "fj", "fi", "fr",
	"fy", "ff", "gd", "gl", "lg", "ka", "de", "el", "kl", "gn", "gu", "ht", "ha", "he", "hz", "hi",
	"ho", "hu", "is", "io", "ig", "id", "ia", "ie", "iu", "ik", "ga", "it", "ja", "jv", "kn", "kr",
	"ks", "kk", "km", "ki", "rw", "ky", "kv", "kg", "ko", "kj", "ku", "lo", "la", "lv", "li", "ln",
	"lt", "lu", "lb", "mk", "mg", "ms", "ml", "mt", "gv", "mi", "mr", "mh", "mn", "na", "nv", "nd",
	"nr", "ng", "ne", "no", "nb", "nn", "oc", "oj", "or", "om", "os", "pi", "ps", "fa", "pl", "pt",
	"pa", "qu", "ro", "rm", "rn", "ru", "se", "sm", "sg", "sa", "sc", "sr", "sn", "sd", "si", "sk",
	"sl", "so", "st", "es", "su", "sw", "ss", "sv", "tl", "ty", "tg", "ta", "tt", "te", "th", "bo",
	"ti", "to", "ts", "tn", "tr", "tk", "tw", "ug", "uk", "ur", "uz", "ve", "vi", "vo", "wa", "cy",
	"wo", "xh", "ii", "yi", "yo", "za", "zu",
	// lots more regions can also be specified, but here are a few to allow as edge cases
	"fil", "zh-Hans", "zh-Hant", "pt-br", "es-419",
];

// load template files as static strings
const CONFIG_TEMPLATE: &str = include_str!("../supporting/templates/config.toml.template");
const CARGO_TEMPLATE: &str = include_str!("../supporting/templates/Cargo.toml.template");
const SOURCE_LIB_TEMPLATE: &str = include_str!("../supporting/templates/source-lib.rs.template");
const SOURCE_TEMPLATE_LIB_TEMPLATE: &str =
	include_str!("../supporting/templates/source-template-lib.rs.template");
const TEMPLATE_LIB_TEMPLATE: &str =
	include_str!("../supporting/templates/template-lib.rs.template");
const CDYLIB_STR: &str = "
[lib]
crate-type = [\"cdylib\"]

";

pub fn run(
	path: Option<std::path::PathBuf>,
	name: Option<String>,
	url: Option<String>,
	mut languages: Vec<String>,
	content_rating: Option<SourceContentRating>,
	template: bool,
	template_name: Option<String>,
) -> anyhow::Result<()> {
	// verify already input languages
	for lang in &languages {
		if !VALID_LANGUAGE_CODES.contains(&lang.as_str()) {
			return Err(anyhow!("Invalid language code: {}", lang));
		}
	}

	let current_dir = std::env::current_dir().context("Failed to get current directory")?;
	let target_path = path.unwrap_or(current_dir.clone());

	// use directory name as default source name
	let directory_name = target_path
		.file_name()
		.context("Failed to get directory name")?
		.to_string_lossy();

	let template_name = if template {
		Some(match template_name {
			Some(n) => n,
			None => Input::<String>::new()
				.with_prompt("Template name")
				.interact_text()?,
		})
	} else {
		None
	};

	// if name is None, prompt for it. use directory name as default
	let name = match name {
		Some(n) => n,
		None => Input::new()
			.with_prompt("Source name")
			.default(directory_name.to_string())
			.interact_text()?,
	};

	// if url is None, prompt for it
	let url = match url {
		Some(u) => u,
		None => Input::new()
			.with_prompt("Source URL")
			.validate_with(|input: &String| -> Result<(), &str> {
				if input.starts_with("http://") || input.starts_with("https://") {
					Ok(())
				} else {
					Err("URL must start with http:// or https://")
				}
			})
			.interact_text()?,
	};

	// get language codes
	if languages.is_empty() {
		let available_languages = [
			("English", "en"),
			("Chinese", "zh"),
			("Spanish", "es"),
			("Russian", "ru"),
			("Vietnamese", "vi"),
			("Other", ""),
		];

		let language_names: Vec<&str> = available_languages.iter().map(|(name, _)| *name).collect();

		let selections = MultiSelect::new()
			.with_prompt("Select supported languages")
			.items(&language_names)
			.interact()?;

		if selections.is_empty() {
			return Err(anyhow!("You must select at least one language"));
		}

		for i in selections {
			if language_names[i] == "Other" {
				let other_input: String = Input::new()
					.with_prompt("Enter additional language codes (e.g. `id pt ja`)")
					.validate_with(|input: &String| -> Result<(), &str> {
						let codes = input.split_whitespace().map(|s| s.to_lowercase());
						for code in codes {
							if !VALID_LANGUAGE_CODES.contains(&code.as_str()) {
								return Err(
									"One or more language codes are not valid ISO 639-2 codes",
								);
							}
						}
						Ok(())
					})
					.interact_text()?;

				let mut other_codes: Vec<String> = other_input
					.split_whitespace()
					.map(|s| s.to_lowercase())
					.collect();

				languages.append(&mut other_codes);
			} else {
				let code = available_languages[i].1.to_string();
				languages.push(code);
			}
		}
	}

	// get content rating
	let content_rating = match content_rating {
		Some(rating) => rating,
		None => {
			let rating_options = &["Safe", "Contains NSFW content", "Primarily NSFW content"];
			let selection = Select::new()
				.with_prompt("Select content rating")
				.items(rating_options)
				.default(0)
				.interact()?;

			match selection {
				0 => SourceContentRating::Safe,
				1 => SourceContentRating::ContainsNsfw,
				2 => SourceContentRating::PrimarilyNsfw,
				_ => unreachable!(),
			}
		}
	};

	// create and move to the target directory if necessary
	if target_path != current_dir {
		std::fs::create_dir_all(&target_path).context("Failed to create new directory")?;
		std::env::set_current_dir(&target_path).context("Failed to change directory")?;
	}

	// determine info from configuration
	fn create_package_name(name: &str, template: bool) -> String {
		let name = name
			.chars()
			.map(|c| c.to_ascii_lowercase()) // lowercase
			.map(|c| if c == ' ' { '-' } else { c }) // replace spaces with hyphens
			.filter(|c| c.is_ascii_alphanumeric() || *c == '-' || *c == '_') // ensure unicode-xid compliant
			.collect::<String>()
			.trim_matches(|c| c == '_' || c == '-') // remove trailing and leading underscores and hyphens
			.to_string();
		if name == "test" {
			if template {
				"test_template".to_string()
			} else {
				"test_source".to_string()
			}
		} else {
			name
		}
	}
	let mut package_name = create_package_name(&name, false);
	// handle edge case for reserved package names
	if package_name == "test" {
		package_name = "test_source".to_string();
	}
	let id = format!(
		"{}.{}",
		if languages.len() == 1 {
			&languages[0]
		} else {
			"multi"
		},
		package_name
	);
	let source_json = SourceJson {
		info: SourceInfo {
			id,
			name,
			alt_names: None,
			version: 1,
			url: Some(url),
			urls: None,
			content_rating: Some(content_rating),
			languages,
		},
	};

	if let Some(template_name) = template_name {
		let template_package_name = create_package_name(&template_name, false);
		let template_name = template_name
			.chars()
			.filter(|c| c.is_ascii_alphanumeric())
			.collect::<String>();

		// create workspace Cargo.toml
		let cargo_toml = PathBuf::from("Cargo.toml");
		std::fs::write(&cargo_toml, CARGO_TEMPLATE)
			.context("Failed to create workspace Cargo.toml")?;

		create_template_files(&template_package_name, &template_name)?;
		create_source_files(
			&package_name,
			source_json,
			Some((&template_package_name, &template_name)),
		)?;
	} else {
		create_source_files(&package_name, source_json, None)?;
		println!("configured a source");
	}

	Ok(())
}

// create a new cargo library and add aidoku dependencies to it
// moves the current directory into a newly created directory if in_workspace is true
fn create_empty_aidoku_lib(
	name: &str,
	in_workspace: bool,
	template_name: Option<(&str, &str)>,
) -> Result<(), anyhow::Error> {
	// initialize via cargo
	let mut cmd = &mut std::process::Command::new("cargo");
	if in_workspace {
		// if we're in a workspace, create a new directory
		cmd = cmd.arg("new").arg(name);
	} else {
		// otherwise, use the current directory
		cmd = cmd.arg("init").arg("--name").arg(name);
	}
	cmd.arg("--lib")
		.stdout(std::process::Stdio::null())
		.stderr(std::process::Stdio::null())
		.status()
		.context("Failed to initialize via cargo")?;

	// if this is a workspace, move into the new directory we created
	if in_workspace {
		std::env::set_current_dir(PathBuf::from(name)).context("Failed to change directory")?;
	}

	// add aidoku library dependencies
	let mut add_lib_cmd = std::process::Command::new("cargo");
	let mut add_test_lib_cmd = std::process::Command::new("cargo");
	let mut add_test_macro_cmd = std::process::Command::new("cargo");

	let mut commands = vec![
		// todo: remove --branch when this lands on main
		add_lib_cmd
			.arg("add")
			.arg("aidoku")
			.arg("--git")
			.arg("https://github.com/Aidoku/aidoku-rs.git"),
		add_test_lib_cmd
			.arg("add")
			.arg("aidoku")
			.arg("--git")
			.arg("https://github.com/Aidoku/aidoku-rs.git")
			.arg("--features")
			.arg("test")
			.arg("--dev"),
		add_test_macro_cmd
			.arg("add")
			.arg("aidoku-test")
			.arg("--git")
			.arg("https://github.com/Aidoku/aidoku-rs.git")
			.arg("--dev"),
	];

	// if a template name is provided, add it as a dependency
	let mut add_template_cmd = std::process::Command::new("cargo");
	if let Some((template_package_name, _)) = template_name {
		commands.push(
			add_template_cmd
				.arg("add")
				.arg("--path")
				.arg(format!("../{template_package_name}")),
		);
	}

	for command in commands {
		let result = command
			.stdout(std::process::Stdio::null())
			.stderr(std::process::Stdio::null())
			.status()
			.context("Failed to initialize source via cargo")?;

		if !result.success() {
			return Err(anyhow!("Failed to initialize source via cargo"));
		}
	}

	Ok(())
}

// creates files for a new source
fn create_source_files(
	package_name: &str,
	source_json: SourceJson,
	template_name: Option<(&str, &str)>,
) -> Result<(), anyhow::Error> {
	let in_workspace = template_name.is_some();

	// creates and moves into our new project folder if necessary
	create_empty_aidoku_lib(package_name, in_workspace, template_name)?;

	// override lib.rs
	let lib_rs_path = PathBuf::from("src/lib.rs");
	let source_name = source_json
		.info
		.name
		.chars()
		.filter(|c| c.is_ascii_alphanumeric())
		.collect::<String>();
	let lib_rs_content = if let Some((template_package_name, template_name)) = template_name {
		SOURCE_TEMPLATE_LIB_TEMPLATE
			.replace("{{SOURCE_NAME}}", &source_name)
			.replace("{{TEMPLATE_NAME}}", template_name)
			.replace(
				"{{TEMPLATE_LIB_NAME}}",
				&template_package_name.replace("-", "_"),
			)
	} else {
		SOURCE_LIB_TEMPLATE.replace("{{SOURCE_NAME}}", &source_name)
	};
	std::fs::write(&lib_rs_path, lib_rs_content).context("Failed to write template lib.rs")?;

	// add wasm config stuff on to Cargo.toml
	let cargo_toml = PathBuf::from("Cargo.toml");
	let mut cargo_toml_content =
		std::fs::read_to_string(&cargo_toml).context("Failed to read Cargo.toml")?;
	cargo_toml_content.push_str(CDYLIB_STR);
	// if we're not in a workspace, add the profile stuff
	if !in_workspace {
		// use the profile configuration from workspace Cargo.toml template
		let profile_text = CARGO_TEMPLATE
			.lines()
			.skip(3)
			.collect::<Vec<_>>()
			.join("\n");
		cargo_toml_content.push_str(&profile_text);
	}
	std::fs::write(&cargo_toml, cargo_toml_content).context("Failed to write Cargo.toml")?;

	// create .cargo/config.toml
	let config_path = PathBuf::from(".cargo");
	std::fs::create_dir_all(&config_path).context("Failed to create .cargo directory")?;
	let config_toml = config_path.join("config.toml");
	std::fs::write(&config_toml, CONFIG_TEMPLATE).context("Failed to create config.toml")?;

	// create res folder
	let res_path = PathBuf::from("res");
	std::fs::create_dir_all(&res_path).context("Failed to create res directory")?;

	// create source.json
	let source_json_path = res_path.join("source.json");
	let tabs_pretty = serde_json::ser::PrettyFormatter::with_indent(b"\t"); // indent with tabs
	let file = std::fs::File::create(&source_json_path).context("Failed to create json file")?;
	let mut w = std::io::BufWriter::new(file);
	let mut ser = serde_json::Serializer::with_formatter(&mut w, tabs_pretty);
	serde::Serialize::serialize(&source_json, &mut ser).context("Failed to write to json file")?;
	w.write_all(b"\n")?; // add a newline after the json data
	w.flush()?;

	// create empty icon.png
	let icon_path = res_path.join("icon.png");
	std::fs::File::create(&icon_path).context("Failed to create empty icon.png")?;

	// if we created and moved into a directory, move out of it
	if in_workspace {
		std::env::set_current_dir(PathBuf::from("..")).context("Failed to change directory")?;
	}

	Ok(())
}

// creates files for a new source template
fn create_template_files(package_name: &str, name: &str) -> Result<(), anyhow::Error> {
	create_empty_aidoku_lib(package_name, true, None)?;

	// override lib.rs
	let lib_rs_path = PathBuf::from("src/lib.rs");
	let lib_rs_content = TEMPLATE_LIB_TEMPLATE.replace("{{TEMPLATE_NAME}}", name);
	std::fs::write(&lib_rs_path, lib_rs_content).context("Failed to write lib.rs template")?;

	// move out of the directory we created
	std::env::set_current_dir(PathBuf::from("..")).context("Failed to change directory")?;

	Ok(())
}
