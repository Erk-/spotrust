extern crate reqwest;
extern crate spotrust;

use spotrust::*;

fn main() {
    let spotify_client = reqwest::Client::new();
    let spotify_token: SpotifyToken = SpotifyToken::new(
        &spotify_client,
        String::from("SPOTIFY_CLIENT_ID"),
        String::from("SPOTIFY_CLIENT_SECRET"),
    );

    let uri = uri_from_str(String::from("spotify:track:4uLU6hMCjMI75M1A2tKUQC")).unwrap();

    // Track
    let track = track_get(&spotify_client, spotify_token.clone(), uri.clone());
    println!("Name: {}\nArtist: {}", track.name, track.artists[0].name);

    // Track features
    let track_features =
        track_get_audio_features(&spotify_client, spotify_token.clone(), uri.clone());
    println!("Danceability: {}", track_features.danceability);

    // Playlist tracks
    let playlist_uri = uri_from_str(String::from(
        "spotify:user:1111580463:playlist:3wPPzGGWWqIWWHu3TuilEh",
    )).unwrap();
    let playlist_tracks =
        playlist_get_tracks(&spotify_client, spotify_token.clone(), playlist_uri.clone());

    println!("Number of tracks: {}", playlist_tracks.items.len());
    println!(
        "First track: {} by {}",
        playlist_tracks.items[0].track.name, playlist_tracks.items[0].track.artists[0].name
    );
}
