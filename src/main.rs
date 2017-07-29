extern crate ini;
use ini::Ini;

fn main() {

    let conf = Ini::load_from_file("config.ini").unwrap();

    let twitch_settings = conf.section(Some("Twitch".to_owned())).unwrap();

    let username = twitch_settings.get("username").unwrap();
    let password = twitch_settings.get("password").unwrap();
    let channel = twitch_settings.get("channel").unwrap();

    println!("{:?} {:?} {:?}", username, password, channel);
}
