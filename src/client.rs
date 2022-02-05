use reqwest::header::{HeaderMap, HeaderValue};

/// get client
pub fn new_client_with_authorization_header(access_token: &str) -> reqwest::Client {
    let mut header_map = HeaderMap::new();
    header_map.insert(
        "Authorization",
        HeaderValue::from_str(&format!("Bearer {}", access_token)).expect("impossible error"),
    );
    reqwest::Client::builder()
        .default_headers(header_map)
        .build()
        .expect("impossible error")
}
