extern crate rustfm_scrobble;
use rustfm_scrobble::{Scrobble, Scrobbler};

fn main() {
    let username = "";
    let password = "";
    let api_key = "";
    let api_secret = "";

    let mut scrobbler = Scrobbler::new(api_key, api_secret);
    let _ = scrobbler.authenticate_with_password(username, password);

    let song = &Scrobble::new("black midi", "Near DT, MI", "Schlagenheim");
    let _ = scrobbler.scrobble(song);
}
