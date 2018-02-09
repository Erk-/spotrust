extern crate reqwest;
extern crate spotrust;

use spotrust::*;

fn main() {
    let spotify_client = reqwest::Client::new();
    let spotify_token = token_get(
        String::from("SPOTIFY_CLIENT_ID"),
        String::from("SPOTIFY_CLIENT_SECRET")
    );

    let uri = uri_from_str(String::from("spotify:track:4uLU6hMCjMI75M1A2tKUQC")).unwrap();

    let track = track_get(&spotify_client, spotify_token.clone(), uri.clone());
    println!("Name: {}\nArtist: {}", track.name, track.artists[0].name);

    let track_features = track_get_audio_features(&spotify_client, spotify_token.clone(), uri.clone());
    println!("Danceability: {}", track_features.danceability);
}
