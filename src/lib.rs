extern crate slack;
#[macro_use] extern crate lazy_static;
extern crate regex;
extern crate multimap;
extern crate crypto;

pub mod types;
pub mod dictionary;
mod logic;
mod parser;

pub struct NiancatHandler;

impl slack::EventHandler for NiancatHandler {
    fn on_event(&mut self,
                _client: &mut slack::RtmClient,
                event: Result<&slack::Event, slack::Error>,
                raw_json: &str) {
        match event {
            Ok(ok_event) => println!("on_event(event: {:?}, raw_json: {:?}", ok_event, raw_json),
            Err(bad_event) => println!("on_event(bad event: {:?}, raw_json: {:?}", bad_event, raw_json)
        }
    }

    fn on_ping(&mut self, _client: &mut slack::RtmClient) {
        println!("on_ping");
    }

    fn on_close(&mut self, _client: &mut slack::RtmClient) {
        println!("on_close");
    }

    fn on_connect(&mut self, _client: &mut slack::RtmClient) {
        println!("on_connect");
    }
}
