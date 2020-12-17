use std::time::Duration;
use chrono::Local;

pub fn log_request(count: i32, duration: Duration, message: &str) {
    let time = Local::now().to_rfc2822().to_string();
    let formatted_duration = format!("{:?}", duration);
    println!("Request Count: {:<10} | Time: {:<35} | Duration: {:<15} | Message: {}", count, time, formatted_duration, message);
}