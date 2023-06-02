# spotify-lyrics-api-rust
A client for scraping song lyrics from Spotify written in Rust.
This project is based on [akashrchandran/spotify-lyrics-api](https://github.com/akashrchandran/spotify-lyrics-api), which was written in PHP.

Example usage:
```rust
use spotify_lyrics_api::{Spotify, SpotifyID};

fn main() {
    let sp_dc = "YOUR_SP_DC";
    let track_id = "xyz";

    // Create a client
    let spotify: Spotify = Spotify::new(sp_dc);
    let track_id: SpotifyID = SpotifyID::from_id(track_id);

    let lyrics: String = spotify.get_lyrics(&track_id).unwrap(); /* json */
    println!("{lyrics}");
}
```

Alternatively, you can also use song links. The track ID will be parsed from the URL.
```rust
use spotify_lyrics_api::{Spotify, SpotifyID};

fn main() {
    let sp_dc = "YOUR_SP_DC";
    let track_link = "https://open.spotify.com/track/xyz?xy=blablablah";

    // Create a client
    let spotify: Spotify = Spotify::new(sp_dc);
    let track_id: SpotifyID = SpotifyID::from_url(track_link);

    let lyrics: String = spotify.get_lyrics(&track_id).unwrap(); /* json */
    println!("{lyrics}");
}
```
> To get your SP_DC token, check out [this](https://github.com/akashrchandran/syrics/wiki/Finding-sp_dc) guide.

## Dependencies
- [`curl`](https://crates.io/crates/curl)
- [`json`](https://crates.io/crates/json) (Note: This is __not__ the same as [`serde_json`](https://crates.io/crates/serde_json))

## Objects
- `Spotify`
    - Represents a client, which can grab the lyrics
    - Can be `clone()`d
    - Implements `Debug`, so it can be printed using the debug formatter - `println!("{:?}", obj)`
- `SpotifyID`
    - Represents a unique ID of a track on Spotify
    - Can be `clone()`d and copied
    - Implements `Debug`, so it can be printed using the debug formatter - `println!("{:?}", obj)`

## Output
The `get_lyrics()` function returns a [`String`](https://doc.rust-lang.org/std/string/struct.String.html) containing a JSON object. It does __not__ return a [`JsonValue`](https://docs.rs/json/0.12.4/json/enum.JsonValue.html).
The JSON is mostly the same as what [akashrchandran/spotify-lyrics-api](https://github.com/akashrchandran/spotify-lyrics-api) gives you. You will get some extra information such as the "lyrics provider" (which as of now, is [Musixmatch](https://www.musixmatch.com)), "background color" (used by the Spotify Web Client) and some other stuff.

### Example output
```json
{
    "lyrics":
    {
        "syncType": "LINE_SYNCED",
        "lines":
        [
            {
                "startTimeMs": "13830",
                "words": "Forgive me for makin' you wait up for me",
                "syllables":
                [],
                "endTimeMs": "0"
            },
            {
                "startTimeMs": "17470",
                "words": "I pull you in like the waves of the sea",
                "syllables":
                [],
                "endTimeMs": "0"
            },
            ...
        ],
        "provider": "MusixMatch",
        "providerLyricsId": "01234567890",
        "providerDisplayName": "Musixmatch",
        "syncLyricsUri": "",
        "isDenseTypeface": false,
        "alternatives":
        [],
        "language": "en",
        "isRtlLanguage": false,
        "fullscreenAction": "FULLSCREEN_LYRICS"
    },
    "colors":
    {
        "background": -1481964,
        "text": -11591418,
        "highlightText": -1
    },
    "hasVocalRemoval": false
}
```