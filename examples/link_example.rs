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
