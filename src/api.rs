use reqwest;
use std::borrow::Cow;
use std::collections::HashMap;

use super::error;

use derive_builder::Builder;
use serde::Deserialize;
use serde_json;

macro_rules! push_own {
    ($s:ident . $inner:ident, $push_val:expr, $save_var:ident) => {
        let mut $save_var = $s.$inner.clone().to_string();
        $save_var.push_str($push_val);
    };
}

#[derive(Debug)]
pub struct Api {
    cookie: String,
    address: String,
    client: reqwest::Client,
}

impl Api {
    pub fn new(
        username: &str,
        password: &str,
        address: &str,
    ) -> Result<Self, error::Authentication> {
        let mut form = HashMap::new();
        form.insert("username", username);
        form.insert("password", password);

        let client = reqwest::Client::new();

        let mut response = client.post(address).form(&form).send()?;

        let headers = match response.headers().get("set-cookie") {
            Some(header) => header,
            None => return Err(error::Authentication::MissingHeaders),
        };

        let cookie_str = headers.to_str()?;
        let cookie_header = match cookie_str.find(";") {
            Some(index) => index,
            None => return Err(error::Authentication::MissingCookie),
        };

        let cookie = match cookie_str.get(0..cookie_header) {
            Some(cookie) => cookie,
            None => return Err(error::Authentication::SliceError),
        };

        Ok(Self {
            cookie: cookie.to_string(),
            address: address.to_string(),
            client: client,
        })
    }

    pub fn application_version(&self) -> Result<String, error::Application> {
        push_own! {self.address, "/api/v2/app/version", addr};

        let res = self.client.get(&addr).send()?.text()?;
        Ok(res)
    }

    pub fn api_version(&self) -> Result<String, error::Application> {
        push_own! {self.address, "/api/v2/app/webapiVersion", addr};

        let res = self.client.get(&addr).send()?.text()?;
        Ok(res)
    }

    // Not yet in API ?
    fn build_info(&self) -> Result<BuildInfo, error::Application> {
        push_own! {self.address, "/api/v2/app/buildInfo", addr};

        let res = self.client.get(&addr).send()?;
        let info = serde_json::from_reader(res)?;

        Ok(info)
    }

    pub fn shutdown(&self) -> Result<(), error::Application> {
        push_own! {self.address, "/api/v2/app/shutdown", addr};

        let res = self.client.get(&addr).send()?;

        Ok(())
    }

    // TODO: make struct
    pub fn preferences(&self) -> Result<Preferences, error::Application> {
        push_own! {self.address, "/api/v2/app/preferences", addr};

        let res = self.client.get(&addr).send()?;
        let pref = serde_json::from_reader(res)?;

        unimplemented!()
        // Ok(pref)
    }

    pub fn set_preferences(&self) -> Result<(), error::Application> {
        unimplemented!()
    }

    pub fn default_save_path(&self) -> Result<String, error::Application> {
        push_own! {self.address, "/api/v2/app/defaultSavePath", addr};

        let mut res = self.client.get(&addr).send()?;

        Ok(res.text()?)
    }

    // ######     
    // ###### Logging 
    // ######     

    // Error here
    pub fn get_log(&self, log_request: &LogRequest) -> Result<Vec<Log>, error::Log> {
        let url = format! {"/api/v2/log/main?{}", log_request.url()};
        push_own! {self.address, &url, addr};

        let mut res = self.client.get(&addr).send()?;
        dbg! {res.text()};

        let log: Vec<Log> = serde_json::from_reader(res)?;

        Ok(log)
    }

    pub fn get_peer_log(&self) -> Result<Vec<Peer>, error::Log> {
        unimplemented!()
    }

    // #####
    // ##### Sync
    // #####

    pub fn get_main_data(&self) -> Result<MainData, error::SyncError> {
        unimplemented!()
    }

    // get_torrent_peers is a trait

    // #####    
    // ##### Transfer Info
    // #####

    pub fn get_global_transfer_info(&self) -> Result<(), () > {
        unimplemented!()
    }

    pub fn get_alternate_speed_limits_state(&self) -> Result<(), error::Transfer> {unimplemented!()}

    pub fn set_alternate_speed_limits_state(&self) -> Result<(), error::Transfer> {unimplemented!()}

    pub fn get_global_donwload_limit(&self) -> Result<(), error::Transfer> {unimplemented!()}

    pub fn set_global_download_limit(&self) -> Result<(), error::Transfer > {unimplemented!()}


    pub fn get_global_upload_limit(&self) -> Result<(), error::Transfer> {unimplemented!()}

    pub fn set_global_upload_limit(&self) -> Result<(), error::Transfer > {unimplemented!()}

    // ban_peers is a trait

    pub fn get_torrent_list(&self) -> Result<Vec<Torrent>, error::TorrentMangement> {
        push_own! {self.address, "/api/v2/torrents/info", addr};

        let res = self.client.get(&addr).send()?;
    }


}
// filter optional 	Filter torrent list. Allowed filters: all, downloading, completed, paused, active, inactive, 'resumed'
// category optional 	Get torrents with the given category (empty string means "without category"; no "category" parameter means "any category")
// sort optional 	Sort torrents by given key. All the possible keys are listed here below
// reverse optional 	Enable reverse sorting. Possible values are true and false (default)
// limit optional 	Limit the number of torrents returned
// offset optional 	Set offset (if less than 0, offset from end)
// hashes optional 	Filter by hashes. Can contain multiple hashes separated by |
#[derive(Debug, Builder)]
struct TorrentRequest {
    filter: Option<TorrentFilter>,
    category: Option<String>,
    sort: Option<String>,
    reverse: Option<bool>,
    limit: Option<u64>,
    offset: Option<i64>,
    hashes: Option<Vec<String>>
}
impl TorrentRequest {
    fn url(&self) -> String{

        macro_rules! add_more {
            ($s:ident . $var:ident, $url:ident) => {
                if let Some(x) = $s.$var {
                    let insertion = format!{"{}={}", stringify!{$var}, $var}

                    if $url != "" {
                        $url.push("&")
                    }
                    $url.push_str(insertion);
                }
            };
        }
        
        let mut url = "".to_string();

        add_more!{self.filter, url};
        // add_more!{self.filter, url};
        // add_more!{self.filter, url};
        // add_more!{self.filter, url};
        // add_more!{self.filter, url};
        // add_more!{self.filter, url};

        return url
    }
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
    connection_status: ConnectionStatus
}

// Possible values of connection_status:
// Value
    // connected
    // firewalled
    // disconnected

#[derive(Debug, Deserialize)]
enum ConnectionStatus {
    connected,
    firewalled,
    disconnected
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
    server_state: ServerState
}


// hash 	string 	Torrent hash
// name 	string 	Torrent name
// size 	integer 	Total size (bytes) of files selected for download
// progress 	float 	Torrent progress (percentage/100)
// dlspeed 	integer 	Torrent download speed (bytes/s)
// upspeed 	integer 	Torrent upload speed (bytes/s)
// priority 	integer 	Torrent priority. Returns -1 if queuing is disabled or torrent is in seed mode
// num_seeds 	integer 	Number of seeds connected to
// num_complete 	integer 	Number of seeds in the swarm
// num_leechs 	integer 	Number of leechers connected to
// num_incomplete 	integer 	Number of leechers in the swarm
// ratio 	float 	Torrent share ratio. Max ratio value: 9999.
// eta 	integer 	Torrent ETA (seconds)
// state 	string 	Torrent state. See table here below for the possible values
// seq_dl 	bool 	True if sequential download is enabled
// f_l_piece_prio 	bool 	True if first last piece are prioritized
// category 	string 	Category of the torrent
// tags 	string 	Comma-concatenated tag list of the torrent
// super_seeding 	bool 	True if super seeding is enabled
// force_start 	bool 	True if force start is enabled for this torrent

#[derive(Debug, Deserialize)]
pub struct Torrent {
    hash: String,
    name: String,
    size: u64,
    progress: f64,
    dlspeed: u64,
    upspeed: u64,
    priority: u64,
    num_seeds: u64,
    num_complete: u64,
    ratio: f64,
    eta: u64,
    state: State,
    seq_dl: bool,
    f_l_piece_prio: bool,
    category: String,
    tags: String,
    super_seeding: bool,
    force_start: bool
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
    error,
    missingFiles,
    uploading,
    pausedUP,
    queuedUP,
    stalledUP,
    checkingUP,
    forcedUP,
    allocating,
    downloading,
    metaDL,
    pausedDL,
    queuedDL,
    stalledDL,
    checkingDL,
    forceDL,
    checkingResumeData,
    moving,
    unknown
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
