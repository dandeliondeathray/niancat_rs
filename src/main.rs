extern crate niancat;
extern crate slack;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let api_key = match args.len() {
        0 | 1 => panic!("No API key in args!"),
        x => {
            args[x - 1].clone()
        }
    };

    let mut handler = niancat::NiancatHandler;
    let mut client = slack::RtmClient::new(&api_key);
    let r = client.login_and_run::<niancat::NiancatHandler>(&mut handler);
    match r {
        Ok(_) => {}
        Err(err) => panic!("Error: {}", err),
    }
    println!("{}", client.get_name().unwrap());
    println!("{}", client.get_team().unwrap().name);
}
