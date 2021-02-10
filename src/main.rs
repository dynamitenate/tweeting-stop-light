mod keys;
mod request;
mod response;

use keys::Keys;
use reqwest::blocking::Client;
use std::time::{Duration, Instant};
use std::thread::sleep;
use log4rs;
use log::{info, error};
use rust_gpiozero::LED;
use std::collections::HashMap;
use response::{LightColor, TwitterMention};

fn main() {
    // Start logging from config file
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    info!("Starting up...");

    // Create request client
    info!("Creating TLS client...");
    let client = request::get_tls_client(None);

    // Retrieve keys from JSON file
    info!("Retrieving keys from file...");
    let keys = keys::get_keys_from_file("keys.json").unwrap();

    // Get LEDs
    let mut leds = HashMap::new();
    leds.insert(LightColor::Red, LED::new(26));
    leds.insert(LightColor::Yellow, LED::new(20));
    leds.insert(LightColor::Green, LED::new(21));

    // Get initial tweet
    info!("Retrieving initial tweet from file...");
    let initial_tweet = response::get_mention_from_file().clone();
    if !initial_tweet.is_none() {
        let tweet = initial_tweet.clone().unwrap();
        let light_color = &tweet.light_color.unwrap();
        info!("Setting initial tweet from >>> ({:?}) @{}: #{:?}", &tweet.tweet_date, &tweet.from_user, light_color);
        update_stoplight(&light_color, &leds);
    }

    // Start request loop
    if !client.is_none() {
        request_loop(&client.unwrap(), &keys, &leds, initial_tweet.clone());
    }
}

fn request_loop(client: &Client, keys: &Keys, leds: &HashMap<LightColor, LED>, initial_tweet: Option<TwitterMention>) {
    // Most recent valid tweet id
    let mut most_recent_tweet_id = if initial_tweet.is_none() { None } else { Some(initial_tweet.unwrap().tweet_id.to_string()) };

    // Request loop
    loop {

        // Make request
        let new_tweet = request_iteration(&client, &keys, &most_recent_tweet_id);

        // Actions if there is a new tweet with valid hashtags
        if !new_tweet.is_none() {
            let tweet = new_tweet.unwrap();
            let light_color = tweet.light_color.unwrap();
            most_recent_tweet_id = Some(tweet.tweet_id.to_string());
            update_stoplight(&light_color, leds);
            response::send_mention_to_file(&tweet);
        }

        // Wait 15s
        sleep(Duration::new(15, 0));
    }
}

fn request_iteration(client: &Client, keys: &Keys, since_id: &Option<String>) -> Option<TwitterMention> {
    let request_time = Instant::now();
    let response = request::send_request(&client, &keys, &since_id);
    if !response.is_none() {
        let json = response::response_to_json(&response.unwrap());
        if !json.is_none() {
            let new_twitter_mention = response::get_mention_from_json(json.unwrap());
            if !new_twitter_mention.is_none(){
                let mention = new_twitter_mention.unwrap();
                let user = &mention.from_user;
                if !&mention.light_color.is_none() {
                    let light_color = &mention.light_color.unwrap();
                    let time = request_time.elapsed();
                    info!("({:?}) @{}: #{:?}", &time, &user, &light_color);
                    return Some(mention);
                } else {
                    let time = request_time.elapsed();
                    info!("({:?}) @{}: No valid hashtags!", &time, &user);
                    return None;
                }
            } else {
                let time = request_time.elapsed();
                info!("({:?}) {}", &time, "No new tweets!");
                return None;
            }
        } else {
            let time = request_time.elapsed();
            error!("({:?}) {}", &time, "Could not parse JSON!");
            return None;
        }
    } else {
        let time = request_time.elapsed();
        error!("({:?}) {}", &time, "Could not make the request!");
        return None;
    }
}

fn update_stoplight(light_color: &LightColor, leds: &HashMap<LightColor, LED>) {
    &leds[&LightColor::Red].off();
    &leds[&LightColor::Yellow].off();
    &leds[&LightColor::Green].off();
    &leds[light_color].on();
}