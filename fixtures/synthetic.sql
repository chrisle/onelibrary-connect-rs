-- Synthetic mock database schema/data from pinned OneLibrary fixture, MIT.
CREATE TABLE album (
    album_id INTEGER PRIMARY KEY,
    name TEXT,
    artist_id INTEGER,
    image_id INTEGER,
    isComplation INTEGER,
    nameForSearch TEXT
  );
INSERT INTO "album"("album_id","name","artist_id","image_id","isComplation","nameForSearch") VALUES(1,'Test Album',1,1,0,'test album');
INSERT INTO "album"("album_id","name","artist_id","image_id","isComplation","nameForSearch") VALUES(2,'Another Album',2,NULL,0,'another album');
CREATE TABLE artist (
    artist_id INTEGER PRIMARY KEY,
    name TEXT,
    nameForSearch TEXT
  );
INSERT INTO "artist"("artist_id","name","nameForSearch") VALUES(1,'Test Artist','test artist');
INSERT INTO "artist"("artist_id","name","nameForSearch") VALUES(2,'Another Artist','another artist');
INSERT INTO "artist"("artist_id","name","nameForSearch") VALUES(3,'Remixer One','remixer one');
CREATE TABLE category (
    category_id INTEGER PRIMARY KEY,
    menuItem_id INTEGER,
    sequenceNo INTEGER,
    isVisible INTEGER
  );
INSERT INTO "category"("category_id","menuItem_id","sequenceNo","isVisible") VALUES(1,1,1,1);
INSERT INTO "category"("category_id","menuItem_id","sequenceNo","isVisible") VALUES(2,2,2,1);
INSERT INTO "category"("category_id","menuItem_id","sequenceNo","isVisible") VALUES(3,3,3,1);
INSERT INTO "category"("category_id","menuItem_id","sequenceNo","isVisible") VALUES(4,4,4,0);
INSERT INTO "category"("category_id","menuItem_id","sequenceNo","isVisible") VALUES(5,5,5,1);
CREATE TABLE color (
    color_id INTEGER PRIMARY KEY,
    name TEXT
  );
INSERT INTO "color"("color_id","name") VALUES(1,'Pink');
INSERT INTO "color"("color_id","name") VALUES(2,'Red');
INSERT INTO "color"("color_id","name") VALUES(3,'Orange');
INSERT INTO "color"("color_id","name") VALUES(4,'Yellow');
INSERT INTO "color"("color_id","name") VALUES(5,'Green');
CREATE TABLE content (
    content_id INTEGER PRIMARY KEY,
    title TEXT,
    titleForSearch TEXT,
    subtitle TEXT,
    bpmx100 INTEGER,
    length INTEGER,
    trackNo INTEGER,
    discNo INTEGER,
    artist_id_artist INTEGER,
    artist_id_remixer INTEGER,
    artist_id_originalArtist INTEGER,
    artist_id_composer INTEGER,
    artist_id_lyricist INTEGER,
    album_id INTEGER,
    genre_id INTEGER,
    label_id INTEGER,
    key_id INTEGER,
    color_id INTEGER,
    image_id INTEGER,
    djComment TEXT,
    rating INTEGER,
    releaseYear INTEGER,
    releaseDate TEXT,
    dateCreated TEXT,
    dateAdded TEXT,
    path TEXT,
    fileName TEXT,
    fileSize INTEGER,
    fileType INTEGER,
    bitrate INTEGER,
    bitDepth INTEGER,
    samplingRate INTEGER,
    isrc TEXT,
    djPlayCount INTEGER,
    isHotCueAutoLoadOn INTEGER,
    isKuvoDeliverStatusOn INTEGER,
    kuvoDeliveryComment TEXT,
    masterDbId INTEGER,
    masterContentId INTEGER,
    analysisDataFilePath TEXT,
    analysedBits INTEGER,
    contentLink INTEGER,
    hasModified INTEGER,
    cueUpdateCount INTEGER,
    analysisDataUpdateCount INTEGER,
    informationUpdateCount INTEGER
  );
INSERT INTO "content"("content_id","title","titleForSearch","subtitle","bpmx100","length","trackNo","discNo","artist_id_artist","artist_id_remixer","artist_id_originalArtist","artist_id_composer","artist_id_lyricist","album_id","genre_id","label_id","key_id","color_id","image_id","djComment","rating","releaseYear","releaseDate","dateCreated","dateAdded","path","fileName","fileSize","fileType","bitrate","bitDepth","samplingRate","isrc","djPlayCount","isHotCueAutoLoadOn","isKuvoDeliverStatusOn","kuvoDeliveryComment","masterDbId","masterContentId","analysisDataFilePath","analysedBits","contentLink","hasModified","cueUpdateCount","analysisDataUpdateCount","informationUpdateCount") VALUES(1,'Test Track','test track','Extended Mix',12800,300,1,1,1,3,NULL,NULL,NULL,1,1,1,1,1,1,'Test comment',5,2024,'2024-01-01','2024-01-01','2024-01-01','/Music/test.mp3','test.mp3',5000000,1,320,16,44100,NULL,10,1,1,NULL,NULL,NULL,'/PIONEER/USBANLZ/P001/0001/ANLZ0000',NULL,NULL,0,0,0,0);
INSERT INTO "content"("content_id","title","titleForSearch","subtitle","bpmx100","length","trackNo","discNo","artist_id_artist","artist_id_remixer","artist_id_originalArtist","artist_id_composer","artist_id_lyricist","album_id","genre_id","label_id","key_id","color_id","image_id","djComment","rating","releaseYear","releaseDate","dateCreated","dateAdded","path","fileName","fileSize","fileType","bitrate","bitDepth","samplingRate","isrc","djPlayCount","isHotCueAutoLoadOn","isKuvoDeliverStatusOn","kuvoDeliveryComment","masterDbId","masterContentId","analysisDataFilePath","analysedBits","contentLink","hasModified","cueUpdateCount","analysisDataUpdateCount","informationUpdateCount") VALUES(2,'Another Track','another track',NULL,14000,240,2,1,2,NULL,NULL,NULL,NULL,2,2,NULL,2,2,NULL,NULL,4,2023,'2023-06-15','2023-06-15','2023-06-15','/Music/another.mp3','another.mp3',4000000,1,256,16,48000,NULL,5,0,0,NULL,NULL,NULL,'/PIONEER/USBANLZ/P002/0001/ANLZ0000',NULL,NULL,0,0,0,0);
INSERT INTO "content"("content_id","title","titleForSearch","subtitle","bpmx100","length","trackNo","discNo","artist_id_artist","artist_id_remixer","artist_id_originalArtist","artist_id_composer","artist_id_lyricist","album_id","genre_id","label_id","key_id","color_id","image_id","djComment","rating","releaseYear","releaseDate","dateCreated","dateAdded","path","fileName","fileSize","fileType","bitrate","bitDepth","samplingRate","isrc","djPlayCount","isHotCueAutoLoadOn","isKuvoDeliverStatusOn","kuvoDeliveryComment","masterDbId","masterContentId","analysisDataFilePath","analysedBits","contentLink","hasModified","cueUpdateCount","analysisDataUpdateCount","informationUpdateCount") VALUES(3,'Unknown Track','unknown track',NULL,0,180,1,1,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,0,NULL,NULL,NULL,'2024-01-15','/Music/unknown.mp3','unknown.mp3',3000000,1,128,16,44100,NULL,0,0,0,NULL,NULL,NULL,NULL,NULL,NULL,0,0,0,0);
INSERT INTO "content"("content_id","title","titleForSearch","subtitle","bpmx100","length","trackNo","discNo","artist_id_artist","artist_id_remixer","artist_id_originalArtist","artist_id_composer","artist_id_lyricist","album_id","genre_id","label_id","key_id","color_id","image_id","djComment","rating","releaseYear","releaseDate","dateCreated","dateAdded","path","fileName","fileSize","fileType","bitrate","bitDepth","samplingRate","isrc","djPlayCount","isHotCueAutoLoadOn","isKuvoDeliverStatusOn","kuvoDeliveryComment","masterDbId","masterContentId","analysisDataFilePath","analysedBits","contentLink","hasModified","cueUpdateCount","analysisDataUpdateCount","informationUpdateCount") VALUES(4,'Playlist Track A','playlist track a',NULL,13000,210,3,1,1,NULL,NULL,NULL,NULL,1,1,1,3,3,1,NULL,3,2024,NULL,NULL,'2024-01-10','/Music/playlist_a.mp3','playlist_a.mp3',4500000,1,320,16,44100,NULL,2,1,0,NULL,NULL,NULL,'/PIONEER/USBANLZ/P003/0001/ANLZ0000',NULL,NULL,0,0,0,0);
INSERT INTO "content"("content_id","title","titleForSearch","subtitle","bpmx100","length","trackNo","discNo","artist_id_artist","artist_id_remixer","artist_id_originalArtist","artist_id_composer","artist_id_lyricist","album_id","genre_id","label_id","key_id","color_id","image_id","djComment","rating","releaseYear","releaseDate","dateCreated","dateAdded","path","fileName","fileSize","fileType","bitrate","bitDepth","samplingRate","isrc","djPlayCount","isHotCueAutoLoadOn","isKuvoDeliverStatusOn","kuvoDeliveryComment","masterDbId","masterContentId","analysisDataFilePath","analysedBits","contentLink","hasModified","cueUpdateCount","analysisDataUpdateCount","informationUpdateCount") VALUES(5,'Playlist Track B','playlist track b',NULL,13500,225,4,1,2,NULL,NULL,NULL,NULL,2,2,2,1,4,2,NULL,4,2024,NULL,NULL,'2024-01-10','/Music/playlist_b.mp3','playlist_b.mp3',4800000,1,320,16,44100,NULL,3,1,0,NULL,NULL,NULL,'/PIONEER/USBANLZ/P004/0001/ANLZ0000',NULL,NULL,0,0,0,0);
CREATE TABLE cue (
    cue_id INTEGER PRIMARY KEY,
    content_id INTEGER,
    kind INTEGER,
    colorTableIndex INTEGER,
    cueComment TEXT,
    isActiveLoop INTEGER,
    beatLoopNumerator INTEGER,
    beatLoopDenominator INTEGER,
    inUsec INTEGER,
    outUsec INTEGER,
    in150FramePerSec INTEGER,
    out150FramePerSec INTEGER,
    inMpegFrameNumber INTEGER,
    outMpegFrameNumber INTEGER,
    inMpegAbs INTEGER,
    outMpegAbs INTEGER,
    inDecodingStartFramePosition INTEGER,
    outDecodingStartFramePosition INTEGER,
    inFileOffsetInBlock INTEGER,
    OutFileOffsetInBlock INTEGER,
    inNumberOfSampleInBlock INTEGER,
    outNumberOfSampleInBlock INTEGER
  );
INSERT INTO "cue"("cue_id","content_id","kind","colorTableIndex","cueComment","isActiveLoop","beatLoopNumerator","beatLoopDenominator","inUsec","outUsec","in150FramePerSec","out150FramePerSec","inMpegFrameNumber","outMpegFrameNumber","inMpegAbs","outMpegAbs","inDecodingStartFramePosition","outDecodingStartFramePosition","inFileOffsetInBlock","OutFileOffsetInBlock","inNumberOfSampleInBlock","outNumberOfSampleInBlock") VALUES(1,1,0,NULL,'Start',0,NULL,NULL,0,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL);
INSERT INTO "cue"("cue_id","content_id","kind","colorTableIndex","cueComment","isActiveLoop","beatLoopNumerator","beatLoopDenominator","inUsec","outUsec","in150FramePerSec","out150FramePerSec","inMpegFrameNumber","outMpegFrameNumber","inMpegAbs","outMpegAbs","inDecodingStartFramePosition","outDecodingStartFramePosition","inFileOffsetInBlock","OutFileOffsetInBlock","inNumberOfSampleInBlock","outNumberOfSampleInBlock") VALUES(2,1,1,2,'Drop',0,NULL,NULL,32000000,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL);
INSERT INTO "cue"("cue_id","content_id","kind","colorTableIndex","cueComment","isActiveLoop","beatLoopNumerator","beatLoopDenominator","inUsec","outUsec","in150FramePerSec","out150FramePerSec","inMpegFrameNumber","outMpegFrameNumber","inMpegAbs","outMpegAbs","inDecodingStartFramePosition","outDecodingStartFramePosition","inFileOffsetInBlock","OutFileOffsetInBlock","inNumberOfSampleInBlock","outNumberOfSampleInBlock") VALUES(3,1,0,NULL,'Build',1,4,1,64000000,80000000,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL);
INSERT INTO "cue"("cue_id","content_id","kind","colorTableIndex","cueComment","isActiveLoop","beatLoopNumerator","beatLoopDenominator","inUsec","outUsec","in150FramePerSec","out150FramePerSec","inMpegFrameNumber","outMpegFrameNumber","inMpegAbs","outMpegAbs","inDecodingStartFramePosition","outDecodingStartFramePosition","inFileOffsetInBlock","OutFileOffsetInBlock","inNumberOfSampleInBlock","outNumberOfSampleInBlock") VALUES(4,1,2,3,'Hook',1,2,1,128000000,136000000,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL);
INSERT INTO "cue"("cue_id","content_id","kind","colorTableIndex","cueComment","isActiveLoop","beatLoopNumerator","beatLoopDenominator","inUsec","outUsec","in150FramePerSec","out150FramePerSec","inMpegFrameNumber","outMpegFrameNumber","inMpegAbs","outMpegAbs","inDecodingStartFramePosition","outDecodingStartFramePosition","inFileOffsetInBlock","OutFileOffsetInBlock","inNumberOfSampleInBlock","outNumberOfSampleInBlock") VALUES(5,4,1,1,'Intro',0,NULL,NULL,1000000,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL);
INSERT INTO "cue"("cue_id","content_id","kind","colorTableIndex","cueComment","isActiveLoop","beatLoopNumerator","beatLoopDenominator","inUsec","outUsec","in150FramePerSec","out150FramePerSec","inMpegFrameNumber","outMpegFrameNumber","inMpegAbs","outMpegAbs","inDecodingStartFramePosition","outDecodingStartFramePosition","inFileOffsetInBlock","OutFileOffsetInBlock","inNumberOfSampleInBlock","outNumberOfSampleInBlock") VALUES(6,4,2,4,'Main Loop',1,8,1,60000000,90000000,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL);
CREATE TABLE genre (
    genre_id INTEGER PRIMARY KEY,
    name TEXT
  );
INSERT INTO "genre"("genre_id","name") VALUES(1,'Electronic');
INSERT INTO "genre"("genre_id","name") VALUES(2,'House');
INSERT INTO "genre"("genre_id","name") VALUES(3,'Techno');
CREATE TABLE history (
    history_id INTEGER PRIMARY KEY,
    sequenceNo INTEGER,
    name TEXT,
    attribute INTEGER,
    history_id_parent INTEGER
  );
INSERT INTO "history"("history_id","sequenceNo","name","attribute","history_id_parent") VALUES(1,1,'2024-01-01',0,NULL);
INSERT INTO "history"("history_id","sequenceNo","name","attribute","history_id_parent") VALUES(2,2,'2024-01-02',0,NULL);
INSERT INTO "history"("history_id","sequenceNo","name","attribute","history_id_parent") VALUES(3,3,'2024-01-03',0,NULL);
CREATE TABLE history_content (
    history_id INTEGER,
    content_id INTEGER,
    sequenceNo INTEGER
  );
INSERT INTO "history_content"("history_id","content_id","sequenceNo") VALUES(1,1,1);
INSERT INTO "history_content"("history_id","content_id","sequenceNo") VALUES(1,2,2);
INSERT INTO "history_content"("history_id","content_id","sequenceNo") VALUES(1,4,3);
INSERT INTO "history_content"("history_id","content_id","sequenceNo") VALUES(2,4,1);
INSERT INTO "history_content"("history_id","content_id","sequenceNo") VALUES(2,5,2);
INSERT INTO "history_content"("history_id","content_id","sequenceNo") VALUES(2,1,3);
INSERT INTO "history_content"("history_id","content_id","sequenceNo") VALUES(3,2,1);
CREATE TABLE hotCueBankList (
    hotCueBankList_id INTEGER PRIMARY KEY,
    sequenceNo INTEGER,
    name TEXT,
    image_id INTEGER,
    attribute INTEGER,
    hotCueBankList_id_parent INTEGER
  );
INSERT INTO "hotCueBankList"("hotCueBankList_id","sequenceNo","name","image_id","attribute","hotCueBankList_id_parent") VALUES(1,1,'Bank A',NULL,0,NULL);
INSERT INTO "hotCueBankList"("hotCueBankList_id","sequenceNo","name","image_id","attribute","hotCueBankList_id_parent") VALUES(2,2,'Bank B',NULL,0,NULL);
CREATE TABLE hotCueBankList_cue (
    hotCueBankList_id INTEGER,
    cue_id INTEGER,
    sequenceNo INTEGER
  );
INSERT INTO "hotCueBankList_cue"("hotCueBankList_id","cue_id","sequenceNo") VALUES(1,2,1);
INSERT INTO "hotCueBankList_cue"("hotCueBankList_id","cue_id","sequenceNo") VALUES(1,4,2);
INSERT INTO "hotCueBankList_cue"("hotCueBankList_id","cue_id","sequenceNo") VALUES(2,5,1);
INSERT INTO "hotCueBankList_cue"("hotCueBankList_id","cue_id","sequenceNo") VALUES(2,6,2);
CREATE TABLE image (
    image_id INTEGER PRIMARY KEY,
    path TEXT
  );
INSERT INTO "image"("image_id","path") VALUES(1,'/PIONEER/USBANLZ/P001/0001/artwork.jpg');
INSERT INTO "image"("image_id","path") VALUES(2,'/PIONEER/USBANLZ/P002/0001/artwork.jpg');
CREATE TABLE key (
    key_id INTEGER PRIMARY KEY,
    name TEXT
  );
INSERT INTO "key"("key_id","name") VALUES(1,'Am');
INSERT INTO "key"("key_id","name") VALUES(2,'C');
INSERT INTO "key"("key_id","name") VALUES(3,'Fm');
CREATE TABLE label (
    label_id INTEGER PRIMARY KEY,
    name TEXT
  );
INSERT INTO "label"("label_id","name") VALUES(1,'Test Label');
INSERT INTO "label"("label_id","name") VALUES(2,'Another Label');
CREATE TABLE menuItem (
    menuItem_id INTEGER PRIMARY KEY,
    kind INTEGER,
    name TEXT
  );
INSERT INTO "menuItem"("menuItem_id","kind","name") VALUES(1,128,'Genre');
INSERT INTO "menuItem"("menuItem_id","kind","name") VALUES(2,129,'Artist');
INSERT INTO "menuItem"("menuItem_id","kind","name") VALUES(3,130,'Album');
INSERT INTO "menuItem"("menuItem_id","kind","name") VALUES(4,131,'Track');
INSERT INTO "menuItem"("menuItem_id","kind","name") VALUES(5,139,'Key');
CREATE TABLE myTag (
    myTag_id INTEGER PRIMARY KEY,
    sequenceNo INTEGER,
    name TEXT,
    attribute INTEGER,
    myTag_id_parent INTEGER
  );
INSERT INTO "myTag"("myTag_id","sequenceNo","name","attribute","myTag_id_parent") VALUES(1,1,'Favorites',0,NULL);
INSERT INTO "myTag"("myTag_id","sequenceNo","name","attribute","myTag_id_parent") VALUES(2,2,'Energy',1,NULL);
INSERT INTO "myTag"("myTag_id","sequenceNo","name","attribute","myTag_id_parent") VALUES(3,1,'High Energy',0,2);
INSERT INTO "myTag"("myTag_id","sequenceNo","name","attribute","myTag_id_parent") VALUES(4,2,'Low Energy',0,2);
INSERT INTO "myTag"("myTag_id","sequenceNo","name","attribute","myTag_id_parent") VALUES(5,3,'Classics',0,NULL);
CREATE TABLE myTag_content (
    myTag_id INTEGER,
    content_id INTEGER
  );
INSERT INTO "myTag_content"("myTag_id","content_id") VALUES(1,1);
INSERT INTO "myTag_content"("myTag_id","content_id") VALUES(1,4);
INSERT INTO "myTag_content"("myTag_id","content_id") VALUES(3,1);
INSERT INTO "myTag_content"("myTag_id","content_id") VALUES(3,4);
INSERT INTO "myTag_content"("myTag_id","content_id") VALUES(4,2);
INSERT INTO "myTag_content"("myTag_id","content_id") VALUES(5,1);
INSERT INTO "myTag_content"("myTag_id","content_id") VALUES(5,2);
CREATE TABLE playlist (
    playlist_id INTEGER PRIMARY KEY,
    sequenceNo INTEGER,
    name TEXT,
    image_id INTEGER,
    attribute INTEGER,
    playlist_id_parent INTEGER
  );
INSERT INTO "playlist"("playlist_id","sequenceNo","name","image_id","attribute","playlist_id_parent") VALUES(1,1,'My Favorites',NULL,0,NULL);
INSERT INTO "playlist"("playlist_id","sequenceNo","name","image_id","attribute","playlist_id_parent") VALUES(2,2,'DJ Sets',NULL,1,NULL);
INSERT INTO "playlist"("playlist_id","sequenceNo","name","image_id","attribute","playlist_id_parent") VALUES(3,1,'Club Night',NULL,0,2);
INSERT INTO "playlist"("playlist_id","sequenceNo","name","image_id","attribute","playlist_id_parent") VALUES(4,2,'Festival',NULL,0,2);
INSERT INTO "playlist"("playlist_id","sequenceNo","name","image_id","attribute","playlist_id_parent") VALUES(5,3,'Empty Playlist',NULL,0,NULL);
CREATE TABLE playlist_content (
    playlist_id INTEGER,
    content_id INTEGER,
    sequenceNo INTEGER
  );
INSERT INTO "playlist_content"("playlist_id","content_id","sequenceNo") VALUES(1,1,1);
INSERT INTO "playlist_content"("playlist_id","content_id","sequenceNo") VALUES(1,2,2);
INSERT INTO "playlist_content"("playlist_id","content_id","sequenceNo") VALUES(1,4,3);
INSERT INTO "playlist_content"("playlist_id","content_id","sequenceNo") VALUES(3,1,1);
INSERT INTO "playlist_content"("playlist_id","content_id","sequenceNo") VALUES(3,4,2);
INSERT INTO "playlist_content"("playlist_id","content_id","sequenceNo") VALUES(3,5,3);
INSERT INTO "playlist_content"("playlist_id","content_id","sequenceNo") VALUES(4,2,1);
INSERT INTO "playlist_content"("playlist_id","content_id","sequenceNo") VALUES(4,5,2);
CREATE TABLE property (
    deviceName TEXT,
    dbVersion TEXT,
    numberOfContents INTEGER,
    createdDate TEXT,
    backGroundColorType INTEGER,
    myTagMasterDBID INTEGER
  );
INSERT INTO "property"("deviceName","dbVersion","numberOfContents","createdDate","backGroundColorType","myTagMasterDBID") VALUES('Test Device','1.0.0',5,'2024-01-01 00:00:00',0,NULL);
CREATE TABLE sort (
    sort_id INTEGER PRIMARY KEY,
    menuItem_id INTEGER,
    sequenceNo INTEGER,
    isVisible INTEGER,
    isSelectedAsSubColumn INTEGER
  );
INSERT INTO "sort"("sort_id","menuItem_id","sequenceNo","isVisible","isSelectedAsSubColumn") VALUES(1,1,1,1,0);
INSERT INTO "sort"("sort_id","menuItem_id","sequenceNo","isVisible","isSelectedAsSubColumn") VALUES(2,2,2,1,1);
INSERT INTO "sort"("sort_id","menuItem_id","sequenceNo","isVisible","isSelectedAsSubColumn") VALUES(3,3,3,0,0);
INSERT INTO "sort"("sort_id","menuItem_id","sequenceNo","isVisible","isSelectedAsSubColumn") VALUES(4,4,4,1,0);
