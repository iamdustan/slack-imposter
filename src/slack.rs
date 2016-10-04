extern crate curl;
extern crate rustc_serialize;

use std::io::Read;
use rustc_serialize::json;
use curl::easy::Easy;

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct SlackDataStruct  {
    pub text: String,
    pub icon_url: String,
    pub username: String,
    pub channel: String,
}

pub fn send(url: &str, body: &SlackDataStruct) -> u8 {
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
    0
}

