use async_trait::async_trait;

use super::api::Api;
use super::data::{self, Hash, Torrent, TorrentInfo, TorrentProperties, Tracker};
use super::error::Error;

#[async_trait]
pub trait TorrentData<T> {
    async fn properties(&self, other: &'_ T) -> Result<TorrentProperties, Error>;
    async fn trackers(&self, other: &'_ T) -> Result<Vec<Tracker>, Error>;
    async fn contents(&self, other: &'_ T) -> Result<Vec<TorrentInfo>, Error>;
}

// Resume a torrent
#[async_trait]
pub trait Resume<T> {
    async fn resume(&self, other: &'_ T) -> Result<(), Error>;
}
#[async_trait]
pub trait Pause<T> {
    async fn pause(&self, other: &'_ T) -> Result<(), Error>;
}

#[async_trait]
impl TorrentData<Api> for Torrent {
    async fn properties(&self, api: &'_ Api) -> Result<TorrentProperties, Error> {
        let _hash = &self.hash;
        let addr = push_own! {api.address, "/api/v2/torrents/properties?hash=", _hash};

        let res = api.client.get(&addr).send().await?.bytes().await?;

        let props = serde_json::from_slice(&res)?;
        Ok(props)
    }

    async fn trackers(&self, api: &'_ Api) -> Result<Vec<Tracker>, Error> {
        let _hash = &self.hash;
        let addr = push_own! {api.address, "/api/v2/torrents/trackers?hash=", _hash};

        let res = api.client.get(&addr).send().await?.bytes().await?;

        let trackers = serde_json::from_slice(&res)?;
        Ok(trackers)
    }

    async fn contents(&self, api: &'_ Api) -> Result<Vec<TorrentInfo>, Error> {
        let _hash = &self.hash;
        let addr = push_own! {api.address, "/api/v2/torrents/files?hash=", _hash};

        let res = api
            .client
            .get(&addr)
            .headers(api.make_headers()?)
            .send()
            .await?
            .bytes()
            .await?;

        let info = serde_json::from_slice(&res)?;

        Ok(info)
    }
}

#[async_trait]
impl TorrentData<Torrent> for Api {
    async fn properties(&self, torrent: &'_ Torrent) -> Result<TorrentProperties, Error> {
        torrent.properties(&self).await
    }

    async fn trackers(&self, torrent: &'_ Torrent) -> Result<Vec<Tracker>, Error> {
        torrent.trackers(&self).await
    }

    async fn contents(&self, torrent: &'_ Torrent) -> Result<Vec<TorrentInfo>, Error> {
        torrent.contents(&self).await
    }
}

#[async_trait]
impl Resume<Api> for Torrent {
    async fn resume(&self, api: &'_ Api) -> Result<(), Error> {
        let _hash = &self.hash;
        let addr = push_own! {api.address, "/api/v2/torrents/resume?hashes=", _hash};

        let res = api.client.get(&addr).send().await?;

        match res.error_for_status() {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::from(e)),
        }
    }
}

#[async_trait]
impl Resume<Api> for Hash {
    async fn resume(&self, api: &'_ Api) -> Result<(), Error> {
        let _hash = &self.hash;
        let addr = push_own! {api.address, "/api/v2/torrents/resume?hashes=", _hash};

        let res = api.client.get(&addr).send().await?;

        match res.error_for_status() {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::from(e)),
        }
    }
}

#[async_trait]
impl Resume<Api> for Vec<Hash> {
    async fn resume(&self, api: &'_ Api) -> Result<(), Error> {
        // concat all hashes together with "|" character separation
        let mut hash_url = self
            .iter()
            .map(|x| {
                let mut cln = x.hash.clone();
                cln.push_str("|");
                cln
            })
            .collect::<String>();

        // remove the final | from the string
        hash_url.remove(hash_url.len() - 1);

        dbg! {&hash_url};

        let addr = push_own! {api.address, "/api/v2/torrents/resume?hashes=", &hash_url};

        let res = api.client.get(&addr).send().await?;

        match res.error_for_status() {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::from(e)),
        }
    }
}

#[async_trait]
impl Pause<Api> for Hash {
    async fn pause(&self, api: &'_ Api) -> Result<(), Error> {
        let _hash = &self.hash;
        let addr = push_own! {api.address, "/api/v2/pause/pause?hashes=", _hash};

        let res = api.client.get(&addr).send().await?;

        match res.error_for_status() {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::from(e)),
        }
    }
}

#[async_trait]
impl Pause<Api> for Vec<Hash> {
    async fn pause(&self, api: &'_ Api) -> Result<(), Error> {
        // concat all hashes together with "|" character separation
        let mut hash_url = self
            .iter()
            .map(|x| {
                let mut cln = x.hash.clone();
                cln.push_str("|");
                cln
            })
            .collect::<String>();

        // remove the final | from the string
        hash_url.remove(hash_url.len() - 1);

        dbg! {&hash_url};

        let addr = push_own! {api.address, "/api/v2/torrents/pause?hashes=", &hash_url};

        let res = api.client.get(&addr).send().await?;

        match res.error_for_status() {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::from(e)),
        }
    }
}
