use std::time::{Duration, SystemTime};

#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize)]
pub struct SpotifyGetToken {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
    pub scope: String,
}

#[allow(non_snake_case)]
#[derive(Clone, Debug)]
pub struct SpotifyToken {
    pub access_token: String,
    pub expires_in: Duration,
    pub toi: SystemTime,
    pub cid: String,
    pub cs: String,
}

#[allow(non_snake_case)]
#[derive(Clone, Debug)]
pub enum UriType {
    Track,
    Album,
    Playlist,
    Artist,
    Other,
}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Clone, Debug)]
pub struct SpotifyURI {
    pub uri: String,
    pub uri_type: UriType,
}

// https://developer.spotify.com/web-api/object-model/

// DONE https://developer.spotify.com/web-api/object-model/#album-object-full
#[allow(dead_code)] 
#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize)]
pub struct SpotifyAlbumFull {
    pub album_type: String,
    pub artists: Vec<SpotifyArtistSimplified>,
    pub available_markets: Vec<String>,
    pub copyrights: Vec<SpotifyCopyright>,
    pub external_ids: SpotifyExternalIds,
    pub external_urls: SpotifyExternalUrls,
    pub genres: Vec<String>,
    pub href: String,
    pub id: String,
    pub images: Vec<SpotifyImage>,
    pub label: String,
    pub name: String,
    pub popularity: usize,
    pub release_date: String,
    pub release_date_precision: String,
    pub tracks: SpotifyPagingTracks,
    #[serde(rename = "type")]
    pub type_: String,
    pub uri: String,
}

// DONE https://developer.spotify.com/web-api/object-model/#album-object-simplified
#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize)]
pub struct SpotifyAlbumSimplified {
    pub album_type: String,
    pub artists: Vec<SpotifyArtistSimplified>,
    pub available_markets: Vec<String>,
    pub external_urls: SpotifyExternalUrls,
    pub href: String,
    pub id: String,
    pub images: Vec<SpotifyImage>,
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub uri: String,
}

// DONE https://developer.spotify.com/web-api/object-model/#artist-object-full
#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize)]
pub struct SpotifyArtistFull {
    pub external_urls: SpotifyExternalUrls,
    pub followers: SpotifyFollowers,
    pub genres: Vec<String>,
    pub href: String,
    pub id: String,
    pub images: Vec<SpotifyImage>,
    pub name: String,
    pub popularity: usize,
    #[serde(rename = "type")]
    pub type_: String,
    pub uri: String,
}

// DONE https://developer.spotify.com/web-api/object-model/#artist-object-simplified
#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize)]
pub struct SpotifyArtistSimplified {
    pub external_urls: SpotifyExternalUrls,
    pub href: String,
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub uri: String,
}

// DONE https://developer.spotify.com/web-api/object-model/#audio-features-object
#[allow(dead_code)]
#[allow(non_snake_case)]
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
    #[serde(rename = "type")]
    pub type_: String,
    pub uri: String,
    pub valence: f64,
}

// DONE https://developer.spotify.com/web-api/object-model/#category-object
#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize)]
pub struct SpotifyCategory {
    pub href: String,
    pub icons: Vec<SpotifyImage>,
    pub id: String,
    pub name: String
}

// DONE https://developer.spotify.com/web-api/object-model/#copyright-object
#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize)]
pub struct SpotifyCopyright {
    pub text: String,
    #[serde(rename = "type")]
    pub type_: String,
}

// DONE https://developer.spotify.com/web-api/object-model/#cursor-object
#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize)]
pub struct SpotifyCursor {
    pub after: String,
}

// DONE https://developer.spotify.com/web-api/object-model/#error-object
#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize)]
pub struct SpotifyError {
    pub status: usize,
    pub message: String,
}

// DONE https://developer.spotify.com/web-api/object-model/#external-id-object
// DOES NOT take care of other types of external id than the ones below.
#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize)]
pub struct SpotifyExternalIds {
    pub isrc: Option<String>,
    pub ean: Option<String>,
    pub upc: Option<String>,
}

// DONE https://developer.spotify.com/web-api/object-model/#external-url-object
// DOES NOT take care of external urls except the ones listed below.
#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize)]
pub struct SpotifyExternalUrls {
    pub spotify: String,
}

// DONE https://developer.spotify.com/web-api/object-model/#followers-object
#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize)]
pub struct SpotifyFollowers {
    pub href: Option<String>, // NOTE: Does not work at the moment per url above.
    pub total: usize,
}

// DONE https://developer.spotify.com/web-api/object-model/#image-object
#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize)]
pub struct SpotifyImage {
    pub height: usize,
    pub url: String,
    pub width: usize,
}

#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize)]
pub struct SpotifyPagingTracks {
    pub href: String,
    pub items: Vec<SpotifyTrackSimplified>,
    pub limit: usize,
    pub next: String,
    pub offset: usize,
    pub previous: String,
    pub total: usize,
}

#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize)]
pub struct SpotifyCursorBasedPagingTracks {
    pub href: String,
    pub items: Vec<SpotifyTrackSimplified>,
    pub limit: usize,
    pub next: String,
    pub cursors: SpotifyCursor,
    pub total: usize,
}

// DONE https://developer.spotify.com/web-api/object-model/#playlist-object-full
#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize)]
pub struct SpotifyPlaylistFull {
    pub collaborative: bool,
    pub description: String,
    pub external_urls: SpotifyExternalUrls,
    pub followers: SpotifyFollowers,
    pub href: String,
    pub id: String,
    pub images: Vec<SpotifyImage>,
    pub name: String,
    pub owner: SpotifyUserPublic,
    pub public: Option<bool>,
    pub snapshot_id: String,
    pub tracks: SpotifyPagingTracks,
    #[serde(rename = "type")]
    pub type_: String,
    pub uri: String,
}

// DONE https://developer.spotify.com/web-api/object-model/#playlist-object-simplified
#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize)]
pub struct SpotifyPlaylistSimplified {
    pub collaborative: bool,
    pub external_urls: SpotifyExternalUrls,
    pub href: String,
    pub id: String,
    pub images: Vec<SpotifyImage>,
    pub name: String,
    pub owner: SpotifyUserPublic,
    pub public: Option<bool>,
    pub snapshot_id: String,
    pub tracks: SpotifyPagingTracks,
    #[serde(rename = "type")]
    pub type_: String,
    pub uri: String,
}

// DONE https://developer.spotify.com/web-api/object-model/#playlist-track-object
#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize)]
pub struct SpotifyPlaylistTrack {
    pub added_at: String,
    pub added_by: SpotifyUserPublic,
    pub is_local: bool,
    pub track: SpotifyTrackFull,
}

// DONE https://developer.spotify.com/web-api/object-model/#recommendations-object
#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize)]
pub struct SpotifyRecommendations {
    pub seeds: Vec<SpotifyRecommendationsSeed>,
    pub tracks: Vec<SpotifyTrackSimplified>,
}

// DONE https://developer.spotify.com/web-api/object-model/#recommendations-seed-object
#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize)]
pub struct SpotifyRecommendationsSeed {
    pub afterFilteringSize: usize,
    pub afterRelinkingSize: usize,
    pub href: String,
    pub id: String,
    pub initialPoolSize: usize,
    #[serde(rename = "type")]
    pub type_: String,
}

// DONE https://developer.spotify.com/web-api/object-model/#saved-track-object
#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize)]
pub struct SpotifySavedTrack {
    pub added_at: String,
    pub track: SpotifyTrackFull,
}

// DONE https://developer.spotify.com/web-api/object-model/#saved-album-object
#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize)]
pub struct SpotifySavedAlbum {
    added_at: String,
    album: SpotifyAlbumFull
}

// UNDUCUMENTED https://developer.spotify.com/web-api/object-model/#track-object-full
#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize)]
pub struct SpotifyRestrictions {
    pub reason: String,
}

// DONE https://developer.spotify.com/web-api/object-model/#track-object-full
#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize)]
pub struct SpotifyTrackFull {
    pub album: SpotifyAlbumSimplified,
    pub artists: Vec<SpotifyArtistSimplified>,
    pub available_markets: Vec<String>,
    pub disc_number: usize,
    pub duration_ms: usize,
    pub explicit: bool,
    pub external_ids: SpotifyExternalIds,
    pub external_urls: SpotifyExternalUrls,
    pub href: String,
    pub id: String,
    pub is_playable: bool,
    pub linked_from: SpotifyTrackLink,
    pub restrictions: SpotifyRestrictions,
    pub name: String,
    pub popularity: usize,
    pub preview_url: Option<String>,
    pub track_number: usize,
    #[serde(rename = "type")]
    pub type_: String,
    pub uri: String,
}

// DONE https://developer.spotify.com/web-api/object-model/#track-object-simplified
#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize)]
pub struct SpotifyTrackSimplified {
    pub artists: Vec<SpotifyArtistSimplified>,
    pub available_markets: Vec<String>,
    pub disc_number: usize,
    pub duration_ms: usize,
    pub explicit: bool,
    pub external_urls: SpotifyExternalUrls,
    pub href: String,
    pub id: String,
    pub is_playable: bool,
    pub linked_from: SpotifyTrackLink,
    pub name: String,
    pub preview_url: Option<String>,
    pub track_number: usize,
    #[serde(rename = "type")]
    pub type_: String,
    pub uri: String,
}

// DONE https://developer.spotify.com/web-api/object-model/#track-link
#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize)]
pub struct SpotifyTrackLink {
    pub external_urls: SpotifyExternalUrls,
    pub href: String,
    pub id: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub uri: String,
}

// DONE https://developer.spotify.com/web-api/object-model/#user-object-private
#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize)]
pub struct SpotifyUserPrivate {
    pub birthdate: String,
    pub country: String,
    pub display_name: Option<String>,
    pub email: String,
    pub external_urls: SpotifyExternalUrls,
    pub followers: SpotifyFollowers,
    pub href: String,
    pub id: String,
    pub images: Vec<SpotifyImage>,
    pub product: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub uri: String,
}

// DONE https://developer.spotify.com/web-api/object-model/#user-object-public
#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize)]
pub struct SpotifyUserPublic {
    pub display_name: Option<String>,
    pub external_urls: SpotifyExternalUrls,
    pub followers: SpotifyFollowers,
    pub href: String,
    pub id: String,
    pub images: Vec<SpotifyImage>,
    #[serde(rename = "type")]
    pub type_: String,
    pub uri: String,
}

// DONE https://developer.spotify.com/web-api/object-model/#play-history-object
#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize)]
pub struct SpotifyPlayHistory {
    pub track: SpotifyTrackSimplified,
    pub played_at: String,
    pub context: SpotifyContext
}

// DONE https://developer.spotify.com/web-api/object-model/#context-object
#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize)]
pub struct SpotifyContext {
    #[serde(rename = "type")]
    pub type_: String,
    pub href: String,
    pub external_urls: SpotifyExternalUrls,
    pub uri: String,
}

