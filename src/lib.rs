extern crate reqwest;
extern crate base64;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use reqwest::header::{Headers, UserAgent, ContentType};
use reqwest::header::Bearer;
use reqwest::header::Authorization;

use std::time::{Duration, SystemTime};

mod model;
mod token;
mod utils;

fn construct_headers_token(t: SpotifyToken) -> Headers {
    let tok = token_update(t);
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    headers.set(Authorization(Bearer { token: tok.access_token.to_owned() }));
    headers
}

pub fn track_get(client: &reqwest::Client, t: SpotifyToken, id: SpotifyURI) -> SpotifyTrack {
    let url = format!("https://api.spotify.com/v1/tracks/{}", id.uri);
    let res = client
        .get(url.as_str())
        .headers(construct_headers_token(t))
        .send();
    let mut resp = match res {
        Ok(resp) => resp,
        Err(why) => panic!("Err: {}", why),
    };
    let jres: SpotifyTrack = resp.json().unwrap();
    jres
}

/*
fn main() {
    let c = reqwest::Client::new();
    let token = token_get("SPOTIFY_CLIENT_ID", "SPOTIFY_CLIENT_SECRET");
    let uri = uri_from_str(String::from("spotify:track:11dFghVXANMlKmJXsNCbNl"));
    let track = track_get(&c, token, uri);
    println!("Name: {:?}", track.name);
}
*/
