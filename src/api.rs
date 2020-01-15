use super::error;
use reqwest;
use std::collections::HashMap;

use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_json;
use serde_urlencoded;
use tokio;

macro_rules! push_own {
    ($s:ident . $inner:ident, $( $push_val:expr),+) => {
        {
            let mut save = $s.$inner.clone().to_string();
            $(
                save.push_str($push_val);
            )+
            save
        }
    };
}

#[derive(Debug)]
pub struct Api {
    cookie: String,
    address: String,
    client: reqwest::Client,
}

impl Api {
    pub async fn new(username: &str, password: &str, address: &str) -> Result<Self, error::Error> {
        let mut form = HashMap::new();
        form.insert("username", username);
        form.insert("password", password);

        let client = reqwest::Client::new();

        let response = client.post(address).form(&form).send().await?;

        let headers = match response.headers().get("set-cookie") {
            Some(header) => header,
            None => return Err(error::Error::MissingHeaders),
        };

        let cookie_str = headers.to_str()?;
        let cookie_header = match cookie_str.find(";") {
            Some(index) => index,
            None => return Err(error::Error::MissingCookie),
        };

        let cookie = match cookie_str.get(0..cookie_header) {
            Some(cookie) => cookie,
            None => return Err(error::Error::SliceError),
        };

        Ok(Self {
            cookie: cookie.to_string(),
            address: address.to_string(),
            client,
        })
    }

    pub async fn application_version(&self) -> Result<String, error::Error> {
        let addr = push_own! {self.address, "/api/v2/app/version"};

        let res = self.client.get(&addr).send().await?.text().await?;
        Ok(res)
    }

    pub async fn api_version(&self) -> Result<String, error::Error> {
        let addr = push_own! {self.address, "/api/v2/app/webapiVersion"};

        let res = self.client.get(&addr).send().await?.text().await?;
        Ok(res)
    }

    // Not yet in API ?
    pub async fn build_info(&self) -> Result<BuildInfo, error::Error> {
        let addr = push_own! {self.address, "/api/v2/app/buildInfo"};

        let res = self.client.get(&addr).send().await?.bytes().await?;

        let info = serde_json::from_slice(&res)?;

        Ok(info)
    }

    pub async fn shutdown(&self) -> Result<(), error::Error> {
        let addr = push_own! {self.address, "/api/v2/app/shutdown"};

        let res = self.client.get(&addr).send().await?;

        Ok(())
    }

    // TODO: make struct
    pub async fn preferences(&self) -> Result<Preferences, error::Error> {
        let addr = push_own! {self.address, "/api/v2/app/preferences"};

        let res = self.client.get(&addr).send().await?.bytes().await?;
        let pref = serde_json::from_slice(&res)?;

        unimplemented!()
        // Ok(pref)
    }

    pub fn set_preferences(&self) -> Result<(), error::Error> {
        unimplemented!()
    }

    pub async fn default_save_path(&self) -> Result<String, error::Error> {
        let addr = push_own! {self.address, "/api/v2/app/defaultSavePath"};

        let mut res = self.client.get(&addr).send().await?;

        Ok(res.text().await?)
    }

    // ######
    // ###### Logging
    // ######

    // Error here
    pub async fn get_log(&self, log_request: &LogRequest) -> Result<Vec<Log>, error::Error> {
        let url = format! {"/api/v2/log/main?{}", log_request.url()};
        let addr = push_own! {self.address, &url};

        let res = self.client.get(&addr).send().await?.bytes().await?;

        let log: Vec<Log> = serde_json::from_slice(&res)?;

        Ok(log)
    }

    pub fn get_peer_log(&self) -> Result<Vec<Peer>, error::Error> {
        unimplemented!()
    }

    // #####
    // ##### Sync
    // #####

    pub fn get_main_data(&self) -> Result<MainData, error::Error> {
        unimplemented!()
    }

    // get_torrent_peers is a trait

    // #####
    // ##### Transfer Info
    // #####

    pub fn get_global_transfer_info(&self) -> Result<(), ()> {
        unimplemented!()
    }

    pub fn get_alternate_speed_limits_state(&self) -> Result<(), error::Error> {
        unimplemented!()
    }

    pub fn set_alternate_speed_limits_state(&self) -> Result<(), error::Error> {
        unimplemented!()
    }

    pub fn get_global_donwload_limit(&self) -> Result<(), error::Error> {
        unimplemented!()
    }

    pub fn set_global_download_limit(&self) -> Result<(), error::Error> {
        unimplemented!()
    }

    pub fn get_global_upload_limit(&self) -> Result<(), error::Error> {
        unimplemented!()
    }

    pub fn set_global_upload_limit(&self) -> Result<(), error::Error> {
        unimplemented!()
    }

    // ban_peers is a trait

    // TODO: extra filtering parameters here
    pub async fn get_torrent_list(&self) -> Result<Vec<Torrent>, error::Error> {
        let addr = push_own! {self.address, "/api/v2/torrents/info"};

        let res = self.client.get(&addr).send().await?.bytes().await?;

        let all_torrents: Vec<Torrent> = serde_json::from_slice(&res)?;

        Ok(all_torrents)
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
    fn url(&self) -> String {
        unimplemented! {}
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

pub trait GetTorrentPeers {
    // still TODO in the api
    // should be impl'd for anyone who owns a torrent info hash
    fn get_torrent_peers(&self) -> ();
}

pub trait BanPeer {
    fn ban_peer(&self) -> ();
}

// dl_info_speed 	integer 	Global download rate (bytes/s)
// dl_info_data 	integer 	Data downloaded this session (bytes)
// up_info_speed 	integer 	Global upload rate (bytes/s)
// up_info_data 	integer 	Data uploaded this session (bytes)
// dl_rate_limit 	integer 	Download rate limit (bytes/s)
// up_rate_limit 	integer 	Upload rate limit (bytes/s)
// dht_nodes 	integer 	DHT nodes connected to
// connection_status 	string 	Connection status. See possible values here below

#[derive(Debug, Deserialize)]
pub struct TransferInfo {
    dl_info_speed: u64,
    dl_info_data: u64,
    up_info_speed: u64,
    up_info_data: u64,
    dl_rate_limit: u64,
    up_rate_limit: u64,
    dht_nodes: u64,
    connection_status: ConnectionStatus,
}

// Possible values of connection_status:
// Value
// connected
// firewalled
// disconnected

#[derive(Debug, Deserialize)]
enum ConnectionStatus {
    Connected,
    Firewalled,
    Disconnected,
}

// TODO: fix struct definitions
#[derive(Debug, Deserialize)]
pub struct MainData {
    rid: u64,
    full_update: bool,
    torrents: Torrent,
    torrents_removed: Vec<String>,
    categories: Categories,
    categories_removed: Vec<String>,
    tags: Vec<String>,
    tags_removed: Vec<String>,
    queueing: bool,
    server_state: ServerState,
}

// added_on 	integer 	Time (Unix Epoch) when the torrent was added to the client
// amount_left 	integer 	Amount of data left to download (bytes)
// auto_tmm 	bool 	Whether this torrent is managed by Automatic Torrent Management
// category 	string 	Category of the torrent
// completed 	integer 	Amount of transfer data completed (bytes)
// completion_on 	integer 	Time (Unix Epoch) when the torrent completed
// dl_limit 	integer 	Torrent download speed limit (bytes/s). -1 if ulimited.
// dlspeed 	integer 	Torrent download speed (bytes/s)
// downloaded 	integer 	Amount of data downloaded
// downloaded_session 	integer 	Amount of data downloaded this session
// eta 	integer 	Torrent ETA (seconds)
// f_l_piece_prio 	bool 	True if first last piece are prioritized
// force_start 	bool 	True if force start is enabled for this torrent
// hash 	string 	Torrent hash
// last_activity 	integer 	Last time (Unix Epoch) when a chunk was downloaded/uploaded
// magnet_uri 	string 	Magnet URI corresponding to this torrent
// max_ratio 	float 	Maximum share ratio until torrent is stopped from seeding/uploading
// max_seeding_time 	integer 	Maximum seeding time (seconds) until torrent is stopped from seeding
// name 	string 	Torrent name
// num_complete 	integer 	Number of seeds in the swarm
// num_incomplete 	integer 	Number of leechers in the swarm
// num_leechs 	integer 	Number of leechers connected to
// num_seeds 	integer 	Number of seeds connected to
// priority 	integer 	Torrent priority. Returns -1 if queuing is disabled or torrent is in seed mode
// progress 	float 	Torrent progress (percentage/100)
// ratio 	float 	Torrent share ratio. Max ratio value: 9999.
// ratio_limit 	float 	TODO (what is different from max_ratio?)
// save_path 	string 	Path where this torrent's data is stored
// seeding_time_limit 	integer 	TODO (what is different from max_seeding_time?)
// seen_complete 	integer 	Time (Unix Epoch) when this torrent was last seen complete
// seq_dl 	bool 	True if sequential download is enabled
// size 	integer 	Total size (bytes) of files selected for download
// state 	string 	Torrent state. See table here below for the possible values
// super_seeding 	bool 	True if super seeding is enabled
// tags 	string 	Comma-concatenated tag list of the torrent
// time_active 	integer 	Total active time (seconds)
// total_size 	integer 	Total size (bytes) of all file in this torrent (including unselected ones)
// tracker 	string 	The first tracker with working status. (TODO: what is returned if no tracker is working?)
// up_limit 	integer 	Torrent upload speed limit (bytes/s). -1 if ulimited.
// uploaded 	integer 	Amount of data uploaded
// uploaded_session 	integer 	Amount of data uploaded this session
// upspeed 	integer 	Torrent upload speed (bytes/s)

#[derive(Debug, Deserialize)]
pub struct Torrent {
    added_on: u32,
    amount_left: u32,
    auto_tmm: bool,
    category: String,
    completed: u64,
    completion_on: u32,
    dl_limit: i64,
    dlspeed: u64,
    downloaded: u64,
    downloaded_session: u64,
    eta: u64,
    f_l_piece_prio: bool,
    force_start: bool,
    hash: String,
    last_activity: u64,
    magnet_uri: String,
    max_ratio: f64,
    max_seeding_time: i64,
    name: String,
    num_complete: u64,
    num_incomplete: u64,
    num_leechs: u64,
    num_seeds: u64,
    priority: i64,
    progress: f64,
    ratio: f64,
    ratio_limit: f64,
    save_path: String,
    seeding_time_limit: i64,
    seen_complete: u64,
    seq_dl: bool,
    size: u64,
    state: State,
    super_seeding: bool,
    tags: String,
    time_active: u64,
    total_size: u64,
    tracker: String,
    up_limit: i64,
    uploaded: u64,
    uploaded_session: u64,
    upspeed: u64,
}

impl Torrent {
    /// Corresponds to get_torrent_generic_properties in qbit documentation
    pub async fn properties(&self, api: &Api) -> Result<TorrentProperties, error::Error> {
        let _hash = &self.hash;
        let addr = push_own! {api.address, "/api/v2/torrents/properties?hash=", _hash};
        dbg! {&addr};

        // let res = api.client.get(&addr).send().await?.bytes().await?;
        let res = api.client.get(&addr).send().await?.bytes().await?;

        let props = serde_json::from_slice(&res)?;
        Ok(props)
    }

    pub async fn trackers(&self, api: &Api) -> Result<Vec<Tracker>, error::Error> {
        let _hash = &self.hash;
        let addr = push_own! {api.address, "/api/v2/torrents/trackers?hash=", _hash};

        let res = api.client.get(&addr).send().await?.bytes().await?;

        // dbg1{}

        let trackers = serde_json::from_slice(&res)?;
        Ok(trackers)
    }

    pub async fn resume(&self, api: &Api) -> Result<(), error::Error> {
        let _hash = &self.hash;
        let addr = push_own! {api.address, "/api/v2/torrents/trackers?hashes=", _hash};

        // let res = api.client.get(&addr).send().await?.bytes().await?;

        resume_torrents(&api, &self.hash).await
    }
}

async fn resume_torrents<'a, T: Into<Hash>>(api: &Api, hashes: T) -> Result<(), error::Error> {
    let hash = hashes.into();
    let url = hash.url();
    let addr = push_own! {api.address, "/api/v2/torrents/resume", &url};
    
    let res = api.client.get(&addr).send().await?;

    match res.error_for_status() {
        Ok(_) => Ok(()),
        Err(e) => Err(error::Error::from(e))
    }
}

#[derive(Deserialize,Serialize)]
pub struct Hash {
    hashes: Vec<String>,
}
impl Hash {
    fn url(&self) -> String{
        let mut url = String::with_capacity(self.hashes.len() * 32);
        url.push_str("?hashes=");
        for h in &self.hashes {
            url.push_str(h);
            url.push_str("|")
        }
        return url[0..url.len()-1].into();
    }
}
impl<'a> From<&'a str> for Hash {
    fn from(e: &'a str) -> Hash {
        return Hash { hashes: vec![e.into()] };
    }
}
impl<'a> From<&'a String> for Hash {
    fn from(e: &'a String) -> Hash {
        return Hash { hashes: vec![e.into()] };
    }
}
impl<'a> From<String> for Hash {
    fn from(e: String) -> Hash {
        return Hash { hashes: vec![e] };
    }
}
impl From<Vec<String>> for Hash {
    fn from(e: Vec<String>) -> Hash {
        return Hash { hashes: e };
    }
}
impl <'a> From<Vec<&'a str>> for Hash {
    fn from(e: Vec<&'a str>) -> Hash {
        return Hash { hashes: e.into_iter().map(|x| x.into()).collect() };
    }
}

// url 	string 	Tracker url
// status 	integer 	Tracker status. See the table below for possible values
// tier 	integer 	Tracker priority tier. Lower tier trackers are tried before higher tiers
// num_peers 	integer 	Number of peers for current torrent, as reported by the tracker
// num_seeds 	integer 	Number of seeds for current torrent, asreported by the tracker
// num_leeches 	integer 	Number of leeches for current torrent, as reported by the tracker
// num_downloaded 	integer 	Number of completed downlods for current torrent, as reported by the tracker
// msg 	string 	Tracker message (there is no way of knowing what this message is - it's up to tracker admins)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tracker {
    url: String,
    status: u8,
    // TODO: fix this since some people do things non standard with "/" here
    // tier: u32,
    num_peers: i32,
    num_seeds: i32,
    num_leeches: i32,
    num_downloaded: i64,
    msg: String,
}
impl Tracker {
    pub fn status(&self) -> TrackerStatus {
        match self.status {
            0 => TrackerStatus::TrackerDisabled,
            1 => TrackerStatus::NotContacted,
            2 => TrackerStatus::Working,
            3 => TrackerStatus::Updating,
            4 => TrackerStatus::NotWorking,
            _ => TrackerStatus::UnknownResponse,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TrackerStatus {
    TrackerDisabled,
    NotContacted,
    Working,
    Updating,
    NotWorking,
    UnknownResponse,
}

/// Metadata about a torrent. returned from Torrent::properties()
/// save_path 	string 	Torrent save path
/// creation_date 	integer 	Torrent creation date (Unix timestamp)
/// piece_size 	integer 	Torrent piece size (bytes)
/// comment 	string 	Torrent comment
/// total_wasted 	integer 	Total data wasted for torrent (bytes)
/// total_uploaded 	integer 	Total data uploaded for torrent (bytes)
/// total_uploaded_session 	integer 	Total data uploaded this session (bytes)
/// total_downloaded 	integer 	Total data uploaded for torrent (bytes)
/// total_downloaded_session 	integer 	Total data downloaded this session (bytes)
/// up_limit 	integer 	Torrent upload limit (bytes/s)
/// dl_limit 	integer 	Torrent download limit (bytes/s)
/// time_elapsed 	integer 	Torrent elapsed time (seconds)
/// seeding_time 	integer 	Torrent elapsed time while complete (seconds)
/// nb_connections 	integer 	Torrent connection count
/// nb_connections_limit 	integer 	Torrent connection count limit
/// share_ratio 	float 	Torrent share ratio
/// addition_date API4 	integer 	When this torrent was added (unix timestamp)
/// completion_date API4 	integer 	Torrent completion date (unix timestamp)
/// created_by API4 	string 	Torrent creator
/// dl_speed_avg API4 	integer 	Torrent average download speed (bytes/second)
/// dl_speed API4 	integer 	Torrent download speed (bytes/second)
/// eta API4 	integer 	Torrent ETA (seconds)
/// last_seen API4 	integer 	Last seen complete date (unix timestamp)
/// peers API4 	integer 	Number of peers connected to
/// peers_total API4 	integer 	Number of peers in the swarm
/// pieces_have API4 	integer 	Number of pieces owned
/// pieces_num API4 	integer 	Number of pieces of the torrent
/// reannounce API4 	integer 	Number of seconds until the next announce
/// seeds API4 	integer 	Number of seeds connected to
/// seeds_total API4 	integer 	Number of seeds in the swarm
/// total_size API4 	integer 	Torrent total size (bytes)
/// up_speed_avg API4 	integer 	Torrent average upload speed (bytes/second)
/// up_speed API4 	integer 	Torrent upload speed (bytes/second)
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TorrentProperties {
    save_path: String,
    creation_date: u32,
    piece_size: i64,
    comment: String,
    total_wasted: i64,
    total_uploaded: i64,
    total_uploaded_session: i64,
    total_downloaded: i64,
    total_downloaded_session: i64,
    up_limit: i64,
    dl_limit: i64,
    time_elapsed: i64,
    seeding_time: i64,
    nb_connections: i64,
    nb_connections_limit: i64,
    share_ratio: f64,
    addition_date: i64,
    completion_date: i64,
    created_by: String,
    dl_speed_avg: i64,
    dl_speed: i64,
    eta: i64,
    last_seen: i64,
    peers: i64,
    peers_total: i64,
    pieces_have: u64,
    pieces_num: i64,
    reannounce: i64,
    seeds: i64,
    seeds_total: i64,
    total_size: u64,
    up_speed_avg: i64,
    up_speed: i64,
}

// error 	Some error occurred, applies to paused torrents
// missingFiles 	Torrent data files is missing
// uploading 	Torrent is being seeded and data is being transferred
// pausedUP 	Torrent is paused and has finished downloading
// queuedUP 	Queuing is enabled and torrent is queued for upload
// stalledUP 	Torrent is being seeded, but no connection were made
// checkingUP 	Torrent has finished downloading and is being checked
// forcedUP 	Torrent is forced to uploading and ignore queue limit
// allocating 	Torrent is allocating disk space for download
// downloading 	Torrent is being downloaded and data is being transferred
// metaDL 	Torrent has just started downloading and is fetching metadata
// pausedDL 	Torrent is paused and has NOT finished downloading
// queuedDL 	Queuing is enabled and torrent is queued for download
// stalledDL 	Torrent is being downloaded, but no connection were made
// checkingDL 	Same as checkingUP, but torrent has NOT finished downloading
// forceDL 	Torrent is forced to downloading to ignore queue limit
// checkingResumeData 	Checking resume data on qBt startup
// moving 	Torrent is moving to another location
// unknown 	Unknown status

#[derive(Debug, Deserialize)]
enum State {
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "missingFiles")]
    MissingFiles,
    #[serde(rename = "uploading")]
    Uploading,
    #[serde(rename = "pausedUP")]
    PausedUP,
    #[serde(rename = "queuedUP")]
    QueuedUP,
    #[serde(rename = "stalledUP")]
    StalledUP,
    #[serde(rename = "checkingUP")]
    CheckingUP,
    #[serde(rename = "forcedUP")]
    ForcedUP,
    #[serde(rename = "allocating")]
    Allocating,
    #[serde(rename = "downloading")]
    Downloading,
    #[serde(rename = "metaDL")]
    MetaDL,
    #[serde(rename = "pausedDL")]
    PausedDL,
    #[serde(rename = "queuedDL")]
    QueuedDL,
    #[serde(rename = "stalledDL")]
    StalledDL,
    #[serde(rename = "checkingDL")]
    CheckingDL,
    #[serde(rename = "forcedDL")]
    ForceDL,
    #[serde(rename = "checkingResumeData")]
    CheckingResumeData,
    #[serde(rename = "moving")]
    Moving,
    #[serde(rename = "unkown")]
    Unknown,
}

#[derive(Debug, Deserialize)]
pub struct Categories {}

#[derive(Debug, Deserialize)]
pub struct ServerState {}

#[derive(Debug, Deserialize)]
pub struct Peer {}

#[derive(Debug, Deserialize)]
pub struct BuildInfo {
    qt: String,
    libtorrent: String,
    boost: String,
    openssl: String,
    bitness: String,
}

#[derive(Deserialize, Debug)]
pub struct Preferences {}

#[derive(Deserialize, Debug)]
pub struct Log {
    id: u64,
    message: String,
    timestamp: u64,
    r#type: u64,
}

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

impl LogRequest {
    fn url(&self) -> String {
        format! {"normal={}&info={}&warning={}&critical={}&last_known_id={}", self.normal, self.info, self.warning, self.critical, self.last_known_id}
    }
}

fn _is_send<T: Send>(_: T) {}
fn _is_sync<T: Sync>(_: T) {}

#[tokio::test]
async fn is_send() {
    let api = Api::new("", "", "").await;
    _is_send(api);
}
#[tokio::test]
async fn is_sync() {
    let api = Api::new("", "", "").await;
    _is_sync(api);
}
