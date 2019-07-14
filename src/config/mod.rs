use std::fs;
use serde::{Serialize, Deserialize};

lazy_static!{
    pub static ref conf:config = read_file("config.json".to_string());
}

#[derive(Serialize, Deserialize, Debug)]
pub struct config {
    pub AutoResp: Vec<atuo_resp>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct atuo_resp {
    pub Key: String,
    pub Value: Vec<String>,
}

fn read_file(filename:String) -> config {
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    serde_json::from_str(&contents).unwrap()
}