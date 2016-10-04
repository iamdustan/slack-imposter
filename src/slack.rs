extern crate curl;
extern crate rustc_serialize;

use std::io::Read;
use rustc_serialize::json;
use curl::easy::Easy;

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct SlackImposter  {
    text: String,
    icon_url: String,
    username: String,
    channel: String,
}

impl SlackImposter {
    pub fn new (text: &str, icon_url: &str, username: &str, channel: &str) -> SlackImposter {
        SlackImposter {
            text: text.to_string(),
            icon_url: icon_url.to_string(),
            username: username.to_string(),
            channel: channel.to_string(),
        }
    }

    pub fn send(self, url: &str) {
        let encoded = json::encode(&self).unwrap();
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
    }
}

