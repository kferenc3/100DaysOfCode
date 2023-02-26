use std::fs::{self, OpenOptions};
use std::env;
use reqwest::header::COOKIE;
use crate::solutions2015::{
    day1,
    day2,
    day3,
    day4,
};

 #[derive(Debug)]
 pub struct Config {
     pub day: u32,
     pub year: u32,
     pub d_only: bool,
 }

 impl Config {
     pub fn build(mut args: impl Iterator<Item = String>,
 ) -> Result<Config, &'static str> {
     args.next();
    
    let day = match args.next() {
        Some(arg) => arg.parse::<u32>().unwrap(),
        None => return Err("Invalid argument"),
    };

    let year = match args.next() {
        Some(arg) => arg.parse::<u32>().unwrap(),
        _ => 2015,
    };

    let d_only = match args.next() {
        Some(arg) if arg == "-d" => true,
        _ => false,
    };

    if day > 25 && day < 1 {
        return Err("Day must be between 1 and 25")
    } else {
        Ok(Config {
            day,
            year,
            d_only,
        })
    }

}}

pub fn solver(year: u32, day: u32) -> Result<(String, String), &'static str> {
    let input = datagetter(year, day);

    match day {
        1 => Ok(day1::day1solver(&input)),
        2 => Ok(day2::day2solver(&input)),
        3 => Ok(day3::day3solver(&input)),
        4 => Ok(day4::day4solver(&input)),
        _ => return Err("Invalid day"),
    }
    
}

fn datagetter(year: u32, day: u32) -> String {
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