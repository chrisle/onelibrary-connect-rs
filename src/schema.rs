//! Advanced raw row types matching pinned TypeScript schema; numeric database IDs are integers.
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContentRow {
    #[serde(rename = "content_id")]
    pub content_id: i64,
    #[serde(rename = "title")]
    pub title: Option<String>,
    #[serde(rename = "titleForSearch")]
    pub title_for_search: Option<String>,
    #[serde(rename = "subtitle")]
    pub subtitle: Option<String>,
    #[serde(rename = "bpmx100")]
    pub bpmx100: Option<f64>,
    #[serde(rename = "length")]
    pub length: Option<f64>,
    #[serde(rename = "trackNo")]
    pub track_no: Option<i64>,
    #[serde(rename = "discNo")]
    pub disc_no: Option<i64>,
    #[serde(rename = "artist_id_artist")]
    pub artist_id_artist: Option<i64>,
    #[serde(rename = "artist_id_remixer")]
    pub artist_id_remixer: Option<i64>,
    #[serde(rename = "artist_id_originalArtist")]
    pub artist_id_original_artist: Option<i64>,
    #[serde(rename = "artist_id_composer")]
    pub artist_id_composer: Option<i64>,
    #[serde(rename = "artist_id_lyricist")]
    pub artist_id_lyricist: Option<i64>,
    #[serde(rename = "album_id")]
    pub album_id: Option<i64>,
    #[serde(rename = "genre_id")]
    pub genre_id: Option<i64>,
    #[serde(rename = "label_id")]
    pub label_id: Option<i64>,
    #[serde(rename = "key_id")]
    pub key_id: Option<i64>,
    #[serde(rename = "color_id")]
    pub color_id: Option<i64>,
    #[serde(rename = "image_id")]
    pub image_id: Option<i64>,
    #[serde(rename = "djComment")]
    pub dj_comment: Option<String>,
    #[serde(rename = "rating")]
    pub rating: Option<i64>,
    #[serde(rename = "releaseYear")]
    pub release_year: Option<i64>,
    #[serde(rename = "releaseDate")]
    pub release_date: Option<String>,
    #[serde(rename = "dateCreated")]
    pub date_created: Option<String>,
    #[serde(rename = "dateAdded")]
    pub date_added: Option<String>,
    #[serde(rename = "path")]
    pub path: Option<String>,
    #[serde(rename = "fileName")]
    pub file_name: Option<String>,
    #[serde(rename = "fileSize")]
    pub file_size: Option<f64>,
    #[serde(rename = "fileType")]
    pub file_type: Option<i64>,
    #[serde(rename = "bitrate")]
    pub bitrate: Option<i64>,
    #[serde(rename = "bitDepth")]
    pub bit_depth: Option<i64>,
    #[serde(rename = "samplingRate")]
    pub sampling_rate: Option<i64>,
    #[serde(rename = "isrc")]
    pub isrc: Option<String>,
    #[serde(rename = "djPlayCount")]
    pub dj_play_count: Option<i64>,
    #[serde(rename = "isHotCueAutoLoadOn")]
    pub is_hot_cue_auto_load_on: Option<i64>,
    #[serde(rename = "isKuvoDeliverStatusOn")]
    pub is_kuvo_deliver_status_on: Option<i64>,
    #[serde(rename = "kuvoDeliveryComment")]
    pub kuvo_delivery_comment: Option<String>,
    #[serde(rename = "masterDbId")]
    pub master_db_id: Option<i64>,
    #[serde(rename = "masterContentId")]
    pub master_content_id: Option<i64>,
    #[serde(rename = "analysisDataFilePath")]
    pub analysis_data_file_path: Option<String>,
    #[serde(rename = "analysedBits")]
    pub analysed_bits: Option<i64>,
    #[serde(rename = "contentLink")]
    pub content_link: Option<i64>,
    #[serde(rename = "hasModified")]
    pub has_modified: Option<i64>,
    #[serde(rename = "cueUpdateCount")]
    pub cue_update_count: Option<i64>,
    #[serde(rename = "analysisDataUpdateCount")]
    pub analysis_data_update_count: Option<i64>,
    #[serde(rename = "informationUpdateCount")]
    pub information_update_count: Option<i64>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArtistRow {
    #[serde(rename = "artist_id")]
    pub artist_id: i64,
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[serde(rename = "nameForSearch")]
    pub name_for_search: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AlbumRow {
    #[serde(rename = "album_id")]
    pub album_id: i64,
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[serde(rename = "artist_id")]
    pub artist_id: Option<i64>,
    #[serde(rename = "image_id")]
    pub image_id: Option<i64>,
    #[serde(rename = "isComplation")]
    pub is_complation: Option<i64>,
    #[serde(rename = "nameForSearch")]
    pub name_for_search: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GenreRow {
    #[serde(rename = "genre_id")]
    pub genre_id: i64,
    #[serde(rename = "name")]
    pub name: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeyRow {
    #[serde(rename = "key_id")]
    pub key_id: i64,
    #[serde(rename = "name")]
    pub name: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ColorRow {
    #[serde(rename = "color_id")]
    pub color_id: i64,
    #[serde(rename = "name")]
    pub name: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LabelRow {
    #[serde(rename = "label_id")]
    pub label_id: i64,
    #[serde(rename = "name")]
    pub name: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImageRow {
    #[serde(rename = "image_id")]
    pub image_id: i64,
    #[serde(rename = "path")]
    pub path: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CueRow {
    #[serde(rename = "cue_id")]
    pub cue_id: i64,
    #[serde(rename = "content_id")]
    pub content_id: Option<i64>,
    #[serde(rename = "kind")]
    pub kind: Option<i64>,
    #[serde(rename = "colorTableIndex")]
    pub color_table_index: Option<i64>,
    #[serde(rename = "cueComment")]
    pub cue_comment: Option<String>,
    #[serde(rename = "isActiveLoop")]
    pub is_active_loop: Option<i64>,
    #[serde(rename = "beatLoopNumerator")]
    pub beat_loop_numerator: Option<i64>,
    #[serde(rename = "beatLoopDenominator")]
    pub beat_loop_denominator: Option<i64>,
    #[serde(rename = "inUsec")]
    pub in_usec: Option<f64>,
    #[serde(rename = "outUsec")]
    pub out_usec: Option<f64>,
    #[serde(rename = "in150FramePerSec")]
    pub in150_frame_per_sec: Option<i64>,
    #[serde(rename = "out150FramePerSec")]
    pub out150_frame_per_sec: Option<i64>,
    #[serde(rename = "inMpegFrameNumber")]
    pub in_mpeg_frame_number: Option<i64>,
    #[serde(rename = "outMpegFrameNumber")]
    pub out_mpeg_frame_number: Option<i64>,
    #[serde(rename = "inMpegAbs")]
    pub in_mpeg_abs: Option<i64>,
    #[serde(rename = "outMpegAbs")]
    pub out_mpeg_abs: Option<i64>,
    #[serde(rename = "inDecodingStartFramePosition")]
    pub in_decoding_start_frame_position: Option<i64>,
    #[serde(rename = "outDecodingStartFramePosition")]
    pub out_decoding_start_frame_position: Option<i64>,
    #[serde(rename = "inFileOffsetInBlock")]
    pub in_file_offset_in_block: Option<i64>,
    #[serde(rename = "OutFileOffsetInBlock")]
    pub out_file_offset_in_block: Option<i64>,
    #[serde(rename = "inNumberOfSampleInBlock")]
    pub in_number_of_sample_in_block: Option<i64>,
    #[serde(rename = "outNumberOfSampleInBlock")]
    pub out_number_of_sample_in_block: Option<i64>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlaylistRow {
    #[serde(rename = "playlist_id")]
    pub playlist_id: i64,
    #[serde(rename = "sequenceNo")]
    pub sequence_no: Option<i64>,
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[serde(rename = "image_id")]
    pub image_id: Option<i64>,
    #[serde(rename = "attribute")]
    pub attribute: Option<i64>,
    #[serde(rename = "playlist_id_parent")]
    pub playlist_id_parent: Option<i64>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlaylistContentRow {
    #[serde(rename = "playlist_id")]
    pub playlist_id: i64,
    #[serde(rename = "content_id")]
    pub content_id: i64,
    #[serde(rename = "sequenceNo")]
    pub sequence_no: i64,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PropertyRow {
    #[serde(rename = "deviceName")]
    pub device_name: Option<String>,
    #[serde(rename = "dbVersion")]
    pub db_version: Option<String>,
    #[serde(rename = "numberOfContents")]
    pub number_of_contents: Option<i64>,
    #[serde(rename = "createdDate")]
    pub created_date: Option<String>,
    #[serde(rename = "backGroundColorType")]
    pub back_ground_color_type: Option<i64>,
    #[serde(rename = "myTagMasterDBID")]
    pub my_tag_master_dbid: Option<i64>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MyTagRow {
    #[serde(rename = "myTag_id")]
    pub my_tag_id: i64,
    #[serde(rename = "sequenceNo")]
    pub sequence_no: Option<i64>,
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[serde(rename = "attribute")]
    pub attribute: Option<i64>,
    #[serde(rename = "myTag_id_parent")]
    pub my_tag_id_parent: Option<i64>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MyTagContentRow {
    #[serde(rename = "myTag_id")]
    pub my_tag_id: i64,
    #[serde(rename = "content_id")]
    pub content_id: i64,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HistoryRow {
    #[serde(rename = "history_id")]
    pub history_id: i64,
    #[serde(rename = "sequenceNo")]
    pub sequence_no: Option<i64>,
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[serde(rename = "attribute")]
    pub attribute: Option<i64>,
    #[serde(rename = "history_id_parent")]
    pub history_id_parent: Option<i64>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HistoryContentRow {
    #[serde(rename = "history_id")]
    pub history_id: i64,
    #[serde(rename = "content_id")]
    pub content_id: i64,
    #[serde(rename = "sequenceNo")]
    pub sequence_no: i64,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HotCueBankListRow {
    #[serde(rename = "hotCueBankList_id")]
    pub hot_cue_bank_list_id: i64,
    #[serde(rename = "sequenceNo")]
    pub sequence_no: Option<i64>,
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[serde(rename = "image_id")]
    pub image_id: Option<i64>,
    #[serde(rename = "attribute")]
    pub attribute: Option<i64>,
    #[serde(rename = "hotCueBankList_id_parent")]
    pub hot_cue_bank_list_id_parent: Option<i64>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HotCueBankListCueRow {
    #[serde(rename = "hotCueBankList_id")]
    pub hot_cue_bank_list_id: i64,
    #[serde(rename = "cue_id")]
    pub cue_id: i64,
    #[serde(rename = "sequenceNo")]
    pub sequence_no: i64,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MenuItemRow {
    #[serde(rename = "menuItem_id")]
    pub menu_item_id: i64,
    #[serde(rename = "kind")]
    pub kind: Option<i64>,
    #[serde(rename = "name")]
    pub name: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CategoryRow {
    #[serde(rename = "category_id")]
    pub category_id: i64,
    #[serde(rename = "menuItem_id")]
    pub menu_item_id: Option<i64>,
    #[serde(rename = "sequenceNo")]
    pub sequence_no: Option<i64>,
    #[serde(rename = "isVisible")]
    pub is_visible: Option<i64>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SortRow {
    #[serde(rename = "sort_id")]
    pub sort_id: i64,
    #[serde(rename = "menuItem_id")]
    pub menu_item_id: Option<i64>,
    #[serde(rename = "sequenceNo")]
    pub sequence_no: Option<i64>,
    #[serde(rename = "isVisible")]
    pub is_visible: Option<i64>,
    #[serde(rename = "isSelectedAsSubColumn")]
    pub is_selected_as_sub_column: Option<i64>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RecommendedLikeRow {
    #[serde(rename = "content_id_1")]
    pub content_id_1: i64,
    #[serde(rename = "content_id_2")]
    pub content_id_2: i64,
    #[serde(rename = "rating")]
    pub rating: Option<i64>,
    #[serde(rename = "createdDate")]
    pub created_date: Option<f64>,
}
pub struct CueKind;
impl CueKind {
    pub const MEMORY_CUE: i64 = 0;
    pub const HOT_CUE: i64 = 1;
}
pub struct PlaylistAttribute;
impl PlaylistAttribute {
    pub const PLAYLIST: i64 = 0;
    pub const FOLDER: i64 = 1;
}
pub struct MyTagAttribute;
impl MyTagAttribute {
    pub const TAG: i64 = 0;
    pub const FOLDER: i64 = 1;
}
pub struct MenuItemKind;
impl MenuItemKind {
    pub const GENRE: i64 = 128;
    pub const ARTIST: i64 = 129;
    pub const ALBUM: i64 = 130;
    pub const TRACK: i64 = 131;
    pub const PLAYLIST: i64 = 132;
    pub const BPM: i64 = 133;
    pub const RATING: i64 = 134;
    pub const YEAR: i64 = 135;
    pub const REMIXER: i64 = 136;
    pub const LABEL: i64 = 137;
    pub const ORIGINAL_ARTIST: i64 = 138;
    pub const KEY: i64 = 139;
    pub const DATE_ADDED: i64 = 140;
    pub const CUE: i64 = 141;
    pub const COLOR: i64 = 142;
    pub const FOLDER: i64 = 144;
    pub const SEARCH: i64 = 145;
    pub const TIME: i64 = 146;
    pub const BITRATE: i64 = 147;
    pub const FILE_NAME: i64 = 148;
    pub const HISTORY: i64 = 149;
    pub const COMMENTS: i64 = 150;
    pub const DJ_PLAY_COUNT: i64 = 151;
    pub const HOT_CUE_BANK: i64 = 152;
    pub const DEFAULT: i64 = 161;
    pub const ALPHABET: i64 = 162;
    pub const MATCHING: i64 = 170;
}
