use serde_json::Value as JsonValue;
use std::str::FromStr;
use log::{debug, trace, error};
use serde::{Serialize, Deserialize};
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::fs::File;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LightColor {
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

#[derive(Debug, Serialize, Deserialize)]
pub struct TwitterMention {
    pub tweet_id: u64,
    pub tweet_date: String,
    pub from_user: String,
    pub light_color: Option<LightColor>
}

pub fn get_mention_from_file() -> Option<TwitterMention> {
    let file = File::open("most_recent_tweet.json");
    if file.is_ok() {
        let mut contents = String::new();
        let result = file.unwrap().read_to_string(&mut contents);
        if result.is_ok() {
            let content: std::result::Result<TwitterMention, serde_json::Error> = serde_json::from_str(&contents);
            if content.is_ok() {
                let deserialized = content.unwrap();
                trace!("Retrieved twitter mention from file:\n{:#?}", deserialized);
                return Some(deserialized);
            } else {
                error!("Could not deserialize most recent tweet!");
            }
        } else {
            error!("Could not read serialized tweet from file!");
        }
    } else {
        error!("Could not open file to read most recent tweet from file!");
    }
    return None;
}

pub fn send_mention_to_file(mention: &TwitterMention) {
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("most_recent_tweet.json");
    if file.is_ok() {
        let content = serde_json::to_string_pretty(&mention);
        if content.is_ok() {
            let serialized = content.unwrap();
            let result = file.unwrap().write_all(serialized.as_bytes());
            if result.is_ok() {
                trace!("Saved twitter mention to file:\n{:#?}", serialized);
            } else {
                error!("Could not write serialized tweet to file!");
            }
        } else {
            error!("Could not serialize most recent tweet!");
        }
    } else {
        error!("Could not open file to write most recent tweet to file!");
    }
}

pub fn response_to_json(response: &str) -> Option<JsonValue> {
    let json: serde_json::Result<JsonValue> = serde_json::from_str(response);
    if json.is_ok() {
        trace!("{:#?}", json);
        return Some(json.unwrap());
    }
    return None;
}

pub fn get_mention_from_json(json: JsonValue) -> Option<TwitterMention> {
    let has_new_tweets = json.is_array() && json.as_array().unwrap().len() != 0;
    if has_new_tweets {
        debug!("There are new tweets from response.");
        let hashtags: Vec<&str> = json[0]["entities"]["hashtags"].as_array().unwrap()
            .iter()
            .map(|h| h["text"].as_str().unwrap())
            .collect();
        return Some(TwitterMention {
            tweet_id: json[0]["id"].as_u64().unwrap(),
            tweet_date: String::from(json[0]["created_at"].as_str().unwrap()),
            from_user: String::from(json[0]["user"]["screen_name"].as_str().unwrap()),
            light_color: get_light_color(&hashtags)
        })
    }
    debug!("No new tweets from response.");
    return None;
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
        debug!("No valid hashtags in response.");
        return None;
    }
    debug!("First valid hashtag is \"{:?}\".", first_hashtag.unwrap());
    return Some(first_hashtag.unwrap().clone());
}