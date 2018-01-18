extern crate reqwest;
extern crate base64;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use reqwest::header::{Headers, ContentType, Bearer, Authorization}; // UserAgent, 

//use std::time::{Duration, SystemTime};

pub mod model;
pub mod token;
pub mod utils;

pub use model::*;
pub use token::*;
pub use utils::*;

fn construct_headers_token(t: SpotifyToken) -> Headers {
    let tok = token_update(t);
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    headers.set(Authorization(Bearer { token: tok.access_token.to_owned() }));
    headers
}

pub mod track;
pub use track::*;


/*
fn main() {
    let c = reqwest::Client::new();
    let token = token_get("SPOTIFY_CLIENT_ID", "SPOTIFY_CLIENT_SECRET");
    let uri = uri_from_str(String::from("spotify:track:11dFghVXANMlKmJXsNCbNl"));
    let track = track_get(&c, token, uri);
    println!("Name: {:?}", track.name);
}
*/
