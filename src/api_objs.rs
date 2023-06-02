use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct TokenRequestResponse {
    pub clientId: String,
    pub accessToken: String,
    pub accessTokenExpirationTimestampMs: u64,
    pub isAnonymous: bool,
}
