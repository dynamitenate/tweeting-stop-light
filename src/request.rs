use super::keys::Keys;
use oauth1::Token;
use reqwest::blocking::Client;
use reqwest::header::{AUTHORIZATION};
use std::collections::HashMap;
use std::borrow::Cow;

pub fn send_request(client: &Client, keys: &Keys) -> Option<String> {
    let url = "https://api.twitter.com/1.1/statuses/mentions_timeline.json";
    let mut params_map: HashMap<&str, Cow<'_, str>> = HashMap::new();
    params_map.insert("count", Cow::from("1"));
    params_map.insert("trim_user", Cow::from("1"));
    let request = client.get(url)
        .header(AUTHORIZATION, oauth1::authorize(
            "GET",
            url,
            &Token::new(&keys.api_key, &keys.api_secret_key),
            Some(&Token::new(&keys.user_key, &keys.user_secret_key)),
            Some(params_map),
        ))
        .query(&[
            ("count", "1"),
            ("trim_user", "1")
        ]);
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