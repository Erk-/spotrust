use model::*;
use regex::Regex;

pub fn uri_from_str(s: String) -> Option<SpotifyURI> {
    let re = Regex::new(r"spotify:[a-z]+:[a-zA-Z0-9]+").unwrap();
    match re.is_match(&s) {
        true => {
            let raw = s.clone();
            let v: Vec<&str> = s.split(':').collect();
            let t: UriType = match v[1] {
                "track" => UriType::Track,
                "album" => UriType::Album,
                "playlist" => UriType::Playlist,
                "artist" => UriType::Artist,
                 _ => UriType::Other,
            };
            let ret = SpotifyURI {
                uri: String::from(v[2]),
                uri_raw: raw,
                uri_type: t,
            };
            Some(ret)},
        false => None,
    }
}

pub fn get_key(key: usize) -> Option<String> {
    match key {
        0 => Some(String::from("C")),
        1 => Some(String::from("C♯/D♭")),
        2 => Some(String::from("D")),
        3 => Some(String::from("D♯/E♭")),
        4 => Some(String::from("E")),
        5 => Some(String::from("F")),
        6 => Some(String::from("F♯/G♭")),
        7 => Some(String::from("G")),
        8 => Some(String::from("G♯/A♭")),
        9 => Some(String::from("A")),
        10 => Some(String::from("A♯/B♭")),
        11 => Some(String::from("B")),
        _ => None,
    }
}

pub fn get_mode(mode: usize) -> Option<String> {
    match mode {
        1 => Some(String::from("Minor")),
        0 => Some(String::from("Major")),
        _ => None,
    }
}

pub fn get_largest_image(imgvec: &Vec<SpotifyImage>) -> Option<SpotifyImage> {
    if imgvec.len() > 0 {
        return Some(imgvec[0].clone());
    }
    else {
        None
    }
}
