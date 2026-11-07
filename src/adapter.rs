use crate::{types::*, Error, Result};
use chrono::{DateTime, Local, NaiveDate, NaiveDateTime, SecondsFormat, Utc};
use rusqlite::{types::ValueRef, Connection, Params};
use serde_json::{json, Value};
use std::path::Path;

// Project only mapped scalar fields. Future SQLite schema extensions may carry
// BLOBs; they must not break unrelated reads or consume result memory. Known
// scalar BLOBs still reach query() and fail explicitly.
const TRACK_SELECT:&str="SELECT c.content_id,c.title,c.length,c.bitrate,c.bpmx100,c.rating,c.djComment,c.path,c.fileName,c.trackNo,c.discNo,c.samplingRate,c.bitDepth,c.djPlayCount,c.releaseYear,c.subtitle,c.isHotCueAutoLoadOn,c.isKuvoDeliverStatusOn,c.fileSize,c.analysisDataFilePath,c.releaseDate,c.dateAdded,c.image_id,c.artist_id_artist,c.artist_id_originalArtist,c.artist_id_remixer,c.artist_id_composer,c.album_id,c.label_id,c.genre_id,c.color_id,c.key_id,a.name AS artistName,al.name AS albumName,g.name AS genreName,k.name AS keyName,col.name AS colorName,lbl.name AS labelName,img.path AS artworkPath,remix.name AS remixerName,orig.name AS originalArtistName,comp.name AS composerName FROM content c LEFT JOIN artist a ON c.artist_id_artist=a.artist_id LEFT JOIN album al ON c.album_id=al.album_id LEFT JOIN genre g ON c.genre_id=g.genre_id LEFT JOIN key k ON c.key_id=k.key_id LEFT JOIN color col ON c.color_id=col.color_id LEFT JOIN label lbl ON c.label_id=lbl.label_id LEFT JOIN image img ON c.image_id=img.image_id LEFT JOIN artist remix ON c.artist_id_remixer=remix.artist_id LEFT JOIN artist orig ON c.artist_id_originalArtist=orig.artist_id LEFT JOIN artist comp ON c.artist_id_composer=comp.artist_id";
pub trait DatabaseAdapter {
    fn database_type(&self) -> DatabaseType;
    fn find_track(&self, id: i64) -> Result<Option<Track>>;
    fn find_playlist(&self, id: Option<i64>) -> Result<PlaylistQueryResult>;
    fn close(&mut self);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DatePolicy {
    #[default]
    Local,
    Utc,
}
#[derive(Debug, Clone, Copy, Default)]
pub struct Options {
    pub date_policy: DatePolicy,
}
pub struct OneLibraryAdapter {
    db: Option<Connection>,
    options: Options,
}
impl OneLibraryAdapter {
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        Self::open_with_options(path, Options::default())
    }
    pub fn open_with_options(path: impl AsRef<Path>, options: Options) -> Result<Self> {
        let db = crate::open_one_library_db(path)?;
        // Preparation validates every adapter table and required lookup/sort/join column
        // before the adapter is returned as ready; optional metadata stays nullable.
        for sql in [TRACK_SELECT,"SELECT content_id,title,length,bitrate,bpmx100,rating,djComment,path,fileName,trackNo,discNo,samplingRate,bitDepth,djPlayCount,releaseYear,subtitle,isHotCueAutoLoadOn,isKuvoDeliverStatusOn,fileSize,analysisDataFilePath,releaseDate,dateAdded FROM content",
   "SELECT content_id,inUsec,outUsec,kind,cueComment,colorTableIndex FROM cue",
   "SELECT playlist_id,name,attribute,playlist_id_parent FROM playlist",
   "SELECT playlist_id,content_id,sequenceNo FROM playlist_content",
   "SELECT myTag_id,name,attribute,myTag_id_parent,sequenceNo FROM myTag",
   "SELECT myTag_id,content_id FROM myTag_content",
   "SELECT history_id,name,history_id_parent,sequenceNo FROM history",
   "SELECT history_id,content_id,sequenceNo FROM history_content",
   "SELECT hotCueBankList_id,name,hotCueBankList_id_parent,sequenceNo FROM hotCueBankList",
   "SELECT hotCueBankList_id,cue_id,sequenceNo FROM hotCueBankList_cue",
   "SELECT menuItem_id,kind,name FROM menuItem",
   "SELECT category_id,menuItem_id,sequenceNo,isVisible FROM category",
   "SELECT sort_id,menuItem_id,sequenceNo,isVisible,isSelectedAsSubColumn FROM sort",
   "SELECT deviceName,dbVersion,numberOfContents,createdDate,backGroundColorType FROM property"] {
    db.prepare(sql).map_err(|_|Error::InvalidSchema)?;
  }
        Ok(Self {
            db: Some(db),
            options,
        })
    }
    pub fn database_type(&self) -> DatabaseType {
        DatabaseType::OneLibrary
    }
    pub fn close(&mut self) {
        self.db.take();
    }
    fn query(&self, sql: &str, params: impl Params) -> Result<Vec<Value>> {
        let mut stmt = self
            .db
            .as_ref()
            .ok_or(Error::Closed)?
            .prepare_cached(sql)
            .map_err(|_| Error::QueryFailed)?;
        let names: Vec<String> = stmt.column_names().iter().map(|s| s.to_string()).collect();
        let mut rows = stmt.query(params).map_err(|_| Error::QueryFailed)?;
        let mut result = Vec::new();
        let mut bytes = 0_usize;
        while let Some(row) = rows.next().map_err(|_| Error::QueryFailed)? {
            if result.len() >= 1_000_000 {
                return Err(Error::ResultLimit);
            }
            let mut item = serde_json::Map::new();
            for (i, name) in names.iter().enumerate() {
                let value = match row.get_ref(i).map_err(|_| Error::QueryFailed)? {
                    ValueRef::Null => Value::Null,
                    ValueRef::Integer(v) => json!(v),
                    ValueRef::Real(v) => json!(v),
                    ValueRef::Text(v) => {
                        bytes = bytes.saturating_add(v.len());
                        Value::String(
                            std::str::from_utf8(v)
                                .map_err(|_| Error::QueryFailed)?
                                .to_owned(),
                        )
                    }
                    ValueRef::Blob(_) => return Err(Error::QueryFailed),
                };
                item.insert(name.clone(), value);
            }
            if bytes > 128 * 1024 * 1024 {
                return Err(Error::ResultLimit);
            }
            result.push(Value::Object(item));
        }
        Ok(result)
    }
    pub fn find_track(&self, id: i64) -> Result<Option<Track>> {
        self.query(&format!("{TRACK_SELECT} WHERE c.content_id=?"), [id])?
            .first()
            .map(|r| track(r, self.options.date_policy))
            .transpose()
    }
    pub fn find_all_tracks(&self) -> Result<Vec<Track>> {
        self.query(TRACK_SELECT, [])?
            .iter()
            .map(|r| track(r, self.options.date_policy))
            .collect()
    }
    pub fn find_cues(&self, id: i64) -> Result<Vec<CueAndLoop>> {
        self.query("SELECT content_id,inUsec,outUsec,kind,cueComment,colorTableIndex FROM cue WHERE content_id=?", [id])?
            .iter()
            .map(|r| {
                let offset = number(r, "inUsec") / 1000.;
                let out = r["outUsec"].as_f64();
                let looped = out.is_some_and(|x| x > 0.);
                let length = (out.unwrap_or(0.) - number(r, "inUsec")) / 1000.;
                let button = integer(r, "kind");
                let hot = (1..=8).contains(&button);
                let label = text_opt(r, "cueComment");
                let color = r["colorTableIndex"].as_i64();
                Ok(match (looped, hot) {
                    (true, true) => CueAndLoop::Hotloop {
                        offset,
                        length,
                        button,
                        label,
                        color,
                    },
                    (true, false) => CueAndLoop::Loop {
                        offset,
                        length,
                        label,
                        color,
                    },
                    (false, true) => CueAndLoop::Hotcue {
                        offset,
                        button,
                        label,
                        color,
                    },
                    (false, false) => CueAndLoop::CuePoint {
                        offset,
                        label,
                        color,
                    },
                })
            })
            .collect()
    }
    pub fn find_playlist_by_id(&self, id: i64) -> Result<Option<Playlist>> {
        Ok(self
            .query("SELECT playlist_id,name,attribute,playlist_id_parent FROM playlist WHERE playlist_id=?", [id])?
            .first()
            .map(playlist))
    }
    pub fn find_playlist(&self, id: Option<i64>) -> Result<PlaylistQueryResult> {
        let rows = match id {
            None => self.query(
                "SELECT playlist_id,name,attribute,playlist_id_parent FROM playlist WHERE playlist_id_parent IS NULL",
                [],
            )?,
            Some(id) => self.query("SELECT playlist_id,name,attribute,playlist_id_parent FROM playlist WHERE playlist_id_parent=?", [id])?,
        };
        let (mut folders, mut playlists) = (Vec::new(), Vec::new());
        for row in rows {
            let p = playlist(&row);
            if p.is_folder {
                folders.push(p)
            } else {
                playlists.push(p)
            }
        }
        let mut entries = Vec::new();
        if let Some(id) = id {
            for (index, row) in self
                .query(
                    "SELECT playlist_id,content_id,sequenceNo FROM playlist_content WHERE playlist_id=? ORDER BY sequenceNo",
                    [id],
                )?
                .iter()
                .enumerate()
            {
                entries.push(PlaylistEntryFks {
                    id: index,
                    sort_index: integer(row, "sequenceNo"),
                    playlist_id: integer(row, "playlist_id"),
                    track_id: integer(row, "content_id"),
                });
            }
        }
        Ok(PlaylistQueryResult {
            folders,
            playlists,
            track_entries: entries,
        })
    }
    pub fn find_playlist_contents(&self, id: i64) -> Result<Vec<i64>> {
        self.ids(
            "SELECT content_id FROM playlist_content WHERE playlist_id=? ORDER BY sequenceNo",
            id,
            "content_id",
        )
    }
    fn reference(&self, table: &str, column: &str, id: i64) -> Result<Option<NamedEntity>> {
        Ok(self
            .query(
                &format!("SELECT {column},name FROM {table} WHERE {column}=?"),
                [id],
            )?
            .first()
            .map(|r| NamedEntity {
                id: integer(r, column),
                name: text(r, "name"),
            }))
    }
    pub fn find_artist(&self, id: i64) -> Result<Option<Artist>> {
        self.reference("artist", "artist_id", id)
    }
    pub fn find_album(&self, id: i64) -> Result<Option<Album>> {
        self.reference("album", "album_id", id)
    }
    pub fn find_genre(&self, id: i64) -> Result<Option<Genre>> {
        self.reference("genre", "genre_id", id)
    }
    pub fn find_key(&self, id: i64) -> Result<Option<Key>> {
        self.reference("key", "key_id", id)
    }
    pub fn find_color(&self, id: i64) -> Result<Option<Color>> {
        self.reference("color", "color_id", id)
    }
    pub fn find_label(&self, id: i64) -> Result<Option<Label>> {
        self.reference("label", "label_id", id)
    }
    pub fn find_artwork(&self, id: i64) -> Result<Option<Artwork>> {
        Ok(self
            .query("SELECT image_id,path FROM image WHERE image_id=?", [id])?
            .first()
            .map(|r| Artwork {
                id: integer(r, "image_id"),
                path: text_opt(r, "path"),
            }))
    }
    fn ids(&self, sql: &str, id: i64, field: &str) -> Result<Vec<i64>> {
        Ok(self
            .query(sql, [id])?
            .iter()
            .map(|r| integer(r, field))
            .collect())
    }
    pub fn find_my_tags(&self, id: Option<i64>) -> Result<MyTagQueryResult> {
        let rows=match id{None=>self.query("SELECT myTag_id,name,attribute,myTag_id_parent,sequenceNo FROM myTag WHERE myTag_id_parent IS NULL OR myTag_id_parent=0 ORDER BY sequenceNo",[])?,Some(id)=>self.query("SELECT myTag_id,name,attribute,myTag_id_parent,sequenceNo FROM myTag WHERE myTag_id_parent=? ORDER BY sequenceNo",[id])?};
        let (mut folders, mut tags) = (Vec::new(), Vec::new());
        for row in rows {
            let t = my_tag(&row);
            if t.is_folder {
                folders.push(t)
            } else {
                tags.push(t)
            }
        }
        Ok(MyTagQueryResult { folders, tags })
    }
    pub fn find_my_tag_by_id(&self, id: i64) -> Result<Option<MyTag>> {
        Ok(self
            .query("SELECT myTag_id,name,attribute,myTag_id_parent,sequenceNo FROM myTag WHERE myTag_id=?", [id])?
            .first()
            .map(my_tag))
    }
    pub fn find_my_tag_contents(&self, id: i64) -> Result<Vec<i64>> {
        self.ids(
            "SELECT content_id FROM myTag_content WHERE myTag_id=?",
            id,
            "content_id",
        )
    }
    pub fn find_my_tags_for_track(&self, id: i64) -> Result<Vec<MyTag>> {
        Ok(self.query("SELECT t.myTag_id,t.name,t.attribute,t.myTag_id_parent,t.sequenceNo FROM myTag t INNER JOIN myTag_content tc ON t.myTag_id=tc.myTag_id WHERE tc.content_id=?",[id])?.iter().map(my_tag).collect())
    }
    pub fn find_history_sessions(&self) -> Result<Vec<HistorySession>> {
        Ok(self
            .query("SELECT history_id,name,history_id_parent,sequenceNo FROM history ORDER BY sequenceNo", [])?
            .iter()
            .map(|r| HistorySession {
                id: integer(r, "history_id"),
                name: text(r, "name"),
                parent_id: r["history_id_parent"].as_i64(),
            })
            .collect())
    }
    pub fn find_history_contents(&self, id: i64) -> Result<Vec<i64>> {
        self.ids(
            "SELECT content_id FROM history_content WHERE history_id=? ORDER BY sequenceNo",
            id,
            "content_id",
        )
    }
    pub fn find_hot_cue_bank_lists(&self) -> Result<Vec<HotCueBankList>> {
        Ok(self
            .query("SELECT hotCueBankList_id,name,hotCueBankList_id_parent,sequenceNo FROM hotCueBankList ORDER BY sequenceNo", [])?
            .iter()
            .map(|r| HotCueBankList {
                id: integer(r, "hotCueBankList_id"),
                name: text(r, "name"),
                parent_id: r["hotCueBankList_id_parent"].as_i64(),
            })
            .collect())
    }
    pub fn find_hot_cue_bank_list_cues(&self, id: i64) -> Result<Vec<i64>> {
        self.ids(
            "SELECT cue_id FROM hotCueBankList_cue WHERE hotCueBankList_id=? ORDER BY sequenceNo",
            id,
            "cue_id",
        )
    }
    pub fn find_menu_items(&self) -> Result<Vec<MenuItem>> {
        Ok(self
            .query(
                "SELECT menuItem_id,kind,name FROM menuItem ORDER BY menuItem_id",
                [],
            )?
            .iter()
            .map(|r| MenuItem {
                id: integer(r, "menuItem_id"),
                kind: integer(r, "kind"),
                name: text(r, "name"),
            })
            .collect())
    }
    pub fn find_visible_categories(&self) -> Result<Vec<Category>> {
        Ok(self.query("SELECT c.category_id,c.menuItem_id,c.sequenceNo,c.isVisible,m.name AS menuName,m.kind AS menuKind FROM category c LEFT JOIN menuItem m ON c.menuItem_id=m.menuItem_id WHERE c.isVisible=1 ORDER BY c.sequenceNo",[])?.iter().map(|r|Category{id:integer(r,"category_id"),menu_item_id:integer(r,"menuItem_id"),name:text(r,"menuName"),kind:integer(r,"menuKind"),is_visible:integer(r,"isVisible")!=0}).collect())
    }
    pub fn find_visible_sort_options(&self) -> Result<Vec<SortOption>> {
        Ok(self.query("SELECT s.sort_id,s.menuItem_id,s.sequenceNo,s.isVisible,s.isSelectedAsSubColumn,m.name AS menuName,m.kind AS menuKind FROM sort s LEFT JOIN menuItem m ON s.menuItem_id=m.menuItem_id WHERE s.isVisible=1 ORDER BY s.sequenceNo",[])?.iter().map(|r|SortOption{id:integer(r,"sort_id"),menu_item_id:integer(r,"menuItem_id"),name:text(r,"menuName"),kind:integer(r,"menuKind"),is_visible:integer(r,"isVisible")!=0,is_selected_as_sub_column:integer(r,"isSelectedAsSubColumn")!=0}).collect())
    }
    pub fn get_property(&self) -> Result<Option<DeviceProperty>> {
        Ok(self
            .query("SELECT deviceName,dbVersion,numberOfContents,createdDate,backGroundColorType FROM property LIMIT 1", [])?
            .first()
            .map(|r| DeviceProperty {
                device_name: text(r, "deviceName"),
                db_version: text(r, "dbVersion"),
                number_of_contents: integer(r, "numberOfContents"),
                created_date: text(r, "createdDate"),
                background_color_type: integer(r, "backGroundColorType"),
            }))
    }
}
impl DatabaseAdapter for OneLibraryAdapter {
    fn database_type(&self) -> DatabaseType {
        Self::database_type(self)
    }
    fn find_track(&self, id: i64) -> Result<Option<Track>> {
        Self::find_track(self, id)
    }
    fn find_playlist(&self, id: Option<i64>) -> Result<PlaylistQueryResult> {
        Self::find_playlist(self, id)
    }
    fn close(&mut self) {
        Self::close(self)
    }
}
fn integer(r: &Value, k: &str) -> i64 {
    r[k].as_i64().unwrap_or(0)
}
fn number(r: &Value, k: &str) -> f64 {
    r[k].as_f64().unwrap_or(0.)
}
fn text(r: &Value, k: &str) -> String {
    text_opt(r, k).unwrap_or_default()
}
fn text_opt(r: &Value, k: &str) -> Option<String> {
    r[k].as_str().map(str::to_owned)
}
fn playlist(r: &Value) -> Playlist {
    Playlist {
        id: integer(r, "playlist_id"),
        name: text(r, "name"),
        is_folder: r["attribute"].as_i64() == Some(1),
        parent_id: r["playlist_id_parent"].as_i64(),
    }
}
fn my_tag(r: &Value) -> MyTag {
    MyTag {
        id: integer(r, "myTag_id"),
        name: text(r, "name"),
        is_folder: r["attribute"].as_i64() == Some(1),
        parent_id: r["myTag_id_parent"].as_i64(),
    }
}
fn relation(r: &Value, id: &str, name: &str) -> Option<NamedEntity> {
    let id = integer(r, id);
    (id != 0).then(|| NamedEntity {
        id,
        name: text(r, name),
    })
}
fn resolve_local(d: NaiveDateTime) -> Option<DateTime<Utc>> {
    // Derive candidates from surrounding UTC offsets. This also disambiguates
    // platforms whose libc mktime reports only one side of a DST overlap.
    let mut candidates = Vec::new();
    for hours in [-48, -24, -6, 0, 6, 24, 48] {
        let probe = d
            .checked_add_signed(chrono::Duration::hours(hours))?
            .and_utc();
        let offset = probe.with_timezone(&Local).offset().local_minus_utc();
        let candidate = d
            .checked_sub_signed(chrono::Duration::seconds(i64::from(offset)))?
            .and_utc();
        if !candidates.contains(&candidate) {
            candidates.push(candidate);
        }
    }
    if let Some(value) = candidates
        .iter()
        .filter(|c| c.with_timezone(&Local).naive_local() == d)
        .min()
    {
        return Some(*value);
    }
    // JS Date advances nonexistent times by the positive timezone gap.
    candidates
        .into_iter()
        .filter(|c| {
            let delta = c.with_timezone(&Local).naive_local() - d;
            delta > chrono::Duration::zero() && delta <= chrono::Duration::hours(24)
        })
        .min()
}

fn normalize_iso_calendar(value: &str) -> Option<String> {
    let bytes = value.as_bytes();
    if bytes.len() < 10
        || bytes[4] != b'-'
        || bytes[7] != b'-'
        || !bytes[..4]
            .iter()
            .chain(&bytes[5..7])
            .chain(&bytes[8..10])
            .all(u8::is_ascii_digit)
    {
        return Some(value.to_owned());
    }
    let year = value[..4].parse::<i32>().ok()?;
    let month = value[5..7].parse::<u32>().ok()?;
    let day = value[8..10].parse::<u32>().ok()?;
    if !(1..=12).contains(&month) || !(1..=31).contains(&day) {
        return None;
    }
    // ECMAScript ISO parsing accepts day1..31 and carries excess days into
    // the following month, including nonleap February and 30-day months.
    let mut calendar = NaiveDate::from_ymd_opt(year, month, 1)?
        .checked_add_signed(chrono::Duration::days(i64::from(day) - 1))?;
    let mut tail = value[10..].to_owned();
    if tail.starts_with('T') || tail.starts_with(' ') {
        let time = &tail.as_bytes()[1..];
        if time.len() < 5
            || time[2] != b':'
            || !time[..2].iter().chain(&time[3..5]).all(u8::is_ascii_digit)
        {
            return None;
        }
        let hour = tail[1..3].parse::<u32>().ok()?;
        let minute = tail[4..6].parse::<u32>().ok()?;
        let mut at = 6;
        let mut second = 0;
        let has_seconds = tail.as_bytes().get(at) == Some(&b':');
        if has_seconds {
            if tail.len() < at + 3
                || !tail.as_bytes()[at + 1..at + 3]
                    .iter()
                    .all(u8::is_ascii_digit)
            {
                return None;
            }
            second = tail[at + 1..at + 3].parse::<u32>().ok()?;
            at += 3;
        }
        let mut fraction_is_zero = true;
        if tail.as_bytes().get(at) == Some(&b'.') {
            if !has_seconds {
                return None;
            }
            at += 1;
            let start = at;
            while tail.as_bytes().get(at).is_some_and(u8::is_ascii_digit) {
                fraction_is_zero &= tail.as_bytes()[at] == b'0';
                at += 1;
            }
            if at == start {
                return None;
            }
        }
        if minute > 59 || second > 59 || hour > 24 {
            return None;
        }
        if hour == 24 {
            if minute != 0 || second != 0 || !fraction_is_zero {
                return None;
            }
            calendar = calendar.succ_opt()?;
            tail.replace_range(1..3, "00");
        }
        if !has_seconds {
            tail.insert_str(6, ":00");
        }
    } else if !tail.is_empty() {
        return None;
    }
    Some(format!("{}{}", calendar.format("%Y-%m-%d"), tail))
}
fn normalized_analysis_path(value: &str) -> Option<String> {
    if value.is_empty() {
        return None;
    }
    let mut units: Vec<u16> = value.encode_utf16().collect();
    units.truncate(units.len().saturating_sub(4));
    // A dangling UTF16 surrogate is not a representable native path. Do not
    // silently manufacture a different path containing a replacement character.
    String::from_utf16(&units).ok()
}
fn date(value: &str, policy: DatePolicy) -> Value {
    let Some(normalized) = normalize_iso_calendar(value) else {
        return Value::Null;
    };
    let value = normalized.as_str();
    let parsed = DateTime::parse_from_rfc3339(value)
        .map(|x| x.with_timezone(&Utc))
        .ok()
        .or_else(|| {
            NaiveDate::parse_from_str(value, "%Y-%m-%d")
                .ok()
                .and_then(|d| d.and_hms_opt(0, 0, 0))
                .map(|d| d.and_utc())
        })
        .or_else(|| {
            [
                "%Y-%m-%d %H:%M:%S%.f",
                "%Y-%m-%dT%H:%M:%S%.f",
                "%Y-%m-%dT%H:%M",
                "%Y-%m-%d %H:%M",
            ]
            .iter()
            .find_map(|fmt| {
                NaiveDateTime::parse_from_str(value, fmt)
                    .ok()
                    .and_then(|d| match policy {
                        DatePolicy::Utc => Some(d.and_utc()),
                        DatePolicy::Local => resolve_local(d),
                    })
            })
        });
    parsed.map_or(Value::Null, |d| {
        Value::String(d.to_rfc3339_opts(SecondsFormat::Millis, true))
    })
}
fn track(r: &Value, policy: DatePolicy) -> Result<Track> {
    Ok(Track {
        id: integer(r, "content_id"),
        title: text(r, "title"),
        duration: number(r, "length"),
        tempo: number(r, "bpmx100") / 100.,
        rating: number(r, "rating"),
        comment: text(r, "djComment"),
        file_path: text(r, "path"),
        file_name: text(r, "fileName"),
        bitrate: r["bitrate"].as_f64(),
        track_number: r["trackNo"].as_f64(),
        disc_number: r["discNo"].as_f64(),
        sample_rate: r["samplingRate"].as_f64(),
        sample_depth: r["bitDepth"].as_f64(),
        play_count: r["djPlayCount"].as_f64(),
        year: r["releaseYear"].as_f64(),
        mix_name: text_opt(r, "subtitle"),
        autoload_hotcues: number(r, "isHotCueAutoLoadOn") != 0.,
        kuvo_public: number(r, "isKuvoDeliverStatusOn") != 0.,
        file_size: r["fileSize"].as_f64(),
        analyze_path: text_opt(r, "analysisDataFilePath")
            .and_then(|s| normalized_analysis_path(&s)),
        release_date: text_opt(r, "releaseDate"),
        date_added: text_opt(r, "dateAdded")
            .filter(|s| !s.is_empty())
            .map(|s| date(&s, policy)),
        analyze_date: None,
        cue_and_loops: None,
        artwork: {
            let id = integer(r, "image_id");
            (id != 0).then(|| Artwork {
                id,
                path: text_opt(r, "artworkPath"),
            })
        },
        artist: relation(r, "artist_id_artist", "artistName"),
        original_artist: relation(r, "artist_id_originalArtist", "originalArtistName"),
        remixer: relation(r, "artist_id_remixer", "remixerName"),
        composer: relation(r, "artist_id_composer", "composerName"),
        album: relation(r, "album_id", "albumName"),
        label: relation(r, "label_id", "labelName"),
        genre: relation(r, "genre_id", "genreName"),
        color: relation(r, "color_id", "colorName"),
        key: relation(r, "key_id", "keyName"),
    })
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn dates_preserve_utc_policy_iso_offsets_and_invalid_null() {
        assert_eq!(
            date("2024-01-02T03:04:05.678+02:30", DatePolicy::Utc),
            json!("2024-01-02T00:34:05.678Z")
        );
        assert_eq!(
            date("2024-01-02 03:04:05", DatePolicy::Utc),
            json!("2024-01-02T03:04:05.000Z")
        );
        assert_eq!(
            date("2024-01-02", DatePolicy::Local),
            json!("2024-01-02T00:00:00.000Z")
        );
        assert_eq!(date("invalid", DatePolicy::Local), Value::Null);
    }
    #[cfg(unix)]
    #[test]
    fn dates_follow_js_local_timezone_and_dst() {
        if std::env::var_os("ONELIBRARY_DATE_CHILD").is_none() {
            let output = std::process::Command::new(std::env::current_exe().unwrap())
                .args([
                    "--exact",
                    "adapter::tests::dates_follow_js_local_timezone_and_dst",
                ])
                .env("ONELIBRARY_DATE_CHILD", "1")
                .env("TZ", "America/New_York")
                .output()
                .unwrap();
            assert!(
                output.status.success(),
                "isolated local timezone fixture failed: {}",
                String::from_utf8_lossy(&output.stdout)
            );
            return;
        }
        assert_eq!(
            date("2024-01-02T03:04:05", DatePolicy::Local),
            json!("2024-01-02T08:04:05.000Z")
        );
        assert_eq!(
            date("2024-03-10T02:30:00", DatePolicy::Local),
            json!("2024-03-10T07:30:00.000Z")
        );
        assert_eq!(
            date("2024-11-03T01:30:00", DatePolicy::Local),
            json!("2024-11-03T05:30:00.000Z")
        );
    }
}
#[cfg(test)]
mod calendar_reference_tests {
    use super::*;
    #[test]
    fn iso_calendar_rollover_and_24_hours_match_original_ts_adapter() {
        let cases: Value =
            serde_json::from_str(include_str!("../fixtures/expected-calendar.json")).unwrap();
        assert_eq!(cases["dates"].as_array().unwrap().len(), 23);
        for case in cases["dates"].as_array().unwrap() {
            assert_eq!(
                date(case["input"].as_str().unwrap(), DatePolicy::Utc),
                case["result"],
                "calendar input={}",
                case["input"]
            );
        }
    }
    #[test]
    fn utf16_truncation_omits_unrepresentable_paths_without_replacement_characters() {
        let cases: Value =
            serde_json::from_str(include_str!("../fixtures/expected-calendar.json")).unwrap();
        for case in cases["paths"].as_array().unwrap() {
            let actual = normalized_analysis_path(case["input"].as_str().unwrap());
            assert_eq!(json!(actual), case["nativePath"]);
            if let Some(path) = actual {
                assert!(!path.contains('\u{fffd}'));
                assert_eq!(
                    path.encode_utf16().collect::<Vec<_>>(),
                    serde_json::from_value::<Vec<u16>>(case["jsCodeUnits"].clone()).unwrap()
                );
            }
        }
    }
}
