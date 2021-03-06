#[macro_use]
extern crate clap;
extern crate curl;
extern crate rustc_serialize;
use clap::{App};

mod slack;

fn main() {
    let yaml = load_yaml!("config.yaml");
    let matches = App::from_yaml(yaml).get_matches();

    let yaml = load_yaml!("../impost.yaml");
    let verbosity = matches.occurrences_of("v");

    let who = matches.value_of("as").unwrap();
    let text = matches.value_of("text").unwrap();
    let channel = matches.value_of("channel").unwrap_or(&yaml["channel"].as_str().unwrap());

    let url = &yaml["boturl"].as_str().unwrap();
    let username = &yaml[who]["username"].as_str().unwrap();
    let avatar = &yaml[who]["avatar"].as_str().unwrap();

    let imposter = slack::SlackImposter::new(text, avatar, username, channel);

    if verbosity > 0 { println!("Imposting to be {:?}", imposter) }
    imposter.send(&url);
    if verbosity > 0 { println!("Imposted 👹") }
}
