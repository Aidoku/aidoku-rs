//! Build and package an Buny source.
use anyhow::{anyhow, Context};
use std::io::prelude::*;

pub fn run(path: Option<std::path::PathBuf>) -> anyhow::Result<()> {
	// move to the directory if specified
	if let Some(path) = path {
		let path = path.as_path();
		if !path.exists() {
			return Err(anyhow!("Path does not exist"));
		}
		std::env::set_current_dir(path).context("Failed to change directory")?;
	}

	let current_path = std::env::current_dir().context("Failed to get current directory")?;

	// build rust source
	let result = std::process::Command::new("cargo")
		.arg("build")
		.arg("--release")
		.arg("--target")
		.arg("wasm32-unknown-unknown")
		.status()
		.context("Failed to build source")?;
	if !result.success() {
		return Err(anyhow!("Build failed"));
	}

	// folder containing `source.json` file
	let res_dir = current_path.join("res");

	if !res_dir.exists() {
		return Err(anyhow!("res directory does not exist"));
	}

	// find the build folder containing resulting wasm file
	// start from current directory and go up to 3 parent directories if not found
	let build_dir = {
		let mut result = None;

		let mut cur_parent = current_path.clone();
		let mut build_dir: std::path::PathBuf;

		let max_parents = 3; // only check up to 3 parent directories
		let mut parent_count = 0;

		loop {
			build_dir = cur_parent
				.join("target")
				.join("wasm32-unknown-unknown")
				.join("release");

			if build_dir.exists() {
				result = Some(build_dir);
				break;
			}

			if let Some(parent) = cur_parent.parent() {
				cur_parent = parent.to_path_buf();
				parent_count += 1;
				if parent_count >= max_parents {
					// reached the max parent directories to check
					break;
				}
			} else {
				// reached the root directory, no parent directories left to check
				break;
			}
		}

		result
	};
	let Some(build_dir) = build_dir else {
		return Err(anyhow!("Failed to find build target directory"));
	};

	// folder containing source data we will create and zip
	let payload_dir = build_dir.join("Payload");

	// create payload directory
	std::fs::create_dir_all(&payload_dir).context("Failed to create package directory")?;

	// copy res files to payload directory
	for entry in std::fs::read_dir(res_dir).context("Failed to read res directory")? {
		let path = entry
			.context("Failed to read entry in res directory")?
			.path();

		if path.is_file() {
			let file_name = path.file_name().expect("Valid file must have a name");
			let dest_path = payload_dir.join(file_name);

			std::fs::copy(&path, &dest_path).context("Failed to copy file from res directory")?;
		}
	}

	// copy wasm file to package directory
	// note that this will just copy the first wasm file it finds in the build dir
	// if the cargo package name changes, this might copy the wrong (old) file
	let mut found_binary = false;
	for entry in std::fs::read_dir(build_dir).context("Failed to read build directory")? {
		let entry = entry.context("Failed to read entry in build directory")?;
		let path = entry.path();

		match path.extension() {
			Some(ext) if ext == "wasm" => {
				let dest_path = payload_dir.join("main.wasm");
				std::fs::copy(&path, &dest_path)
					.context("Failed to copy file from build directory")?;
				found_binary = true;
				break;
			}
			_ => {}
		}
	}
	if !found_binary {
		return Err(anyhow!("No wasm file found in build directory"));
	}

	// zip payload directory
	let zip_output = current_path.join("package.bunpak");
	create_zip(payload_dir.as_path(), zip_output.as_path())
		.context("Failed to compress payload directory")?;

	// remove package dir
	std::fs::remove_dir_all(payload_dir).context("Failed to remove payload directory")?;

	Ok(())
}

// zip a directory and output to a file
fn create_zip(src_dir: &std::path::Path, dst_file: &std::path::Path) -> std::io::Result<()> {
	let file = std::fs::File::create(dst_file)?;
	let mut zip = zip::ZipWriter::new(file);
	let options = zip::write::SimpleFileOptions::default()
		.compression_method(zip::CompressionMethod::Deflated)
		.unix_permissions(0o755);

	let dir_name = src_dir
		.file_name()
		.and_then(|n| n.to_str())
		.unwrap_or("Payload");

	for entry in std::fs::read_dir(src_dir)? {
		let entry = entry?;
		let path = entry.path();

		if path.is_file() {
			// let relative_path = path.strip_prefix(src_dir_str).unwrap().to_str().unwrap();
			let Some(file_name) = path.file_name().and_then(|n| n.to_str()) else {
				continue;
			};
			let zip_path = format!("{}/{}", dir_name, file_name);

			zip.start_file(zip_path, options)?;
			let mut f = std::fs::File::open(path)?;
			let mut buffer = Vec::new();
			f.read_to_end(&mut buffer)?;
			zip.write_all(&buffer)?;
		}
	}

	zip.finish()?;
	Ok(())
}
