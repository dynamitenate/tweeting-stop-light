mod keys;
mod request;
mod response;

use keys::Keys;
use reqwest::blocking::Client;
use std::time::{Duration, Instant};
use std::thread::sleep;
use log4rs;
use log::{info, error};

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

    // Start request loop
    if !client.is_none() {
        request_loop(&client.unwrap(), &keys);
    }
}

fn request_loop(client: &Client, keys: &Keys) {
    loop {
        request_iteration(&client, &keys);
        sleep(Duration::new(15, 0));
    }
}

fn request_iteration(client: &Client, keys: &Keys) {
    let request_time = Instant::now();
    let response = request::send_request(&client, &keys);
    if !response.is_none() {
        let json = response::response_to_json(&response.unwrap());
        if !json.is_none() {
            let new_twitter_mention = response::get_mention_from_json(json.unwrap());
            if !new_twitter_mention.is_none(){
                let mention = &new_twitter_mention.unwrap();
                let user = &mention.from_user;
                if !&mention.light_color.is_none() {
                    let message = &mention.light_color.unwrap();
                    let time = request_time.elapsed();
                    info!("({:?}) @{}: #{:?}", &time, &user, &message);
                } else {
                    let time = request_time.elapsed();
                    info!("({:?}) @{}: No valid hashtags!", &time, &user);
                }
            } else {
                let time = request_time.elapsed();
                info!("({:?}) {}", &time, "No new tweets!");
            }
        } else {
            let time = request_time.elapsed();
            error!("({:?}) {}", &time, "Could not parse JSON!");
        }
    } else {
        let time = request_time.elapsed();
        error!("({:?}) {}", &time, "Could make the request!");
    }
}