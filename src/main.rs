extern crate oauth1;
extern crate reqwest;
extern crate chrono;
extern crate serde;

mod keys;
mod request;
mod response;
mod logger;

use reqwest::Client;
use std::time::{Duration, Instant};
use std::thread::sleep;

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
        let json = response::response_to_json(&response);
        if !json.is_none() {
            let new_twitter_mention = response::get_mention_from_json(json.unwrap());
            if !new_twitter_mention.is_none(){
                let message = format!("{:?}", &new_twitter_mention.unwrap().light_color);
                logger::log_request(request_number, request_time.elapsed(), &message);
            } else {
                logger::log_request(request_number, request_time.elapsed(), "No new tweets!");
            }
        } else {
            logger::log_request(request_number, request_time.elapsed(), "Could not parse JSON!");
        }
        sleep(Duration::new(15, 0));
        request_number += 1;
    }
}