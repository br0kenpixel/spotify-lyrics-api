use serde::Deserialize;
use serde_aux::prelude::deserialize_number_from_string;

#[derive(Clone, Debug, Deserialize)]
pub struct LyricsData {
    pub lyrics: Lyrics,
    pub colors: Colors,
    pub hasVocalRemoval: bool,
}

#[derive(Clone, Debug, Deserialize)]
#[allow(non_camel_case_types)]
pub enum SyncType {
    LINE_SYNCED,
    WORD_SYNCED,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Lyrics {
    pub syncType: SyncType,
    pub lines: Vec<LyricsLine>,
    pub provider: String,
    pub providerLyricsId: String,
    pub providerDisplayName: String,
    pub syncLyricsUri: String,
    pub isDenseTypeface: bool,
    pub alternatives: Vec<String>,
    pub language: String,
    pub isRtlLanguage: bool,
    pub fullscreenAction: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct LyricsLine {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub startTimeMs: u64,
    pub words: String,
    pub syllables: Vec<String>,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub endTimeMs: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Colors {
    pub background: i32,
    pub text: i32,
    pub highlightText: i32,
}
