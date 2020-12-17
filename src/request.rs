use super::keys::Keys;

use oauth1::Token;
use reqwest::Client;
use reqwest::header::Authorization;
use std::collections::HashMap;
use std::borrow::Cow;

pub fn send_request(client: &Client, keys: &Keys) -> String {
    let url = "https://api.twitter.com/1.1/statuses/mentions_timeline.json";
    let mut params_map: HashMap<&str, Cow<'_, str>> = HashMap::new();
    params_map.insert("count", Cow::from("1"));
    params_map.insert("trim_user", Cow::from("1"));
    let response_text = client.get(url)
        .header(Authorization(oauth1::authorize(
            "GET",
            url,
            &Token::new(&keys.api_key, &keys.api_secret_key),
            Some(&Token::new(&keys.user_key, &keys.user_secret_key)),
            Some(params_map),
        )))
        .query(&[
            ("count", "1"),
            ("trim_user", "1")
        ])
        .send()
        .expect("Could not make the request!")
        .text()
        .expect("Could not read response text!");
    return response_text;
}