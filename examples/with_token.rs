use spotify_lyrics_api::{Spotify, SpotifyID};

fn main() {
    let token = "YOUR_ACCESS_TOKEN";
    let track_link = "https://open.spotify.com/track/6OkP1t5se0cYZTfnNE5gC4";

    // Create a client
    let spotify: Spotify = Spotify::new_with_token(token);
    let track_id: SpotifyID = SpotifyID::from_url(track_link);

    let lyrics = spotify.get_lyrics(&track_id).unwrap(); /* json */
    println!("{lyrics:#?}");
}
