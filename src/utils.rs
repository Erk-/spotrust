pub fn uri_from_str(s: String) -> SpotifyURI {
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
        uri_type: t,
    };
    ret
}

fn get_key(key: usize) -> Option<String> {
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

fn get_mode(mode: usize) -> Option<String> {
    match mode {
        1 => Some(String::from("Minor")),
        0 => Some(String::from("Major")),
        _ => None,
    }
}