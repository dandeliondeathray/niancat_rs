extern crate niancat;
extern crate slack;

use niancat::dictionary::{Dictionary, CheckWord};
use niancat::types::Word;
use niancat::types;
use slack::api::Error;
use slack::api::channels::ListResponse;

#[test]
fn init_dictionary_test() {
    let result = Dictionary::from_file("tests/test_dictionary.txt");

    match result {
        Err(ex) => assert!(false, "Failed to read dictionary from file because: {}", ex),
        Ok(d) => {
            assert!(d.is_solution(&Word("ABCDEFGHI".into())));
            assert!(d.is_solution(&Word("GALLTJUTA".into())));
            assert!(d.is_solution(&Word("UVWXYZÅÄÖ".into())));
            assert!(!d.is_solution(&Word("ABC".into())));
            assert!(!d.is_solution(&Word("ABCDEF".into())));
        }
    }
}

struct FakeListChannels {
    v: Option<ListResponse>,
}

impl niancat::ListChannels for FakeListChannels {
    fn list_channels(&self) -> Result<ListResponse, Error> {
        if let Some(ref list_response) = self.v {
            return Ok(list_response.clone());
        }

        Err(Error::Api("An error".into()))
    }
}

#[test]
fn init_handler_test() {
    let chans: Vec<slack::Channel> = vec![
        slack::Channel {
            id: "C4567".into(),
            name: "general".into(),
            is_channel: true,
            created: 1,
            creator: "user0".into(),
            is_archived: false,
            is_general: true,
            members: None,
            topic: None,
            purpose: None,
            is_member: true,
            last_read: None,
            unread_count: None,
            unread_count_display: None,
        },

       slack::Channel {
            id: "C0123".into(),
            name: "konsulatet".into(),
            is_channel: true,
            created: 1,
            creator: "user0".into(),
            is_archived: false,
            is_general: false,
            members: None,
            topic: None,
            purpose: None,
            is_member: true,
            last_read: None,
            unread_count: None,
            unread_count_display: None,
        },

        slack::Channel {
            id: "C890".into(),
            name: "thirdchannel".into(),
            is_channel: true,
            created: 1,
            creator: "user0".into(),
            is_archived: false,
            is_general: false,
            members: None,
            topic: None,
            purpose: None,
            is_member: true,
            last_read: None,
            unread_count: None,
            unread_count_display: None,
        },
    ];

    let fake_list_channels = FakeListChannels { v: Some(ListResponse { channels: chans }) };
    let result = niancat::initialize(fake_list_channels, &"tests/test_dictionary.txt".into(), &"konsulatet".into());

    match result {
        Err(s) => assert!(false, "Initialization failed, because: {}", s),
        Ok((d, channel_id)) => {
            // Check that the channel "konsulatet" was found to have the id C0123.
            assert_eq!(channel_id, types::Channel("C0123".into()));

            // Same tests as `init_dictionary_test` above.
            assert!(d.is_solution(&Word("ABCDEFGHI".into())));
            assert!(d.is_solution(&Word("GALLTJUTA".into())));
            assert!(d.is_solution(&Word("UVWXYZÅÄÖ".into())));
            assert!(!d.is_solution(&Word("ABC".into())));
            assert!(!d.is_solution(&Word("ABCDEF".into())));
        }
    }
}

#[test]
fn failed_channel_list_init_test() {
    let fake_list_channels = FakeListChannels { v: None };
    let result = niancat::initialize(fake_list_channels, &"tests/test_dictionary.txt".into(), &"konsulatet".into());

    match result {
        Err(_) => {},
        Ok((_, channel_id)) => {
            assert!(false, "Got channel id {:?}, but expected error", channel_id);
        }
    }
}

#[test]
fn failed_dictionary_init_test() {
    let chans: Vec<slack::Channel> = vec![
       slack::Channel {
            id: "C0123".into(),
            name: "konsulatet".into(),
            is_channel: true,
            created: 1,
            creator: "user0".into(),
            is_archived: false,
            is_general: false,
            members: None,
            topic: None,
            purpose: None,
            is_member: true,
            last_read: None,
            unread_count: None,
            unread_count_display: None,
        },
    ];

    let fake_list_channels = FakeListChannels { v: Some(ListResponse { channels: chans }) };
    let result = niancat::initialize(fake_list_channels, &"tests/no_such_dictionary.txt".into(), &"konsulatet".into());

    match result {
        Err(_) => {},
        Ok(_) => {
            assert!(false, "Got an unexpected dictionary");
        }
    }
}