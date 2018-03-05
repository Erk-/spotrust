extern crate reqwest;

use reqwest::header::{Headers, UserAgent};
use std::time::{Duration, SystemTime};
use model::*;
use base64;

fn construct_headers_auth(cid: &String, cs: &String) -> Headers {
    let mut headers = Headers::new();
    headers.set(UserAgent::new("reqwest"));
    let client_id = cid;
    let client_secret = cs;
    let auth = format!(
        "Basic {}",
        base64::encode(&format!("{}:{}", client_id, client_secret))
    );
    headers.set_raw("Authorization", auth);
    headers
}

impl SpotifyToken {
    pub fn new(client: &reqwest::Client, cid: String, cs: String) -> Self {
        let o_client = client.to_owned();
        token_get_client(o_client, cid, cs)
    }

    pub fn is_valid(&self) -> bool {
        let sys_time = SystemTime::now();
        let difference = sys_time
            .duration_since(self.toi)
            .expect("SystemTime::duration_since failed");
        if difference > self.expires_in {
            false
        } else {
            true
        }
    }

    pub fn update(&mut self, client: &reqwest::Client) {
        let o_client = client.to_owned();
        let t_token = token_get_client(o_client, self.cid.clone(), self.cs.clone());
        self.access_token = t_token.access_token;
        self.expires_in = t_token.expires_in;
        self.toi = t_token.toi;
    }
}

fn token_get_client(client: reqwest::Client, cid: String, cs: String) -> SpotifyToken {
    let params = [("grant_type", "client_credentials")];
    let res = client
        .post("https://accounts.spotify.com/api/token")
        .headers(construct_headers_auth(&cid, &cs))
        .form(&params)
        .send();
    let resp: SpotifyGetToken = res.unwrap().json().unwrap();
    let token: SpotifyToken = SpotifyToken {
        access_token: resp.access_token,
        expires_in: Duration::from_secs(resp.expires_in),
        toi: SystemTime::now(),
        cid: cid,
        cs: cs,
    };
    token
}

/*
fn token_update(t: SpotifyToken) -> SpotifyToken {
    let sys_time = SystemTime::now();
    let difference = sys_time
        .duration_since(t.toi)
        .expect("SystemTime::duration_since failed");
    if difference > t.expires_in {
        token_get(t.cid, t.cs)
    } else {
        t
    }
}
*/
