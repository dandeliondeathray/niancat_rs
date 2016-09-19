extern crate niancat;
extern crate slack;

use std::time::Duration;
use std::thread;

use niancat::{SlackListChannels, initialize, NiancatHandler};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 4 {
        println!("Usage: niancat <token> <channel> <dictionary>")
    }
    let api_key = args[1].clone();
    let channel_name = args[2].clone();
    let dictionary_file = args[3].clone();

    let slack_list_channels = SlackListChannels {
        token: api_key.clone(),
    };

    let init_result = initialize(&slack_list_channels, &dictionary_file, &channel_name);
    let (dictionary, channel_id) = match init_result {
        Err(reason) => panic!(reason),
        Ok(x) => x,
    };



    let mut handler = NiancatHandler::new(&dictionary, channel_id);

    // Get an initial list of all users.
    match slack_list_channels.list_users() {
        Ok(users_list) => {
            for u in users_list.members {
                handler.update_user(&u);
            }
        },

        Err(e) => {
            panic!("Could not list users! Reason: {:?}", e);
        }
    }

    loop {
        let mut client = slack::RtmClient::new(&api_key);
        let r = client.login_and_run::<NiancatHandler>(&mut handler);
        match r {
            Ok(_) => {}
            Err(err) => println!("Error: {}", err),
        }

        print!("Reconnecting in 60 seconds... ");
        thread::sleep(Duration::from_secs(60));
        println!("Reconnecting!");
    }
}
