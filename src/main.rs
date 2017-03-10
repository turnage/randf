#![feature(slice_patterns, exclusive_range_pattern)]

#[macro_use]
extern crate clap;

pub mod id;
pub mod parse;
pub mod range;
pub mod repr;
pub mod spec;

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = clap::App::from_yaml(yaml).get_matches();
}
