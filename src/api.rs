use reqwest;
use serde_json;

// TODO: fix these to specifics
use super::data::*;
use super::error;
use super::queries::*;

#[derive(Debug)]
pub struct Api {
    pub(crate) cookie: String,
    pub(crate) address: String,
    pub(crate) client: reqwest::Client,
}

impl Api {
    pub async fn new(username: &str, password: &str, address: &str) -> Result<Self, error::Error> {
        let client = reqwest::Client::new();

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Referer", address.parse()?);

        let addr = push_own! {address, "/api/v2/auth/login", "?username=", username, "&password=", password};
        let response = client.get(&addr).headers(headers).send().await?;

        let headers = match response.headers().get("set-cookie") {
            Some(header) => header,
            None => return Err(error::Error::MissingHeaders),
        };

        let cookie_str = headers.to_str()?;
        let cookie_header = match cookie_str.find(";") {
            Some(index) => index,
            None => return Err(error::Error::MissingCookie),
        };

        // parse off the "SID=" portion of the cookie
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

        self.client.get(&addr).send().await?;

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

        let res = self.client.get(&addr).send().await?;

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

    // /api/v2/transfer/methodName

    pub fn get_global_transfer_info(&self) -> Result<(), ()> {
        unimplemented!()
    }

    pub fn get_alternate_speed_limits_state(&self) -> Result<(), error::Error> {
        unimplemented!()
    }

    pub async fn toggle_alternative_speed_limits(&self) -> Result<(), error::Error> {
        let addr = push_own! {self.address, "/api/v2/transfer/toggleSpeedLimitsMode"};

        let res = self.client.get(&addr).send().await?;

        match res.error_for_status() {
            Ok(_) => Ok(()),
            Err(e) => Err(error::Error::from(e)),
        }
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

        let res = self
            .client
            .get(&addr)
            .headers(self.make_headers()?)
            .send()
            .await?
            .bytes()
            .await?;

        let all_torrents: Vec<Torrent> = serde_json::from_slice(&res)?;

        Ok(all_torrents)
    }

    pub async fn add_new_torrent(&self, data: &TorrentDownload) -> Result<(), error::Error> {
        let addr = push_own! {self.address, "/api/v2/torrents/add"};

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("cookie", self.cookie.parse()?);
        headers.insert("Referer", self.address.parse()?);

        let res = self
            .client
            .post(&addr)
            .form(data)
            .headers(headers)
            .send()
            .await?;

        match res.error_for_status() {
            Ok(_) => Ok(()),
            Err(e) => Err(error::Error::from(e)),
        }
    }

    /// Make the authentication headers for each request
    pub(crate) fn make_headers(&self) -> Result<reqwest::header::HeaderMap, error::Error> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("cookie", self.cookie.parse()?);
        Ok(headers)
    }

    pub async fn get_all_categories(&self) -> Result<Vec<Categories>, error::Error> {
        // let addr = push_own!(self.address, "/api/v2/torrents/categories");

        // dbg! {&addr};

        // let res = self
        //     .client
        //     .get(&addr)
        //     .headers(self.make_headers()?)
        //     .send()
        //     .await?
        //     .bytes()
        //     .await?;

        // dbg! {&res};
        // let x = serde_json::from_slice(&res)?;
        // Ok(x)

        unimplemented!()
    }

    pub async fn add_category(&self, name: &str, path: &str) -> Result<(), error::Error> {
        let addr = push_own!(
            self.address,
            "/api/v2/torrents/createCategory?savePath=",
            path,
            "&category=",
            name
        );

        dbg! {&addr};

        let res = self
            .client
            .get(&addr)
            .headers(self.make_headers()?)
            .send()
            .await?
            .bytes()
            .await?;
        dbg! {res};

        Ok(())
    }
}
