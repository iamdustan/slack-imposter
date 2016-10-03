#[macro_use]
extern crate clap;
extern crate curl;
extern crate rustc_serialize;
use std::io::Read;
use rustc_serialize::json;
use clap::{Arg, App};
use curl::easy::Easy;

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct SlackDataStruct  {
    text: String,
    icon_url: String,
    username: String,
    channel: String,
}

fn main() {
    let matches = App::new("Impost Your Slack Colleagues")
        .version("1.0")
        .author("Dustan Kasten <dustan.kasten@gmail.com>")
        .about("Does awesome things")
        .arg(Arg::with_name("config")
             .short("z")
             .long("config")
             .value_name("FILE")
             .help("Sets a custom config file")
             .takes_value(true))
        .arg(Arg::with_name("as")
             .short("a")
             .long("as")
             .value_name("as")
             .required(true)
             .help("Who should we impost to be?")
             .takes_value(true))
        .arg(Arg::with_name("text")
             .short("t")
             .long("text")
             .value_name("text")
             .required(true)
             .help("What do you want them to say?")
             .takes_value(true))
        .arg(Arg::with_name("channel")
             .short("c")
             .long("channel")
             .value_name("channel")
             .help("What channel should they post in?")
             .takes_value(true))
        .arg(Arg::with_name("v")
             .short("v")
             .multiple(true)
             .help("Sets the level of verbosity"))
        .get_matches();

    let yaml = load_yaml!("/Users/dkasten/impost.yaml");
    let verbosity = matches.occurrences_of("v");

    let who = matches.value_of("as").unwrap();
    let text = matches.value_of("text").unwrap();
    let channel = matches.value_of("channel").unwrap_or(&yaml["channel"].as_str().unwrap());

    let url = &yaml["boturl"].as_str().unwrap();
    let username = &yaml[who]["username"].as_str().unwrap();
    let avatar = &yaml[who]["avatar"].as_str().unwrap();

    let body = SlackDataStruct {
        text: text.to_string(),
        icon_url: avatar.to_string(),
        username: username.to_string(),
        channel: channel.to_string(),
    };

    if verbosity > 0 { println!("Imposting to be {:?}", body) }
    let encoded = json::encode(&body).unwrap();
    let mut data = encoded.as_bytes();

    let mut easy = Easy::new();
    easy.url(url).unwrap();
    easy.post(true).unwrap();
    easy.post_field_size(data.len() as u64).unwrap();
    let mut transfer = easy.transfer();
    transfer.read_function(|buf| {
        Ok(data.read(buf).unwrap_or(0))
    }).unwrap();
    transfer.perform().unwrap();

    if verbosity > 0 { println!("Imposted 👹") }
}
