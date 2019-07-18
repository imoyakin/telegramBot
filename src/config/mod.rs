use std::fs;
use regex::RegexSet;
use serde::{Serialize, Deserialize};

lazy_static!{
    pub static ref CONF:Config = {
        let mut conf = read_file("config.json".to_string());
        for i in &mut conf.auto_resp {
            i.num = i.value.len();
        }
        return conf
    };
    pub static ref RE:RegexSet = {
        let mut re = vec![];
        for i in &CONF.auto_resp {
            re.push(format!(r#"{}"#, i.key))
        }
        RegexSet::new(&re).unwrap()
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

    #[serde(skip_deserializing)]
    pub num:usize,
}

fn read_file(filename:String) -> Config {
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    serde_json::from_str(&contents).unwrap()
}