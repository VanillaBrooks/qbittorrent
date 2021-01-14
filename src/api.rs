use anyhow::{self, bail};
use reqwest::{self, header, Client, StatusCode, Url};
use serde_json;
use std::sync::Arc;

use base64::write::EncoderWriter as Base64Encoder;
use percent_encoding::{percent_encode, NON_ALPHANUMERIC};
use std::io::Write;

// TODO: fix these to specifics
use super::data::*;
use super::error;
use super::queries::*;

#[derive(Debug, Clone)]
pub struct Api {
    pub(crate) address: Url,
    pub(crate) client: reqwest::Client,
}

impl Api {
    pub async fn new(
        basic_auth: bool,
        username: &str,
        password: &str,
        address: Url,
    ) -> Result<Self, anyhow::Error> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Referer", address.as_str().parse()?);
        headers.insert("Origin", address.origin().ascii_serialization().parse()?);
        headers.insert("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_6) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/14.0.2 Safari/605.1.15".parse()?);

        if basic_auth {
            let mut header_value = b"Basic ".to_vec();
            {
                let mut encoder = Base64Encoder::new(&mut header_value, base64::STANDARD);
                write!(encoder, "{}:", username)?;
                write!(encoder, "{}", password)?;
            }
            headers.insert(
                header::AUTHORIZATION,
                header::HeaderValue::from_bytes(&header_value)?,
            );
        }

        let client = Client::builder()
            .cookie_store(true)
            .default_headers(headers)
            .build()?;

        let addr = push_own! {address, "/api/v2/auth/login"};

        let response = client
            .post(&addr)
            .body(format!(
                "username={}&password={}",
                percent_encode(username.as_bytes(), NON_ALPHANUMERIC),
                percent_encode(password.as_bytes(), NON_ALPHANUMERIC)
            ))
            .header(
                header::CONTENT_TYPE,
                "application/x-www-form-urlencoded; charset=UTF-8",
            )
            .send()
            .await?;

        if response.status() != StatusCode::OK {
            bail!(
                "Cannot authenticate to qbittorrent: {status} {body}",
                status = response.status(),
                body = response.text().await?,
            );
        }

        Ok(Self {
            address: address,
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

        let res = self.client.get(&addr).send().await?.bytes().await?;

        let all_torrents: Vec<Torrent> = serde_json::from_slice(&res)?;

        Ok(all_torrents)
    }

    pub async fn get_trackers(&self, info_hash: &str) -> Result<Vec<Tracker>, anyhow::Error> {
        let addr = push_own! {self.address, "/api/v2/torrents/trackers", "?hash=", info_hash};

        let res = self.client.get(&addr).send().await?.bytes().await?;

        let trackers: Vec<Tracker> = serde_json::from_slice(&res)?;

        Ok(trackers)
    }

    pub async fn add_new_torrent(&self, data: &TorrentDownload) -> Result<(), error::Error> {
        let addr = push_own! {self.address, "/api/v2/torrents/add"};

        let res = self.client.post(&addr).form(data).send().await?;

        match res.error_for_status() {
            Ok(_) => Ok(()),
            Err(e) => Err(error::Error::from(e)),
        }
    }

    pub async fn get_all_categories(&self) -> Result<Vec<Categories>, error::Error> {
        // let addr = push_own!(self.address, "/api/v2/torrents/categories");

        // dbg! {&addr};

        // let res = self
        //     .client
        //     .get(&addr)
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

        let res = self.client.get(&addr).send().await?.bytes().await?;

        Ok(())
    }

    pub async fn add_tag(&self, hashes: Vec<&str>, tag: &str) -> Result<(), anyhow::Error> {
        println!("Tagging {} torrents...", hashes.len());
        let addr = push_own! {self.address, "/api/v2/torrents/addTags"};
        let hashes_query = hashes.join("|");
        let body = format!(
            "hashes={hashes}&tags={tag}",
            hashes = hashes_query,
            tag = percent_encode(tag.as_bytes(), NON_ALPHANUMERIC)
        );

        let response = self
            .client
            .post(&addr)
            .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await?;

        if response.status() != StatusCode::OK {
            bail!(
                "Cannot tag torrents: {status} {body}",
                status = response.status(),
                body = response.text().await?,
            );
        }

        Ok(())
    }

    pub async fn tag_error_trackers(&self, tag: &str, all: bool) -> Result<(), anyhow::Error> {
        let torrents = self.get_torrent_list().await.unwrap();

        let mut handles = vec![];
        let sem = Arc::new(tokio::sync::Semaphore::new(20));
        for torrent in torrents {
            let api = self.clone();
            let permit = Arc::clone(&sem).acquire_owned().await;
            handles.push(tokio::spawn(async move {
                let _permit = permit;
                let hash = torrent.hash();
                let name = torrent.name();
                let trackers = api.get_trackers(hash).await.expect(
                    &format!(
                        "Cannot get trackers from: [{hash}] {name}",
                        hash = hash,
                        name = name
                    )[..],
                );

                for tracker in trackers {
                    match tracker.status() {
                        TrackerStatus::TrackerDisabled | TrackerStatus::Working => continue,
                        _ => {
                            if tracker.msg().len() > 0 || all {
                                println!(
                                    "[{hash}]({status:?}) {name} ({msg})",
                                    hash = hash,
                                    status = tracker.status(),
                                    name = name,
                                    msg = tracker.msg()
                                );
                                return Some(hash.clone());
                            } else {
                                continue;
                            }
                        }
                    }
                }

                None
            }));
        }
        let results = futures::future::join_all(handles).await;

        let results: Vec<String> = results
            .into_iter()
            .filter_map(Result::ok)
            .filter(|x| x.is_some())
            .map(|x| x.unwrap().hash)
            .collect();

        let hashes: Vec<&str> = results.iter().map(|s| &**s).collect();

        self.add_tag(hashes, tag).await?;

        Ok(())
    }
}
