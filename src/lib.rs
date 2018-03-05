extern crate base64;
extern crate regex;
extern crate reqwest;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use reqwest::header::{Authorization, Bearer, ContentType, Headers}; // UserAgent,

//use std::time::{Duration, SystemTime};

pub mod model;
pub mod token;
pub mod utils;

pub use model::*;
pub use token::*;
pub use utils::*;

fn construct_headers_token(t: SpotifyToken) -> Headers {
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    headers.set(Authorization(Bearer {
        token: t.access_token.to_owned(),
    }));
    headers
}

pub mod track;
pub mod playlist;

pub use track::*;
pub use playlist::*;

/*
fn main() {
    let c = reqwest::Client::new();
    let token = token_get("SPOTIFY_CLIENT_ID", "SPOTIFY_CLIENT_SECRET");
    let uri = uri_from_str(String::from("spotify:track:11dFghVXANMlKmJXsNCbNl"));
    let track = track_get(&c, token, uri);
    println!("Name: {:?}", track.name);
}
*/
