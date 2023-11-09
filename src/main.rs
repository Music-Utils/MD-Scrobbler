use crate::scrobble::scrobble;
use std::io;
use std::thread;

mod scrobble;

fn main() -> Result<(), std::io::Error> {
    let mut counter = 1;
    loop {
        println!("iteration {}", counter);
        let input = vec_from_input();
        thread::spawn(|| {
            scrobble(input);
        });
        counter += 1;
    }
    //Ok(())
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
