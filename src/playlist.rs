extern crate reqwest;

use model::*;
use construct_headers_token;

pub fn playlist_get_tracks(client: &reqwest::Client, t: SpotifyToken, id: SpotifyURI) -> SpotifyPagingTracks {
    let uri_parts: Vec<&str> = id.uri_raw.split(':').collect();

    let url =
        format!("https://api.spotify.com/v1/users/{}/playlists/{}/tracks",
                &uri_parts[2],
                &uri_parts[4]
        );

    let res = client
        .get(url.as_str())
        .headers(construct_headers_token(t.clone()))
        .send();
    
    let mut resp = match res {
        Ok(resp) => resp,
        Err(why) => panic!("Err: {}", why),
    };
    let jres: SpotifyPagingTracks = resp.json().unwrap();
    jres
}
