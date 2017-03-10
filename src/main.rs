#![feature(slice_patterns)]

#[macro_use]
extern crate clap;

pub mod parse;
pub mod range;
pub mod repr;

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = clap::App::from_yaml(yaml).get_matches();
}
