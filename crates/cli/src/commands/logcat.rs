use axum::{Router, body::Bytes, http::StatusCode, response::IntoResponse, routing::post};
use colored::*;
use std::net::SocketAddr;

pub async fn run(port: u16) -> anyhow::Result<()> {
	// post request handler for logs
	async fn handle_post(body: Bytes) -> impl IntoResponse {
		match std::str::from_utf8(&body) {
			Ok(text) => {
				// extract the date component of the log message
				let (date, log) = if let Some((first, rest)) = text.split_once("] ") {
					let date = format!("{}]", first); // add back the ]
					(Some(date), rest)
				} else {
					(None, text)
				};

				// color the log if it has a level
				let log = if log.contains("[ERROR]") {
					log.red()
				} else if log.contains("[WARN]") {
					log.yellow()
				} else if log.contains("[DEBUG]") {
					log.bright_black()
				} else {
					log.into()
				};

				// print the log
				if let Some(date) = date {
					println!("{} {}", date.bright_black(), log);
				} else {
					println!("{}", log);
				}

				StatusCode::OK
			}
			Err(_) => StatusCode::BAD_REQUEST,
		}
	}

	// open server on / listening for post requests
	let app = Router::new().route("/", post(handle_post));

	let addr = SocketAddr::from(([0, 0, 0, 0], port));

	let local_ip_address = local_ip_address::local_ip()
		.map(|ip| ip.to_string())
		.unwrap_or("localhost".into());

	println!("Listening for logs at http://{}:{}", local_ip_address, port);

	axum::serve(tokio::net::TcpListener::bind(addr).await?, app)
		.await
		.map_err(anyhow::Error::from)
}
