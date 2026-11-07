use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NamedEntity {
    pub id: i64,
    pub name: String,
}
pub type Artist = NamedEntity;
pub type Album = NamedEntity;
pub type Genre = NamedEntity;
pub type Key = NamedEntity;
pub type Color = NamedEntity;
pub type Label = NamedEntity;
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Artwork {
    pub id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Playlist {
    pub id: i64,
    pub name: String,
    pub is_folder: bool,
    pub parent_id: Option<i64>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MyTag {
    pub id: i64,
    pub name: String,
    pub is_folder: bool,
    pub parent_id: Option<i64>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistorySession {
    pub id: i64,
    pub name: String,
    pub parent_id: Option<i64>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HotCueBankList {
    pub id: i64,
    pub name: String,
    pub parent_id: Option<i64>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MenuItem {
    pub id: i64,
    pub kind: i64,
    pub name: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Category {
    pub id: i64,
    pub menu_item_id: i64,
    pub name: String,
    pub kind: i64,
    pub is_visible: bool,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SortOption {
    pub id: i64,
    pub menu_item_id: i64,
    pub name: String,
    pub kind: i64,
    pub is_visible: bool,
    pub is_selected_as_sub_column: bool,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceProperty {
    pub device_name: String,
    pub db_version: String,
    pub number_of_contents: i64,
    pub created_date: String,
    pub background_color_type: i64,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistEntryFks {
    pub id: usize,
    pub sort_index: i64,
    pub playlist_id: i64,
    pub track_id: i64,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistEntryRelations {
    pub id: usize,
    pub sort_index: i64,
    pub track: Track,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistQueryResult {
    pub folders: Vec<Playlist>,
    pub playlists: Vec<Playlist>,
    pub track_entries: Vec<PlaylistEntryFks>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MyTagQueryResult {
    pub folders: Vec<MyTag>,
    pub tags: Vec<MyTag>,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntityFK {
    WithFKs,
    WithRelations,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DatabasePreference {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "oneLibrary")]
    OneLibrary,
    #[serde(rename = "pdb")]
    Pdb,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DatabaseType {
    #[serde(rename = "oneLibrary")]
    OneLibrary,
    #[serde(rename = "pdb")]
    Pdb,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum CueAndLoop {
    #[serde(rename = "cue_point")]
    CuePoint {
        offset: f64,
        #[serde(skip_serializing_if = "Option::is_none")]
        label: Option<String>,
        color: Option<i64>,
    },
    #[serde(rename = "loop")]
    Loop {
        offset: f64,
        length: f64,
        #[serde(skip_serializing_if = "Option::is_none")]
        label: Option<String>,
        color: Option<i64>,
    },
    #[serde(rename = "hot_cue")]
    Hotcue {
        offset: f64,
        button: i64,
        #[serde(skip_serializing_if = "Option::is_none")]
        label: Option<String>,
        color: Option<i64>,
    },
    #[serde(rename = "hot_loop")]
    Hotloop {
        offset: f64,
        length: f64,
        button: i64,
        #[serde(skip_serializing_if = "Option::is_none")]
        label: Option<String>,
        color: Option<i64>,
    },
}
pub type CuePoint = CueAndLoop;
pub type Loop = CueAndLoop;
pub type Hotcue = CueAndLoop;
pub type Hotloop = CueAndLoop;
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Track {
    pub id: i64,
    pub title: String,
    pub duration: f64,
    pub tempo: f64,
    pub rating: f64,
    pub comment: String,
    pub file_path: String,
    pub file_name: String,
    pub autoload_hotcues: bool,
    pub kuvo_public: bool,
    pub cue_and_loops: Option<Vec<CueAndLoop>>,
    pub artwork: Option<Artwork>,
    pub artist: Option<Artist>,
    pub original_artist: Option<Artist>,
    pub remixer: Option<Artist>,
    pub composer: Option<Artist>,
    pub album: Option<Album>,
    pub label: Option<Label>,
    pub genre: Option<Genre>,
    pub color: Option<Color>,
    pub key: Option<Key>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub track_number: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disc_number: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_rate: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_depth: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub play_count: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mix_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analyze_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analyze_date: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_added: Option<serde_json::Value>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackFks {
    pub id: i64,
    pub title: String,
    pub duration: f64,
    pub tempo: f64,
    pub rating: f64,
    pub comment: String,
    pub file_path: String,
    pub file_name: String,
    pub autoload_hotcues: bool,
    pub kuvo_public: bool,
    pub cue_and_loops: Option<Vec<CueAndLoop>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artwork_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artist_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_artist_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remixer_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub composer_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub album_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genre_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub track_number: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disc_number: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_rate: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_depth: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub play_count: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mix_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analyze_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analyze_date: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_added: Option<serde_json::Value>,
}
pub type PlaylistEntry = PlaylistEntryRelations;

pub struct HotcueButton;
#[allow(non_upper_case_globals)]
impl HotcueButton {
    pub const A: i64 = 1;
    pub const B: i64 = 2;
    pub const C: i64 = 3;
    pub const D: i64 = 4;
    pub const E: i64 = 5;
    pub const F: i64 = 6;
    pub const G: i64 = 7;
    pub const H: i64 = 8;
}
pub struct CueColor;
#[allow(non_upper_case_globals)]
impl CueColor {
    pub const None: i64 = 0;
    pub const Blank: i64 = 21;
    pub const Magenta: i64 = 49;
    pub const Violet: i64 = 56;
    pub const Fuchsia: i64 = 60;
    pub const LightSlateBlue: i64 = 62;
    pub const Blue: i64 = 1;
    pub const SteelBlue: i64 = 5;
    pub const Aqua: i64 = 9;
    pub const SeaGreen: i64 = 14;
    pub const Teal: i64 = 18;
    pub const Green: i64 = 22;
    pub const Lime: i64 = 26;
    pub const Olive: i64 = 30;
    pub const Yellow: i64 = 32;
    pub const Orange: i64 = 38;
    pub const Red: i64 = 42;
    pub const Pink: i64 = 45;
}
