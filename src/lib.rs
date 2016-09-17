extern crate slack;
#[macro_use] extern crate lazy_static;
extern crate regex;
extern crate multimap;
extern crate crypto;
extern crate hyper;

use std::collections::HashMap;

use slack::api::channels::ListResponse;
use slack::api;


pub mod types;
pub mod dictionary;
mod logic;
mod parser;
mod response;

use response::{Respond, new_responder, SlackResponse};
use types::{Channel, Name};

pub struct NiancatHandler<'a> {
    state: logic::Niancat<'a>,
    responder: Box<Respond>,
    users: HashMap<String, slack::User>,
}

impl<'a> NiancatHandler<'a> {
    pub fn new(dict: &'a dictionary::Dictionary, main_channel: types::Channel) -> NiancatHandler<'a> {
        NiancatHandler {
            state: logic::Niancat::new(dict),
            responder: new_responder(&main_channel),
            users: HashMap::new(),
        }
    }

    fn handle_command(&mut self,
                      client: &mut slack::RtmClient,
                      channel: &types::Channel,
                      name: &types::Name,
                      text: &String) {
        let command_result = parser::parse_command(channel, name, text);

        if let Some(result) = command_result {
            let slack_responses = match result {
                Ok(command) => {
                    let response_message = logic::apply(&command, &mut self.state);
                    self.responder.serialize(&response_message)
                },

                Err(invalid_command) => {
                    self.responder.serialize_invalid_command(&invalid_command)
                },
            };

            for SlackResponse(channel, msg) in slack_responses {
                let result = client.send_message(channel.0.as_str(), msg.as_str());
                if let Err(x) = result {
                    println!("Response was not sent! Reason: {:?}", x);
                }
            }
        }
    }

    fn handle_message(&mut self, client: &mut slack::RtmClient, message: &slack::Message) {
        match *message {
            slack::Message::Standard{ref channel, ref user, ref text, ..} => {
                if channel.is_none() {
                    println!("Message with no channel! text: {:?}", text);
                    return;
                }

                if user.is_none() {
                    println!("Message with no user! text: {:?}", user);
                    return;
                }

                if text.is_none() {
                    println!("Message with no text!");
                    return;
                }

                let user = user.clone().unwrap();
                let name = match self.users.get(&user) {
                    None => {
                        println!("Unknown user {:?}", user);
                        return;
                    },
                    Some(u) => u.name.clone(),
                };

                let channel = channel.clone().unwrap();
                let text = text.clone().unwrap();

                self.handle_command(client, &Channel(channel), &Name(name), &text);
            },

            _ => {},
        }
    }

    fn handle_event(&mut self, client: &mut slack::RtmClient, event: &slack::Event) {
        match *event {
            slack::Event::Message(ref m) => self.handle_message(client, m),
            slack::Event::UserChange {ref user} => self.update_user(user),
            slack::Event::TeamJoin{ref user} => self.update_user(user),
            _ => {},
        }
    }

    pub fn update_user(&mut self, user: &slack::User) {
        println!("Updating user {:?}", user);
        self.users.entry(user.id.clone()).or_insert(user.clone());
    }
}

//pub fn parse_command(chan: &Channel, name: &Name, text: &String) -> CommandResult {

impl<'a> slack::EventHandler for NiancatHandler<'a> {
    fn on_event(&mut self,
                client: &mut slack::RtmClient,
                event: Result<&slack::Event, slack::Error>,
                raw_json: &str) {
        match event {
            Ok(ok_event) => {
                println!("on_event(event: {:?}, raw_json: {:?}", ok_event, raw_json);
                self.handle_event(client, ok_event);
            },
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
        println!("Connected!");
    }
}

pub trait ListChannels {
    fn list_channels(&self) -> Result<ListResponse, api::Error>;
}

pub fn initialize<T: ListChannels>(c: &T, dictionary_path: &String, channel_name: &String) ->
    Result<(dictionary::Dictionary, types::Channel), String> {

    // List all channels and handle the response.
    let list_response = c.list_channels();
    let list_response = match list_response {
        Err(ref e) => return Err(format!("{:?}", e)),
        Ok(x) => x,
    };

    // Find the channel with the given name, if present.
    let channel = list_response.channels.iter().find(|&x| &x.name == channel_name);
    let channel_id: types::Channel = match channel {
        None => return Err(format!("No channel named {} found", channel_name)),
        Some(ref c) => types::Channel(c.id.clone()),
    };

    // Load the dictionary from a file.
    let dictionary = dictionary::Dictionary::from_file(dictionary_path);
    let dictionary = match dictionary {
        Ok(d) => d,
        Err(e) => return Err(format!("Could not load dictionary, reason: {}", e)),
    };

    Ok((dictionary, channel_id))
}

//
//
//

pub struct SlackListChannels {
    pub token: String,
}

impl ListChannels for SlackListChannels {
    fn list_channels(&self) -> Result<ListResponse, api::Error> {
        let client = hyper::Client::new();
        api::channels::list(&client, &self.token, Some(true))
    }
}

impl SlackListChannels {
    pub fn list_users(&self) -> Result<api::users::ListResponse, api::Error> {
        let client = hyper::Client::new();
        api::users::list(&client, &self.token, Some(false))
    }
}