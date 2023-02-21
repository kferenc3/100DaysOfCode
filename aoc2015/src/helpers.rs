use std::fs::{self, OpenOptions};
use std::env;
use reqwest::header::COOKIE;

#[derive(Debug)]
pub struct Config {
    year: u32,
    day: u32,
    d_only: String,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>,
) -> Result<Config, &'static str> {
    args.next();

    let year = match args.next() {
        Some(arg) => arg.parse::<u32>().unwrap(),
        None => return Err("Invalid arguments"),
    };
    
    let day = match args.next() {
        Some(arg) => arg.parse::<u32>().unwrap(),
        None => return Err("Invalid argument"),
    };

    let d_only = match args.next() {
        Some(arg) => arg,
        None => return Err("Invalid argument"),
    };

    Ok(Config {
        year,
        day,
        d_only,
    })

}}
    
pub fn datagetter(year: u32, day: u32) -> String {
    let fname = "inputdata_day".to_string() + &day.to_string();
    let url = "https://adventofcode.com/".to_string() + &year.to_string() + "/day/" + &day.to_string() + "/input";
    
    
    match OpenOptions::new().read(true).open(&fname) {
        Ok(_) => return fs::read_to_string(fname).expect("Unable to read file"),
        Err(_) => return get_request(&url, &fname),
    };
}

fn get_request(u: &str, f: &str) -> String {
    let client = reqwest::blocking::Client::new();
    let cookie = env::var("AOC_SESSION").expect("No session cookie named AOC_SESSION found in environment");

    let content =  client
        .get(u)
        .header(COOKIE, cookie)
        .send()
        .expect("Couldn't get response. Maybe the input is not available yet.")
        .text()
        .unwrap();

    match OpenOptions::new().write(true).create_new(true).open(f) {
        Ok(_) => fs::write(f, content).expect("Unable to write to file"),
        Err(_) => println!("Error!"),
    }

    return fs::read_to_string(f).expect("Unable to read newly created file")
}