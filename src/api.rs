use crate::config::get_config;

// build reqwest client with token header
use reqwest::{Client, RequestBuilder};
use reqwest::header::HeaderMap;

pub fn get_client(path: String) -> Result<RequestBuilder, Box<dyn std::error::Error>> {
    let config = get_config()?;
    let request_path = format!("{}/{}", config.server, path);
    let mut header_map = HeaderMap::new();
    header_map.insert("Auth-Token", format!("{}", config.token).parse().unwrap());
    let client = Client::builder()
        .default_headers(
            header_map
        )
        .build()?;
    Ok(client.get(&request_path))
}

pub fn post_client(path: String, data: String) -> Result<RequestBuilder, Box<dyn std::error::Error>> {
    let config = get_config()?;
    let request_path = format!("{}/{}", config.server, path);
    let mut header_map = HeaderMap::new();
    header_map.insert("Auth-Token", format!("{}", config.token).parse().unwrap());
    let client = Client::builder()
        .default_headers(
            header_map
        )
        .build()?;
    Ok(client.get(&request_path)
        .body(data))
}

pub fn post_without_body_client(path: String) -> Result<RequestBuilder, Box<dyn std::error::Error>> {
    let config = get_config()?;
    let request_path = format!("{}/{}", config.server, path);
    let mut header_map = HeaderMap::new();
    header_map.insert("Auth-Token", format!("{}", config.token).parse().unwrap());
    let client = Client::builder()
        .default_headers(
            header_map
        )
        .build()?;
    Ok(client.get(&request_path))
}