#[macro_use]
extern crate clap;

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = clap::App::from_yaml(yaml).get_matches();
}
