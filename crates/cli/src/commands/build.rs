//! Build a source list.
use crate::models::*;
use anyhow::{Context, anyhow};
use rayon::prelude::*;
use std::io::prelude::*;

pub fn run(
	files: Vec<std::path::PathBuf>,
	output_path: &std::path::PathBuf,
	name: Option<String>,
) -> anyhow::Result<()> {
	// ensure files were provided
	if files.is_empty() {
		return Err(anyhow!("No source package files provided"));
	}

	// create output directory
	std::fs::create_dir_all(output_path).context("Failed to create output directory")?;

	// create sources and icons folders
	let sources_path = output_path.join("sources");
	let icons_path = output_path.join("icons");
	std::fs::create_dir_all(&sources_path).context("Failed to create output sources directory")?;
	std::fs::create_dir_all(&icons_path).context("Failed to create output icons directory")?;

	// process each provided source package file
	let mut results: Vec<SourceJson> = files
		.par_iter() // rayon for parallelization
		.filter_map(|zip_path| {
			let Ok(file) = std::fs::File::open(zip_path) else {
				eprintln!("error: failed to open {}", zip_path.display());
				return None;
			};
			let mut archive = zip::read::ZipArchive::new(std::io::BufReader::new(file)).ok()?;

			// parse source.json
			let parsed: SourceJson = {
				let mut source_json = archive.by_name("Payload/source.json").ok()?;

				let mut contents = String::new();
				source_json.read_to_string(&mut contents).ok()?;

				let Ok(result) = serde_json::from_str(&contents) else {
					eprintln!(
						"error: failed to parse source.json for {}",
						zip_path.display()
					);
					return None;
				};
				result
			};

			// copy package file to sources directory
			let output_filename = format!("{}-v{}.aix", parsed.info.id, parsed.info.version);
			let output_path = sources_path.join(output_filename);
			std::fs::copy(zip_path, &output_path).ok()?;

			// copy icon file to icons directory
			if let Ok(mut icon_file) = archive.by_name("Payload/icon.png") {
				let mut buffer = Vec::new();
				if icon_file.read_to_end(&mut buffer).is_ok() {
					let icon_filename = format!("{}-v{}.png", parsed.info.id, parsed.info.version);
					let icon_path = icons_path.join(icon_filename);
					if let Ok(mut output_file) = std::fs::File::create(&icon_path) {
						if output_file.write_all(&buffer).is_err() {
							eprintln!("warning: failed to write icon to {}", icon_path.display());
						}
					} else {
						eprintln!(
							"warning: failed to create icon file at {}",
							icon_path.display()
						);
					}
				}
			} else {
				eprintln!("warning: no icon.png found in {}", zip_path.display());
			}

			Some(parsed)
		})
		.collect();

	results.sort_by(|a, b| a.info.id.cmp(&b.info.id));

	let items: Vec<SourcesItem> = results
		.into_iter()
		.map(|json| SourcesItem {
			id: json.info.id.clone(),
			name: json.info.name.clone(),
			version: json.info.version,
			icon_url: format!("icons/{}-v{}.png", json.info.id, json.info.version),
			download_url: format!("sources/{}-v{}.aix", json.info.id, json.info.version),
			languages: json.info.languages,
			content_rating: json
				.info
				.content_rating
				.unwrap_or(SourceContentRating::Safe),
			alt_names: json.info.alt_names,
			base_url: match json.info.url {
				url @ Some(_) => url,
				None => json.info.urls.and_then(|urls| urls.first().cloned()),
			},
		})
		.collect();

	let result = SourceList {
		name: name.unwrap_or("Source List".into()),
		sources: items,
	};

	// write items to index.json
	let index_path = output_path.join("index.json");
	let index_min_path = output_path.join("index.min.json");
	let mut index_file = std::fs::File::create(&index_path)?;
	let mut index_min_file = std::fs::File::create(&index_min_path)?;
	serde_json::to_writer_pretty(&mut index_file, &result)?;
	serde_json::to_writer(&mut index_min_file, &result)?;

	Ok(())
}
