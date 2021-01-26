use std::io::BufReader;
use std::fs::File;
use std::path::Path;
use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize, Debug)]
pub struct Keys {
    pub api_key: String,
    pub api_secret_key: String,
    pub user_key: String,
    pub user_secret_key: String
}

pub fn get_keys_from_file<P: AsRef<Path>>(path: P) -> Result<Keys, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let keys = serde_json::from_reader(reader)?;
    return Ok(keys);
}