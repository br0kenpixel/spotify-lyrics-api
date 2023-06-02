#![allow(
    clippy::missing_panics_doc,
    clippy::manual_assert,
    clippy::must_use_candidate,
    unused,
    dead_code,
    non_snake_case
)]

use curl::easy::{Easy, List};
use serde::Deserialize;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;
mod lyrics_objs;
pub use lyrics_objs::*;
mod api_objs;
use api_objs::*;

const TOKEN_URL: &str =
    "https://open.spotify.com/get_access_token?reason=transport&productType=web_player";
const LYRICS_URL: &str = "https://spclient.wg.spotify.com/color-lyrics/v2/track/";

#[derive(Clone, Debug)]
pub struct Spotify {
    token: String,
}

#[derive(Clone, Debug)]
pub struct SpotifyID {
    id: String,
}

impl Spotify {
    pub fn new(sp_dc: &str) -> Self {
        let mut easy = Easy::new();
        easy.timeout(Duration::from_millis(600)).unwrap();
        easy.ssl_verify_peer(false).unwrap();
        easy.ssl_verify_host(false).unwrap();
        easy.verbose(false).unwrap();
        //return transfer???
        easy.show_header(false).unwrap();
        easy.follow_location(false).unwrap();
        easy.custom_request("GET").unwrap();

        let mut headers = List::new();
        headers.append("User-Agent: Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/101.0.0.0 Safari/537.36").unwrap();
        headers.append("App-platform: WebPlayer").unwrap();
        headers
            .append("content-type: text/html; charset=utf-8")
            .unwrap();
        headers
            .append(format!("cookie: sp_dc={sp_dc}").as_str())
            .unwrap();

        easy.http_headers(headers).unwrap();
        easy.url(TOKEN_URL).unwrap();

        let response = Self::get_response_string(&mut easy);
        let response: TokenRequestResponse =
            serde_json::from_str(&response).expect("Invalid SP_DC");
        if response.isAnonymous {
            panic!("Invalid SP_DC");
        }

        Self {
            token: response.accessToken,
        }
    }

    pub fn new_with_token(token: &str) -> Self {
        Self {
            token: token.to_owned(),
        }
    }

    fn get_response_string(easy: &mut Easy) -> String {
        let buf = Arc::new(Mutex::new(Vec::new()));
        let buf_closure = buf.clone();

        easy.write_function(move |data| {
            buf_closure.lock().unwrap().extend_from_slice(data);
            Ok(data.len())
        })
        .unwrap();

        easy.perform().expect("Failed to perform request");
        let buf = buf.lock().unwrap().clone();

        String::from_utf8(buf).unwrap()
    }

    pub fn get_lyrics(&self, track_id: &SpotifyID) -> Option<LyricsData> {
        let mut easy = Easy::new();
        easy.custom_request("GET").unwrap();

        let mut headers = List::new();
        headers.append("User-Agent: Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/101.0.0.0 Safari/537.36").unwrap();
        headers.append("App-platform: WebPlayer").unwrap();
        headers
            .append("content-type: text/html; charset=utf-8")
            .unwrap();
        headers
            .append("Accept: application/json; charset=utf-8")
            .unwrap(); /* otherwise gives protobuf */
        headers
            .append(format!("authorization: Bearer {}", self.token).as_str())
            .unwrap();
        easy.http_headers(headers).unwrap();

        easy.url(format!("{LYRICS_URL}{}?format=json&market=from_token", track_id.id).as_str())
            .unwrap();

        let response = Self::get_response_string(&mut easy);
        let response = serde_json::from_str(&response).unwrap();

        Some(response)
    }
}

impl SpotifyID {
    pub fn from_id(id: &str) -> Self {
        Self { id: id.to_owned() }
    }

    pub fn from_url(url: &str) -> Self {
        let start = url.find("/track/").expect("Invalid URL") + 7;
        let end = url.find('?').map_or(url.len(), |pos| pos);
        let slice = &url[start..end];

        Self {
            id: slice.to_owned(),
        }
    }
}

pub fn fix_end_times(lyrics: &mut LyricsData) {
    let mut lines = &lyrics.lyrics.lines;
    let line_count = lines.len();
    let mut new = Vec::new();

    let mut i = 0;
    loop {
        let Some(mut first) = lines.get(i).cloned() else {
            break;
        };
        let Some(second) = lines.get(i + 1).cloned() else {
            break;
        };

        if second.words == "♪" || second.words.is_empty() {
            first.endTimeMs = second.startTimeMs;
            new.push(first);
            i += 2;
            continue;
        }

        new.push(first);
        new.push(second);
        i += 1;
    }

    lyrics.lyrics.lines = new;
}
