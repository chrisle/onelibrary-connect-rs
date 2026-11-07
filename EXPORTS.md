# Pinned TypeScript export mapping

| TypeScript export | Rust equivalent |
|---|---|
| OneLibraryAdapter | OneLibraryAdapter::open / open_with_options |
| type property | database_type() -> DatabaseType::OneLibrary |
| close | close, plus automatic Drop |
| findTrack / findAllTracks | find_track / find_all_tracks |
| findCues | find_cues |
| findPlaylistById / findPlaylist / findPlaylistContents | find_playlist_by_id / find_playlist / find_playlist_contents |
| findArtist / findAlbum / findGenre / findKey | find_artist / find_album / find_genre / find_key |
| findColor / findLabel / findArtwork | find_color / find_label / find_artwork |
| findMyTags / findMyTagById / findMyTagContents / findMyTagsForTrack | find_my_tags / find_my_tag_by_id / find_my_tag_contents / find_my_tags_for_track |
| findHistorySessions / findHistoryContents | find_history_sessions / find_history_contents |
| findHotCueBankLists / findHotCueBankListCues | find_hot_cue_bank_lists / find_hot_cue_bank_list_cues |
| findMenuItems / findVisibleCategories / findVisibleSortOptions | find_menu_items / find_visible_categories / find_visible_sort_options |
| getProperty | get_property |
| openOneLibraryDb / getEncryptionKey | open_one_library_db / get_encryption_key (UTF8 key bytes) |
| DatabaseAdapter / DatabasePreference / DatabaseType / PlaylistQueryResult | same exported Rust names |
| EntityFK | same variants; explicit Rust type forms below |
| Album / Artist / Artwork / Color / Genre / Key / Label / Playlist | same exported Rust names (named references share NamedEntity) |
| Track WithRelations / WithFKs | Track / TrackFks |
| PlaylistEntry WithRelations / WithFKs | PlaylistEntryRelations (PlaylistEntry alias) / PlaylistEntryFks |
| CueColor / HotcueButton | associated numeric constants, source spelling retained |
| CueAndLoop / CuePoint / Loop / Hotcue / Hotloop | CueAndLoop tagged union, convenience aliases; variants carry their corresponding fields |
| Category / DeviceProperty / HistorySession / HotCueBankList / MenuItem / MyTag / SortOption | same exported Rust names |
| CueKind / MenuItemKind / MyTagAttribute / PlaylistAttribute | same associated constant groups |
| AlbumRow / ArtistRow / CategoryRow / ColorRow / ContentRow / CueRow / GenreRow / HistoryContentRow / HistoryRow / HotCueBankListCueRow / HotCueBankListRow / ImageRow / KeyRow / LabelRow / MenuItemRow / MyTagContentRow / MyTagRow / PlaylistContentRow / PlaylistRow / PropertyRow / RecommendedLikeRow / SortRow | same exported Rust structs; snake_case fields, exact source serde column names |

Rust methods use explicit Result/Option rather than JS throws/undefined.
No reference-table mutation or public database write API exists in the source
library and none is added to the adapter. The low-level returned SQLite handle
remains read-only; set_key is an additional low-level utility for explicit
fixture generation, not an adapter write operation.
