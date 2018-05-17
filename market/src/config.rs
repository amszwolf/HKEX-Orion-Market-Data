use toml;

use std::fs::File;
use std::io::prelude::*;

#[derive(Deserialize)]
#[derive(Debug)]
pub struct IpConfig {
    pub name: Option<String>,
    pub ip: Option<String>,
    pub port: Option<String>,
}

#[derive(Deserialize)]
#[derive(Debug)]
pub struct Conf
{
    pub ip_config: Option<Vec<IpConfig>>
}

pub fn load_ip_config() -> Conf {
    let file_path = "./etc/config.toml";
    let mut file = match File::open(file_path) {
        Ok(f) => f,
        Err(e) => panic!("no such file {} exception:{}", file_path, e)
    };
    let mut str_val = String::new();
    match file.read_to_string(&mut str_val) {
        Ok(s) => s
        ,
        Err(e) => panic!("Error Reading file: {}", e)
    };
    let config: Conf = toml::from_str(&str_val).unwrap();

    // for x in config.ip_config.unwrap() {
    //     println!("{:?}", x);
    // }
    return config;
}

