use async_trait::async_trait;

use serde::Serialize;

use super::api::Api;
use super::data::*;
use super::error::Error;
use super::utils::QueryConcat;

#[async_trait]
pub trait TorrentData<T> {
    async fn properties(&self, other: &'_ T) -> Result<TorrentProperties, Error>;
    async fn trackers(&self, other: &'_ T) -> Result<Vec<Tracker>, Error>;
    // TOOD: when impl'd on Api, self is 'a when it does not need to be
    // when impl'd on Torrent, other is &Api which also does not need to be 'a
    async fn contents<'a>(&'a self, other: &'a T) -> Result<Vec<TorrentInfo<'a>>, Error>;
}

#[async_trait]
pub trait Recheck<T> {
    async fn recheck(&self, other: &'_ T) -> Result<(), Error>;
}

#[async_trait]
pub trait Category<T> {
    async fn set_category(&self, other: &'_ T, category: &str) -> Result<(), Error>;
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
pub trait Tags<T, V: ?Sized> {
    async fn add_tag(&self, other: &'_ T, tags: &'_ V) -> Result<(), Error>;
}

#[async_trait]
impl Category<Api> for Torrent {
    async fn set_category(&self, api: &'_ Api, category: &str) -> Result<(), Error> {
        let addr = push_own!(
            api.address,
            "/api/v2/torrents/setCategory?hashes=",
            &self.hash,
            "&category=",
            category
        );

        let res = api
            .client
            .get(&addr)
            .headers(api.make_headers()?)
            .send()
            .await?
            .bytes()
            .await?;
        dbg! {res};

        Ok(())
    }
}
#[async_trait]
impl TorrentData<Api> for Torrent {
    async fn properties(&self, api: &'_ Api) -> Result<TorrentProperties, Error> {
        self.hash.properties(&api).await
    }

    async fn trackers(&self, api: &'_ Api) -> Result<Vec<Tracker>, Error> {
        self.hash.trackers(api).await
    }

    async fn contents<'a>(&'a self, api: &'a Api) -> Result<Vec<TorrentInfo<'a>>, Error> {
        self.hash.contents(api).await
    }
}

#[async_trait]
impl TorrentData<Api> for Hash {
    async fn properties(&self, api: &'_ Api) -> Result<TorrentProperties, Error> {
        let _hash = &self.hash;
        let addr = push_own! {api.address, "/api/v2/torrents/properties?hash=", self};

        let res = api
            .client
            .get(&addr)
            .headers(api.make_headers()?)
            .send()
            .await?
            .bytes()
            .await?;

        let props = serde_json::from_slice(&res)?;
        Ok(props)
    }

    async fn trackers(&self, api: &'_ Api) -> Result<Vec<Tracker>, Error> {
        let addr = push_own! {api.address, "/api/v2/torrents/trackers?hash=", self};

        let res = api
            .client
            .get(&addr)
            .headers(api.make_headers()?)
            .send()
            .await?
            .bytes()
            .await?;

        let trackers = serde_json::from_slice(&res)?;
        Ok(trackers)
    }

    async fn contents<'a>(&'a self, api: &'a Api) -> Result<Vec<TorrentInfo<'a>>, Error> {
        // let _hash = &self.hash;
        let addr = push_own! {api.address, "/api/v2/torrents/files?hash=", self};

        let res = api
            .client
            .get(&addr)
            .headers(api.make_headers()?)
            .send()
            .await?
            .bytes()
            .await?;

        let info = serde_json::from_slice::<Vec<TorrentInfoSerde>>(&res)?
            .into_iter()
            .map(|x| x.into_info(self))
            .collect();

        Ok(info)
    }
}

#[async_trait]
impl TorrentData<Torrent> for Api {
    async fn properties(&self, torrent: &'_ Torrent) -> Result<TorrentProperties, Error> {
        torrent.hash.properties(&self).await
    }

    async fn trackers(&self, torrent: &'_ Torrent) -> Result<Vec<Tracker>, Error> {
        torrent.hash.trackers(&self).await
    }

    async fn contents<'a>(&'a self, torrent: &'a Torrent) -> Result<Vec<TorrentInfo<'a>>, Error> {
        torrent.hash.contents(&self).await
    }
}

#[async_trait]
impl Resume<Api> for Torrent {
    async fn resume(&self, api: &'_ Api) -> Result<(), Error> {
        self.hash.resume(&api).await
    }
}

#[async_trait]
impl Resume<Api> for Hash {
    async fn resume(&self, api: &'_ Api) -> Result<(), Error> {
        let _hash = &self.hash;
        let addr = push_own! {api.address, "/api/v2/torrents/resume?hashes=", _hash};

        let res = api
            .client
            .get(&addr)
            .headers(api.make_headers()?)
            .send()
            .await?;

        match res.error_for_status() {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::from(e)),
        }
    }
}

#[async_trait]
impl Resume<Api> for Vec<Hash> {
    async fn resume(&self, api: &'_ Api) -> Result<(), Error> {
        let hash_url = QueryConcat::query_concat(&self.as_slice(), '|');

        let addr = push_own! {api.address, "/api/v2/torrents/resume?hashes=", &hash_url};

        let res = api
            .client
            .get(&addr)
            .headers(api.make_headers()?)
            .send()
            .await?;

        match res.error_for_status() {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::from(e)),
        }
    }
}

#[async_trait]
impl Pause<Api> for Torrent {
    async fn pause(&self, api: &'_ Api) -> Result<(), Error> {
        self.hash.pause(&api).await
    }
}

#[async_trait]
impl Pause<Api> for Hash {
    async fn pause(&self, api: &'_ Api) -> Result<(), Error> {
        let _hash = &self.hash;
        let addr = push_own! {api.address, "/api/v2/torrents/pause?hashes=", _hash};

        let res = api
            .client
            .get(&addr)
            .headers(api.make_headers()?)
            .send()
            .await?;

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

        let addr = push_own! {api.address, "/api/v2/torrents/pause?hashes=", &hash_url};

        let res = api
            .client
            .get(&addr)
            .headers(api.make_headers()?)
            .send()
            .await?;

        match res.error_for_status() {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::from(e)),
        }
    }
}

#[derive(Serialize)]
struct TagsUrlHelper<'hash, 'tag> {
    hashes: &'hash [&'hash Hash],
    tags: &'tag [String],
}
impl<'hash, 'tag> TagsUrlHelper<'hash, 'tag> {
    fn url(self) -> Result<String, Error> {
        let mut url = String::with_capacity(25);
        let hashes = QueryConcat::query_concat(self.hashes, '|');
        let tags = QueryConcat::query_concat(&self.tags, ',');

        push_own!(prealloc; url, "hashes=", &hashes,"&tags=", &tags);

        Ok(url)
    }
}

#[async_trait]
impl Tags<Api, [String]> for Torrent {
    async fn add_tag(&self, api: &'_ Api, tags: &'_ [String]) -> Result<(), Error> {
        self.hash.add_tag(&api, tags).await
    }
}

#[async_trait]
impl Tags<Api, [String]> for Hash {
    async fn add_tag(&self, api: &'_ Api, tags: &'_ [String]) -> Result<(), Error> {
        let helper = TagsUrlHelper {
            hashes: &[self],
            tags,
        };
        let addr = push_own! {api.address, "/api/v2/torrents/addTags?", &helper.url()?};

        let res = api
            .client
            .post(&addr)
            .headers(api.make_headers()?)
            .send()
            .await;

        match res?.error_for_status() {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::from(e)),
        }
    }
}

#[async_trait]
impl Tags<Api, [String]> for Vec<Hash> {
    async fn add_tag(&self, api: &'_ Api, tags: &'_ [String]) -> Result<(), Error> {
        // let addr = push_own! {api.address, "/api/v2/torrents/addTags"};

        // let _tags = QueryConcat::query_concat(&tags);
        // let form = serde_json::json! {
        //     {
        //         "hashes" : QueryConcat::query_concat(&self.as_slice()),
        //         "tags" : _tags
        //     }
        // };

        // dbg!{&form};

        // let res = api
        //     .client
        //     .post(&addr)
        //     .headers(api.make_headers()?)
        //     .form(&form)
        //     .send()
        //     .await?;

        // match res.error_for_status() {
        //     Ok(_) => Ok(()),
        //     Err(e) => Err(Error::from(e)),
        // }

        unimplemented!()
    }
}
#[async_trait]
impl Tags<Api, [String]> for Vec<Torrent> {
    async fn add_tag(&self, api: &'_ Api, tags: &'_ [String]) -> Result<(), Error> {
        // let addr = push_own! {api.address, "/api/v2/torrents/addTags"};

        // // let _tags = QueryConcat::query_concat(&tags);
        // // pull hashes from the torrents
        // let hashes = self
        //     .iter()
        //     .map(|x| x.hash.to_string())
        //     .collect::<Vec<String>>();

        // let form = serde_json::json! {
        //     {
        //         "hashes" : QueryConcat::query_concat(&hashes.as_slice()),
        //         "tags" : _tags
        //     }
        // };

        // let res = api
        //     .client
        //     .post(&addr)
        //     .headers(api.make_headers()?)
        //     .form(&form)
        //     .send()
        //     .await?;

        // match res.error_for_status() {
        //     Ok(_) => Ok(()),
        //     Err(e) => Err(Error::from(e)),
        // }

        unimplemented!()
    }
}

#[async_trait]
impl Recheck<Api> for Hash {
    async fn recheck(&self, api: &'_ Api) -> Result<(), Error> {
        let addr = push_own!(api.address, "/api/v2/torrents/recheck?hashes=", &self.hash);

        api.client
            .get(&addr)
            .headers(api.make_headers()?)
            .send()
            .await?;

        Ok(())
    }
}
