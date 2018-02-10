extern crate reqwest;

use model::*;
use construct_headers_token;

pub fn track_get_audio_analysis(client: &reqwest::Client, t: SpotifyToken, id: SpotifyURI) -> SpotifyAudioAnalysis {
    let url = format!("https://api.spotify.com/v1/audio-analysis/{}", id.uri);
    let res = client
        .get(url.as_str())
        .headers(construct_headers_token(t))
        .send();
    let mut resp = match res {
        Ok(resp) => resp,
        Err(why) => panic!("Err: {}", why),
    };
    let jres: SpotifyAudioAnalysis = resp.json().unwrap();
    jres
}

pub fn track_get_audio_features(client: &reqwest::Client, t: SpotifyToken, id: SpotifyURI) -> SpotifyAudioFeatures {
    let url = format!("https://api.spotify.com/v1/audio-features/{}", id.uri);
    let res = client
        .get(url.as_str())
        .headers(construct_headers_token(t))
        .send();
    let mut resp = match res {
        Ok(resp) => resp,
        Err(why) => panic!("Err: {}", why),
    };
    let jres: SpotifyAudioFeatures = resp.json().unwrap();
    jres
}

// TODO: make it take a vec of uri's
pub fn track_get_several_audio_features(client: &reqwest::Client, t: SpotifyToken, id: SpotifyURI) -> SpotifySeveralAudioFeatures {
    let url = format!("https://api.spotify.com/v1/audio-features?ids={}", id.uri);
    let res = client
        .get(url.as_str())
        .headers(construct_headers_token(t))
        .send();
    let mut resp = match res {
        Ok(resp) => resp,
        Err(why) => panic!("Err: {}", why),
    };
    let jres: SpotifySeveralAudioFeatures = resp.json().unwrap();
    jres
}

pub fn track_get(client: &reqwest::Client, t: SpotifyToken, id: SpotifyURI) -> SpotifyTrackFull {
    let url = format!("https://api.spotify.com/v1/tracks/{}", id.uri);
    let res = client
        .get(url.as_str())
        .headers(construct_headers_token(t))
        .send();
    let mut resp = match res {
        Ok(resp) => resp,
        Err(why) => panic!("Err: {}", why),
    };
    //println!("{:?}", &resp.text());
    let jres: SpotifyTrackFull = resp.json().unwrap();
    jres
}

// TODO: make it take a vec of uri's
pub fn track_get_tracks(client: &reqwest::Client, t: SpotifyToken, id: SpotifyURI) -> SpotifyTracks {
    let url = format!("https://api.spotify.com/v1/tracks?ids={}", id.uri);
    let res = client
        .get(url.as_str())
        .headers(construct_headers_token(t))
        .send();
    let mut resp = match res {
        Ok(resp) => resp,
        Err(why) => panic!("Err: {}", why),
    };
    let jres: SpotifyTracks = resp.json().unwrap();
    jres
}
