extern crate slack;
#[macro_use] extern crate lazy_static;
extern crate regex;
extern crate multimap;
extern crate crypto;

mod types;
mod dictionary;
mod logic;

struct NiancatHandler;

impl slack::EventHandler for NiancatHandler {
    fn on_event(&mut self,
                _client: &mut slack::RtmClient,
                event: Result<&slack::Event, slack::Error>,
                raw_json: &str) {
        println!("on_event(event: {:?}, raw_json: {:?}", event, raw_json);
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

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let api_key = match args.len() {
        0 | 1 => panic!("No API key in args!"),
        x => {
            args[x - 1].clone()
        }
    };

    let mut handler = NiancatHandler;
    let mut client = slack::RtmClient::new(&api_key);
    let r = client.login_and_run::<NiancatHandler>(&mut handler);
    match r {
        Ok(_) => {}
        Err(err) => panic!("Error: {}", err),
    }
    println!("{}", client.get_name().unwrap());
    println!("{}", client.get_team().unwrap().name);
}
