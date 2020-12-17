extern crate oauth1;
extern crate reqwest;
extern crate serde_json;
extern crate chrono;
extern crate serde;

mod request;
mod keys;

use reqwest::Client;
use serde_json::Value as JsonValue;
use std::str::FromStr;
use std::time::{Duration, Instant};
use std::thread::sleep;
use chrono::Local;
use serde::Deserialize;

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
    let keys = keys::get_keys_from_file("keys.json").unwrap();

    // actual loop
    let mut request_time;
    let mut request_number = 1;
    loop {
        request_time = Instant::now();
        let response = request::send_request(&client, &keys);
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