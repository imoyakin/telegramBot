use std::fs;
use regex::Regex;
use serde::{Serialize, Deserialize};

lazy_static!{
    pub static ref CONF:Config = read_file("config.json".to_string());
    pub static ref RE:Regex = {
        let mut re = r"(?x)\n".to_string();
        for i in &CONF.auto_resp {
            re = re + &format!(r"{}{}{}{}{}{}", "(", i.key, ")","\n", "-","\n");
        }
        println!("{}", re);
        Regex::new(&re).unwrap()
    };
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub auto_resp: Vec<AtuoResp>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AtuoResp {
    pub key: String,
    pub value: Vec<String>,
}

fn read_file(filename:String) -> Config {
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    serde_json::from_str(&contents).unwrap()
}