use super::error;
use reqwest;
use std::collections::HashMap;

use derive_builder::Builder;

#[macro_use]
use derive_builder;
use serde::{Deserialize, Serialize};

use super::api::Api;

/// Getting log information
#[derive(Debug, Builder, Default)]
pub struct LogRequest {
    #[builder(default)]
    normal: bool,
    #[builder(default)]
    info: bool,
    #[builder(default)]
    warning: bool,
    #[builder(default)]
    critical: bool,
    #[builder(default)]
    last_known_id: u64,
}

// TODO: use serde here instead
impl LogRequest {
    pub(crate) fn url(&self) -> String {
        format! {"normal={}&info={}&warning={}&critical={}&last_known_id={}", self.normal, self.info, self.warning, self.critical, self.last_known_id}
    }
}

/// filter optional 	Filter torrent list. Allowed filters: all, downloading, completed, paused, active, inactive, 'resumed'
/// category optional 	Get torrents with the given category (empty string means "without category"; no "category" parameter means "any category")
/// sort optional 	Sort torrents by given key. All the possible keys are listed here below
/// reverse optional 	Enable reverse sorting. Possible values are true and false (default)
/// limit optional 	Limit the number of torrents returned
/// offset optional 	Set offset (if less than 0, offset from end)
/// hashes optional 	Filter by hashes. Can contain multiple hashes separated by |
#[derive(Debug, Builder, Serialize, Deserialize, Clone)]
struct TorrentRequest {
    filter: Option<TorrentFilter>,
    category: Option<String>,
    sort: Option<String>,
    reverse: Option<bool>,
    limit: Option<u64>,
    offset: Option<i64>,
    hashes: Option<Vec<String>>,
}
impl TorrentRequest {
    // TODO: swap this to www_url_encoding crate
    fn url(&self) -> Result<String, error::Error> {
        let url = serde_urlencoded::to_string(&self)?;
        Ok(url)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
enum TorrentFilter {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "downloading")]
    Downloading,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "paused")]
    Paused,
    #[serde(rename = "active")]
    Active,
}

/// Metadata for downloading magnet links and torrent files
///
/// NOTE: You must include either a `urls` field or `torrents` field
///
/// urls 	string 	URLs separated with newlines
/// torrents 	raw 	Raw data of torrent file. torrents can be presented multiple times.
/// savepath optional 	string 	Download folder
/// cookie optional 	string 	Cookie sent to download the .torrent file
/// category optional 	string 	Category for the torrent
/// skip_checking optional 	string 	Skip hash checking. Possible values are true, false (default)
/// paused optional 	string 	Add torrents in the paused state. Possible values are true, false (default)
/// root_folder optional 	string 	Create the root folder. Possible values are true, false, unset (default)
/// rename optional 	string 	Rename torrent
/// upLimit optional 	integer 	Set torrent upload speed limit. Unit in bytes/second
/// dlLimit optional 	integer 	Set torrent download speed limit. Unit in bytes/second
/// autoTMM optional 	bool 	Whether Automatic Torrent Management should be used
/// sequentialDownload optional 	string 	Enable sequential download. Possible values are true, false (default)
/// firstLastPiecePrio optional 	string 	Prioritize download first last piece. Possible values are true, false (default)
#[derive(Debug, Clone, Deserialize, Serialize, Builder, Default)]
#[builder(setter(into, strip_option))]
pub struct TorrentDownload {
    #[builder(default)]
    urls: Option<String>,
    #[builder(default)]
    torrents: Option<Vec<u8>>,
    #[builder(default)]
    savepath: Option<String>,
    #[builder(default)]
    cookie: Option<String>,
    #[builder(default)]
    category: Option<String>,
    #[builder(default)]
    skip_checking: Option<String>,
    #[builder(default)]
    paused: Option<String>,
    #[builder(default)]
    root_folder: Option<String>,
    #[builder(default)]
    rename: Option<String>,
    #[builder(default)]
    #[serde(rename = "upLimit")]
    upload_limit: Option<i64>,
    #[builder(default)]
    #[serde(rename = "dlLimit")]
    download_limit: Option<i64>,
    #[builder(default)]
    #[serde(rename = "autoTMM")]
    automatic_management: Option<bool>,
    #[builder(default)]
    #[serde(rename = "sequentialDownload")]
    sequential_download: Option<String>,
    #[builder(default)]
    #[serde(rename = "firstLastPiecePrio")]
    first_last_piece_prio: Option<String>,
}

impl TorrentDownload {
    pub async fn download(&self, api: &Api) -> Result<(), error::Error> {
        api.add_new_torrent(&self).await
    }
}
