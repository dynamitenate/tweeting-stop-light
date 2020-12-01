extern crate oauth1;
extern crate reqwest;
extern crate serde_json;
extern crate chrono;
extern crate serde;

use oauth1::Token;
use reqwest::Client;
use reqwest::header::Authorization;
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::borrow::Cow;
use std::str::FromStr;
use std::time::{Duration, Instant};
use std::thread::sleep;
use chrono::Local;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;
use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Clone, Copy)]
enum LightColor {
    Green,
    Yellow,
    Red
}

impl FromStr for LightColor {
    type Err = ();
    fn from_str(input: &str) -> Result<LightColor, Self::Err> {
        match &input.to_lowercase()[..] {
            "green" => Ok(LightColor::Green),
            "yellow" => Ok(LightColor::Yellow),
            "red" => Ok(LightColor::Red),
            _ => Err(())
        }
    }
}

#[derive(Debug)]
struct TwitterMention {
    tweet_id: u64,
    tweet_date: String,
    light_color: Option<LightColor>
}

#[derive(Deserialize, Debug)]
struct Keys {
    api_key: String,
    api_secret_key: String,
    user_key: String,
    user_secret_key: String
}

fn main() {
    request_loop()
}

fn request_loop() {
    // repetitive stuff
    let client = Client::new();
    let keys = get_keys_from_file("keys.json").unwrap();
    let user_key = &keys.user_key;
    let user_secret_key = &keys.user_secret_key;
    let api_key = &keys.api_key;
    let api_secret_key = &keys.api_secret_key;

    // actual loop
    let mut request_time;
    let mut request_number = 1;
    loop {
        request_time = Instant::now();
        let response = send_request(&client, api_key, api_secret_key, user_key, user_secret_key);
        let json = response_to_json(&response[..]);
        if !json.is_none() {
            let new_twitter_mention = get_mention_from_json(json.unwrap());
            if !new_twitter_mention.is_none(){
                println!("[Time: {} | Request Number: {} | Request Duration: {:?}] {:?}", Local::now(), request_number, request_time.elapsed(), new_twitter_mention.unwrap().light_color);
            } else {
                println!("[Time: {} | Request Number: {} | Request Duration: {:?}] No new tweets!", Local::now(), request_number, request_time.elapsed());
            }
        } else {
            println!("[Time: {} | Request Number: {} | Request Duration: {:?}] Could not parse JSON!", Local::now(), request_number, request_time.elapsed());
        }
        sleep(Duration::new(15, 0));
        request_number += 1;
    }
}

fn get_keys_from_file<P: AsRef<Path>>(path: P) -> Result<Keys, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let keys = serde_json::from_reader(reader)?;
    return Ok(keys);
}

fn get_light_color(hashtags: &Vec<&str>) -> Option<LightColor> {
    let valid_hashtags: Vec<LightColor> = hashtags
        .iter()
        .cloned()
        .map(|h| {
            let light_color = LightColor::from_str(h);
            if light_color.is_ok() {
                return Some(light_color.unwrap());
            }
            return None;
        })
        .filter(|h| !h.is_none())
        .map(|h| h.unwrap().clone())
        .collect();
    let first_hashtag = valid_hashtags.first();
    if first_hashtag.is_none() {
        return None;
    }
    return Some(first_hashtag.unwrap().clone());
}

fn get_mention_from_json(json: JsonValue) -> Option<TwitterMention> {
    let has_new_tweets = json.is_array() && json.as_array().unwrap().len() != 0;
    if has_new_tweets {
        let hashtags: Vec<&str> = json[0]["entities"]["hashtags"].as_array().unwrap()
            .iter()
            .map(|h| h["text"].as_str().unwrap())
            .collect();
        return Some(TwitterMention {
            tweet_id: json[0]["id"].as_u64().unwrap(),
            tweet_date: String::from(json[0]["created_at"].as_str().unwrap()),
            light_color: get_light_color(&hashtags)
        })
    }
    return None;
}

fn response_to_json(response: &str) -> Option<JsonValue> {
    let json: serde_json::Result<JsonValue> = serde_json::from_str(response);
    if json.is_ok() {
        return Some(json.unwrap());
    }
    return None;
}

fn send_request(
    client: &Client,
    api_key: &str,
    api_secret_key: &str,
    user_key: &str,
    user_secret_key: &str
) -> String {
    let url = "https://api.twitter.com/1.1/statuses/mentions_timeline.json";
    let mut params_map: HashMap<&str, Cow<'_, str>> = HashMap::new();
    params_map.insert("count", Cow::from("1"));
    params_map.insert("trim_user", Cow::from("1"));
    let response_text = client.get(url)
        .header(Authorization(oauth1::authorize(
            "GET",
            url,
            &Token::new(api_key, api_secret_key),
            Some(&Token::new(user_key, user_secret_key)),
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