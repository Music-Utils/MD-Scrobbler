extern crate rustfm_scrobble;
use rustfm_scrobble::{Scrobble, Scrobbler};
use serde::Deserialize;
use std::{fs::File, io, io::prelude::*};

#[derive(Deserialize)]
struct Config {
    login: Login,
}

#[derive(Deserialize)]
struct Login {
    apikey: String,
    apisecret: String,
    username: String,
    password: String,
}

fn main() {
    let song_info = vec_from_input();

    let mut file = File::open("config.toml").unwrap();
    let mut config = String::new();
    _ = file.read_to_string(&mut config);
    let config: Config = toml::from_str(&config[..]).unwrap();

    let mut scrobbler = Scrobbler::new(&config.login.apikey[..], &config.login.apisecret[..]);
    let _ = scrobbler.authenticate_with_password(&config.login.username, &config.login.password);

    let song = &Scrobble::new(&song_info[0][..], &song_info[1][..], &song_info[2][..]);
    let _ = scrobbler.scrobble(song);
}

fn vec_from_input() -> Vec<String> {
    let stdin = io::stdin();

    let mut buffer = String::new();
    println!("input song name: ");
    stdin.read_line(&mut buffer).unwrap();
    let song_name = buffer;

    let mut buffer = String::new();
    println!("by: ");
    stdin.read_line(&mut buffer).unwrap();
    let song_artist = buffer;

    let mut buffer = String::new();
    println!("from the album: ");
    stdin.read_line(&mut buffer).unwrap();
    let song_album = buffer;

    vec![song_artist, song_name, song_album]
}
