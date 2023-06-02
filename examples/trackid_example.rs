use spotify_lyrics_api::{Spotify, SpotifyID};

fn main() {
    let sp_dc = "YOUR_SP_DC";
    let track_id = "xyz";

    // Create a client
    let spotify: Spotify = Spotify::new(sp_dc);
    let track_id: SpotifyID = SpotifyID::from_id(track_id);

    let lyrics = spotify.get_lyrics(&track_id).unwrap(); /* json */
    println!("{lyrics:#?}");
}
