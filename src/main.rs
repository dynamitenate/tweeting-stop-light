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
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    info!("Starting up...");
    request_loop();
}

fn request_loop() {
    // repetitive stuff
    let client = Client::new();
    info!("Retrieving keys from file...");
    let keys = keys::get_keys_from_file("keys.json").unwrap();

    // actual loop
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
                let message = format!("{:?}", &new_twitter_mention.unwrap().light_color);
                info!("{}: {}", format!("{:?}", request_time.elapsed()), &message);
            } else {
                info!("{}: {}", format!("{:?}", request_time.elapsed()), "No new tweets!");
            }
        } else {
            error!("{}: {}", format!("{:?}", request_time.elapsed()), "Could not parse JSON!");
        }
    } else {
        error!("{}: {}", format!("{:?}", request_time.elapsed()), "Could make the request!");
    }
}