#[macro_use]
extern crate serde;

use std::fs::File;
use std::fs;
use std::io::Write;

mod toml_parse;
use toml_parse::*;

fn main() {
    let toml_str = fs::read_to_string("C:/Users/bad wife/Documents/Egosoft/X4/77065308/logs/print_universe/print_universe.txt").expect("reading file");
    let toml_parsed: Toml = toml::from_str(&toml_str).expect("parsing toml");
    let json_str = serde_json::to_string(&toml_parsed).expect("converting to json");
    let mut buffer = File::create("output/print_universe.json").unwrap();
    buffer.write_all(json_str.as_bytes()).unwrap();
}
