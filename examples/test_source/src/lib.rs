#![no_std]
use aidoku::{
    error::Result,
    prelude::*,
    std::Vec,
    Filter, // std::net::{HttpMethod, Request},
    Manga,
};

#[manga_list_request]
fn get_manga_list(_: Vec<Filter>, page: i32) -> Result<Vec<Manga>> {
    let mut manga = Vec::new();
    Ok(manga)
}
