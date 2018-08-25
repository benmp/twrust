#![allow(unknown_lints)]
#![warn(clippy)]

extern crate dotenv;
extern crate env_logger;
extern crate failure;
extern crate tungstenite;
extern crate url;

use std::env;

use dotenv::dotenv;
use failure::Error;
use tungstenite::{connect, Message};
use url::Url;

mod twitchparser;

type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    env_logger::init();
    dotenv().expect("could not read dotenv file");

    let (mut socket, response) = connect(Url::parse(&env::var("URL").expect("missing URL configuration"))?).expect("tried to connect");

    println!("Connected to the server");
    println!("Response HTTP code: {}", response.code);
    println!("Response contains the following headers:");
    for &(ref header, _ /*value*/) in response.headers.iter() {
        println!("* {}", header);
    }

    match socket.write_message(Message::Text(String::from("PASS oauth:") + &env::var("OAUTH").expect("missing OAUTH configuration"))) {
        Ok(_) => (),
        Err(err) => println!("Tried to PASS: {}", err)
    };

    match socket.write_message(Message::Text(String::from("NICK ") + &env::var("NICK").expect("missing NICK configuration"))) {
        Ok(_) => (),
        Err(err) => println!("Tried to NICK: {}", err)
    };

    match socket.write_message(Message::Text("JOIN #tsm_myth".into())) {
         Ok(_) => (),
         Err(err) => println!("Tried to JOIN: {}", err)
    };

    match socket.write_message(Message::Text("CAP REQ :twitch.tv/membership".into())) {
         Ok(_) => (),
         Err(err) => println!("Tried to CAP REQ membership: {}", err)
    };

    match socket.write_message(Message::Text("CAP REQ :twitch.tv/tags".into())) {
         Ok(_) => (),
         Err(err) => println!("Tried to CAP REQ tags: {}", err)
    };

    match socket.write_message(Message::Text("CAP REQ :twitch.tv/commands".into())) {
         Ok(_) => (),
         Err(err) => println!("Tried to CAP REQ commands: {}", err)
    };

    loop {
        let msg = match socket.read_message() {
            Ok(msg) => msg,
            Err(err) => {
                println!("Error: {}", err);
                break;
            }
        };

        if msg.is_text() {
            match msg.to_text() {
                Ok(txt) => match txt.trim() {
                    "PING :tmi.twitch.tv" => {
                        println!("got ping");
                        socket.write_message(Message::Text(String::from("PONG :tmi.twitch.tv"))).expect("pong failed to send");
                    },
                    _ => println!("Received: {}", txt)
                },
                Err(err) => println!("Error getting text: {}", err)
            }
        }
    }

    socket.close(None)?;

    Ok(())
}
