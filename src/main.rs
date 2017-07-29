use std::default::Default;

mod bot;
use bot::Bot;


fn main() {
    let mut bot = Bot {
        username: "kappa",
        oauth: "oauth:wasd",
        channel: "twitch",
        ..Default::default()
    };

    bot.connect();
}