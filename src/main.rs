extern crate oauth1;
extern crate reqwest;
extern crate chrono;
extern crate serde;

mod request;
mod keys;
mod response;

use reqwest::Client;
use std::time::{Duration, Instant};
use std::thread::sleep;
use chrono::Local;

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
        let json = response::response_to_json(&response[..]);
        if !json.is_none() {
            let new_twitter_mention = response::get_mention_from_json(json.unwrap());
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