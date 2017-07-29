extern crate irc;

use std::default::Default;
use self::irc::client::prelude::*;

pub struct Bot {
  pub username: &'static str,
  pub oauth: &'static str,
  pub channel: &'static str,
  pub in_channel: bool,
}

impl Default for Bot {
  fn default() -> Bot {
    Bot {
      username: "",
      oauth: "",
      channel: "",
      in_channel: false,
    }
  }
}

impl Bot {
  pub fn connect(&mut self) -> () {
    let server = IrcServer::new("config.json").unwrap();
    server.identify().unwrap();
    server.for_each_incoming(|message| {
      println!("{}", message);
      match message.command {
        Command::PRIVMSG(ref target, ref msg) => {
          if msg.contains("Kappa") {
            server.send_privmsg(target, "KKaper").unwrap();
          }
        }
        _ => (),
      }
    }).unwrap()
  }
}