use serde_json::Value as JsonValue;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug)]
pub struct TwitterMention {
    pub tweet_id: u64,
    pub tweet_date: String,
    pub light_color: Option<LightColor>
}

pub fn response_to_json(response: &str) -> Option<JsonValue> {
    let json: serde_json::Result<JsonValue> = serde_json::from_str(response);
    if json.is_ok() {
        return Some(json.unwrap());
    }
    return None;
}

pub fn get_mention_from_json(json: JsonValue) -> Option<TwitterMention> {
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