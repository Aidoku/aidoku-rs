use anyhow::Result;
use buny_cli::commands;
use buny_cli::models::SourceContentRating;
use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
	#[command(subcommand)]
	command: Command,
}

#[derive(Subcommand)]
enum Command {
	/// Build and package a source
	#[clap(alias = "pkg")]
	Package {
		/// Optional path to the source directory
		// If not provided, the current directory will be used
		path: Option<std::path::PathBuf>,
	},
	/// Build a source list
	Build {
		/// Paths to source packages
		files: Vec<std::path::PathBuf>,
		/// Output folder path
		#[arg(short, long, default_value = std::path::PathBuf::from("public").into_os_string())]
		output: std::path::PathBuf,
		/// Source list name
		#[arg(short, long)]
		name: Option<String>,
	},
	/// Initialize a new source
	Init {
		/// Optional path to the directory to initialize the source in
		// If not provided, the current directory will be used
		path: Option<std::path::PathBuf>,
		/// Source name
		#[arg(short, long)]
		name: Option<String>,
		/// Source homepage url
		#[arg(short, long)]
		url: Option<String>,
		/// Source languages
		#[arg(short, long)]
		languages: Vec<String>,
		/// Source content rating
		#[arg(short, long)]
		content_rating: Option<SourceContentRating>,
		/// Create a new source template
		#[arg(long, default_value_t = false)]
		template: bool,
		/// Template name, if creating a template
		#[arg(short, long)]
		template_name: Option<String>,
	},
	/// Open a server for log streaming
	Logcat {
		/// Port to listen on
		#[arg(short, long, default_value = "9000")]
		port: u16,
	},
	/// Build a source list and serve it on the local network
	Serve {
		/// Paths to source packages
		files: Vec<std::path::PathBuf>,
		/// Output folder path
		#[arg(short, long, default_value = std::path::PathBuf::from("public").into_os_string())]
		output: std::path::PathBuf,
		/// Port to serve on
		#[arg(short, long, default_value = "8080")]
		port: u16,
	},
	/// Verify a source is ready to be published
	Verify {
		/// Paths to source packages
		files: Vec<std::path::PathBuf>,
	},
}

#[tokio::main]
async fn main() -> Result<()> {
	let args = Cli::parse();

	match args.command {
		Command::Package { path } => commands::package::run(path)?,
		Command::Build {
			files,
			output,
			name,
		} => commands::build::run(files, &output, name)?,
		Command::Init {
			path,
			name,
			url,
			languages,
			content_rating,
			template,
			template_name,
		} => commands::init::run(
			path,
			name,
			url,
			languages,
			content_rating,
			template,
			template_name,
		)?,
		Command::Logcat { port } => commands::logcat::run(port).await?,
		Command::Serve {
			files,
			output,
			port,
		} => commands::serve::run(files, &output, port).await?,
		Command::Verify { files } => commands::verify::run(files)?,
	}

	Ok(())
}
