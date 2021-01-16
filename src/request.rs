use super::keys::Keys;
use oauth1::Token;
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use std::collections::HashMap;
use std::borrow::Cow;

fn get_oauth_header(method: &str, url: &str, keys: &Keys, params: &[(&str, &str)]) -> HeaderMap {
    let mut params_map: HashMap<&str, Cow<'_, str>> = HashMap::new();
    for (key, value) in params {
        params_map.insert(key, Cow::from(&value[..]));
    }
    let oauth = oauth1::authorize(
        method,
        url,
        &Token::new(&keys.api_key, &keys.api_secret_key),
        Some(&Token::new(&keys.user_key, &keys.user_secret_key)),
        Some(params_map),
    );
    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, HeaderValue::from_str(&oauth).unwrap());
    return headers;
}

pub fn get_tls_client() -> Option<Client> {
    let client_builder = reqwest::blocking::Client::builder()
        .use_rustls_tls();
    let client = match client_builder.build() {
        Ok(client) => Some(client),
        Err(_error) => None
    };
    return client;
}

pub fn send_request(client: &Client, keys: &Keys) -> Option<String> {
    let url = "https://api.twitter.com/1.1/statuses/mentions_timeline.json";
    let params = &[
        ("count", "1"),
        ("trim_user", "1")
    ];
    let header = get_oauth_header("GET", url, &keys, params);
    let request = client.get(url)
        .headers(header)
        .query(params);
    let response_text = match request.send() {
        Ok(response) => {
            match response.text() {
                Ok(text) => Some(text),
                Err(_error) => {
                    None
                }
            }
        },
        Err(_error) => {
            None
        }
    };
    return response_text;
}