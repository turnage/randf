#![feature(slice_patterns)]

#[macro_use]
extern crate clap;

pub mod range;

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = clap::App::from_yaml(yaml).get_matches();
}
