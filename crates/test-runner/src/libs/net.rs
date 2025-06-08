use reqwest::{header::HeaderMap, StatusCode};

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum HttpMethod {
	Get,
	Post,
	Put,
	Head,
	Delete,
}

#[derive(Debug)]
pub struct NetResponse {
	pub status: StatusCode,
	pub headers: HeaderMap,
	pub data: Vec<u8>,
}

#[derive(Debug)]
pub struct NetRequest {
	pub method: HttpMethod,
	pub url: Option<String>,
	pub headers: HeaderMap,
	pub body: Option<Vec<u8>>,
	pub response: Option<NetResponse>,
}

impl NetRequest {
	pub fn new(method: HttpMethod) -> Self {
		Self {
			method,
			url: None,
			headers: HeaderMap::new(),
			body: None,
			response: None,
		}
	}
}
