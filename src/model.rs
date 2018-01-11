use std::time::{Duration, SystemTime};

#[allow(dead_code)]
#[derive(Clone, Debug, Deserialize)]
pub struct SpotifyGetToken {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
    pub scope: String,
}

#[derive(Clone, Debug)]
pub struct SpotifyToken {
    pub access_token: String,
    pub expires_in: Duration,
    pub toi: SystemTime,
    pub cid: String,
    pub cs: String,
}

#[derive(Clone, Debug)]
pub enum UriType {
    Track,
    Album,
    Playlist,
    Artist,
    Other,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct SpotifyURI {
    pub uri: String,
    pub uri_type: UriType,
}

#[allow(dead_code)]
#[derive(Clone, Debug, Deserialize)]
pub struct SpotifyExternalUrls {
    pub spotify: String,
}

#[allow(dead_code)]
#[derive(Clone, Debug, Deserialize)]
pub struct SpotifyArtist {
    pub external_urls: SpotifyExternalUrls,
    pub followers: Option<SpotifyFollowers>,
    pub genres: Option<Vec<String>>,
    pub href: String,
    pub id: String,
    pub images: Option<Vec<SpotifyImage>>,
    pub name: String,
    pub popularity: Option<usize>,
    #[serde(rename = "type")]
    pub type_: String,
    pub uri: String,
}

#[allow(dead_code)]
#[derive(Clone, Debug, Deserialize)]
pub struct SpotifyImage {
    pub height: usize,
    pub url: String,
    pub width: usize,
}

#[allow(dead_code)]
#[derive(Clone, Debug, Deserialize)]
pub struct SpotifyAlbum {
    pub album_type: String,
    pub artists: Vec<SpotifyArtist>,
    pub external_urls: SpotifyExternalUrls,
    pub href: String,
    pub id: String,
    pub images: Vec<SpotifyImage>,
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub uri: String,
}

#[allow(dead_code)]
#[derive(Clone, Debug, Deserialize)]
pub struct SpotifyExternalIds {
    pub isrc: Option<String>,
    pub ean: Option<String>,
    pub upc: Option<String>,
}

#[allow(dead_code)]
#[derive(Clone, Debug, Deserialize)]
pub struct SpotifyTrack {
    pub album: Option<SpotifyAlbum>,
    pub artists: Vec<SpotifyArtist>,
    pub disc_number: usize,
    pub duration_ms: usize,
    pub explicit: bool,
    pub external_ids: SpotifyExternalIds,
    pub external_urls: SpotifyExternalUrls,
    pub href: String,
    pub id: String,
    pub name: String,
    pub popularity: usize,
    pub preview_url: Option<String>,
    pub track_number: usize,
    #[serde(rename = "type")]
    pub type_: String,
    pub uri: String,
}

#[allow(dead_code)]
#[derive(Clone, Debug, Deserialize)]
pub struct SpotifyAudioFeatures {
    pub acousticness: f64,
    pub analysis_url: String,
    pub danceability: f64,
    pub duration_ms: usize,
    pub energy: f64,
    pub id: String,
    pub instrumentalness: f64,
    pub key: usize,
    pub liveness: f64,
    pub loudness: f64,
    pub mode: usize,
    pub speechiness: f64,
    pub tempo: f64,
    pub time_signature: usize,
    pub track_href: String,
    pub href: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub uri: String,
    pub valence: f64,
}

#[allow(dead_code)]
#[derive(Clone, Debug, Deserialize)]
pub struct SpotifyCategory {
    pub href: String,
    pub icons: Vec<SpotifyImage>,
    pub id: String,
    pub name: String
}

#[derive(Clone, Debug, Deserialize)]
pub struct SpotifyCopyright {
    pub text: String,
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SpotifyCursor {
    pub after: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SpotifyError {
    pub status: usize,
    pub message: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SpotifyFollowers {
    pub href: String,
    pub total: usize,
}
/*
// NOT IMPLEMENTED!
/*
#[derive(Clone, Debug, Deserialize)]
pub struct SpotifyPaging {
}
*/

#[derive(Clone, Debug, Deserialize)]
pub struct SpotifyPlaylist {
    pub collaborative: bool,
    pub external_urls: SpotifyExternalUrls,
    pub href: String,
    pub id: String,
    pub images: Vec<SpotifyImage>,
    pub name: String,
    pub owner: SpotifyUser, // TODO: SpotifyUser
    pub public: Option<bool>,
    pub snapshot_id: String,
    pub tracks: SpotifyTracks, // TODO: SpotifyTracks
    #[serde(rename = "type")]
    pub type_: String,
    pub uri: String,
}
*/
