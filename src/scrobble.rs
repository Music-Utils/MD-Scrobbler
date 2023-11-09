use rustfm_scrobble::{Scrobble, Scrobbler};
use serde::Deserialize;
use std::{fs::File, io::prelude::*};

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

pub fn scrobble(song_info: Vec<String>) {
    let mut file = File::open("config.toml").unwrap();
    let mut config = String::new();
    _ = file.read_to_string(&mut config);
    let config: Config = toml::from_str(&config[..]).unwrap();

    let mut scrobbler = Scrobbler::new(&config.login.apikey[..], &config.login.apisecret[..]);
    let _ = scrobbler.authenticate_with_password(&config.login.username, &config.login.password);

    let song = &Scrobble::new(&song_info[0][..], &song_info[1][..], &song_info[2][..]);
    let _ = scrobbler.scrobble(song);
}
