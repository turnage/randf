#![feature(slice_patterns, exclusive_range_pattern)]

#[macro_use]
extern crate clap;
extern crate meval;
extern crate regex;

pub mod constraints;
pub mod id;
pub mod parse;
pub mod range;
pub mod relop;
pub mod repr;
pub mod spec;

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = clap::App::from_yaml(yaml).get_matches();
}
