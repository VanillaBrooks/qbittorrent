use super::api::Api;
use super::data;
use super::error::Error;
use super::queries;
use super::traits::*;
use tokio;

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

#[tokio::test]
async fn is_send_ref() {
    let api = Api::new("", "", "").await;
    _is_send(&api);
}
#[tokio::test]
async fn is_sync_ref() {
    let api = Api::new("", "", "").await;
    _is_sync(&api);
}

#[allow(dead_code)]
async fn default_api() -> Result<Api, Error> {
    Api::new("admin", "adminadmin", "http://localhost:8080").await
}

#[allow(dead_code)]
async fn get_first_torrent() -> (Api, data::Torrent) {
    let api = default_api().await.expect("could not start api");

    let mut torrent_list = api
        .get_torrent_list()
        .await
        .expect("could not get torrnet list");

    let torrent = if let Some(_) = torrent_list.get(0) {
        torrent_list.remove(0)
    } else {
        panic! {"there were no items in the torrent list to check"}
    };
    (api, torrent)
}

#[tokio::test]
async fn torrent_list() {
    let api = default_api().await.unwrap();
    let torrent_list = api.get_torrent_list().await;
    dbg! {&torrent_list};
    assert! {torrent_list.is_ok()};
}

#[tokio::test]
async fn add_new_torrent() {
    let api = default_api().await.unwrap();
    let mut magnet = String::with_capacity(200);
    magnet.push_str("magnet");
    magnet.push_str(":?xt=urn:");
    magnet.push_str("btih:58e6a9fa8af954342c5a31a1793943bae45496aa&dn=JENNIE%20-%20%27SOLO%27%20MV&tr=http%3A%2F%2Fnyaa.tracker.wf%3A7777%2Fannounce&tr=udp%3A%2F%2Fopen.stealth.si%3A80%2Fannounce&tr=udp%3A%2F%2Ftracker.opentrackr.org%3A1337%2Fannounce&tr=udp%3A%2F%2Ftracker.coppersurfer.tk%3A6969%2Fannounce&tr=udp%3A%2F%2Fexodus.desync.com%3A6969%2Fannounce");

    let download = queries::TorrentDownloadBuilder::default()
        .urls(magnet)
        .savepath("E:\\Torrents")
        .upload_limit(300)
        .category("add_new_torrent_test_category")
        .sequential_download("true")
        .download_limit(200)
        .build()
        .unwrap();

    let add_torrent_result = api.add_new_torrent(&download).await;

    dbg! {&add_torrent_result};

    assert! {add_torrent_result.is_ok()};
}

#[tokio::test]
async fn properties() {
    let (api, torrent) = get_first_torrent().await;

    let props = torrent.properties(&api).await;
    dbg! {&props};
    props.expect("could not get torrent properties");
}

#[tokio::test]
async fn trackers() {
    let (api, torrent) = get_first_torrent().await;

    let trackers = torrent.trackers(&api).await;
    dbg! {&trackers};
    trackers.expect("could not get torrent trackers");
}

#[tokio::test]
async fn contents() {
    let api = default_api().await.unwrap();
    let torrents = queries::TorrentRequestBuilder::default()
        .filter(queries::TorrentFilter::Completed)
        .build()
        .unwrap()
        .send(&api)
        .await
        .unwrap()
        .remove(0);

    dbg! {&torrents};

    let contents = torrents.contents(&api).await;

    dbg! {&contents};

    contents.unwrap();
}

#[tokio::test]
async fn test_pause() {
    let api = default_api().await.unwrap();
    let torrents: Vec<data::Torrent> = queries::TorrentRequestBuilder::default()
        .filter(queries::TorrentFilter::Active)
        .build()
        .unwrap()
        .send(&api)
        .await
        .unwrap();

    // filter the list of torrents for one that is actively uploading but not forced
    let torrent = {
        torrents
            .into_iter()
            .next()
            .unwrap()
    };

    dbg! {&torrent};
    let hash: data::Hash = (*torrent.hash()).clone();

    let pause = torrent.hash().pause(&api).await;
    pause.unwrap();

    // sleep to give time to pause
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;

    // search through all the torrents for the one we just paused
    let new_torrent = queries::TorrentRequestBuilder::default()
        .hash(hash)
        .build()
        .unwrap()
        .send(&api)
        .await;

    dbg! {&new_torrent};

    // since new_torrent is a vector of all matches, pop off the only one inside it
    let first = new_torrent.unwrap();
    let first = first.get(0).unwrap();

    // check that it did indeep pause
    match first.state() {
        data::State::PausedDL => {}
        data::State::PausedUP => {}
        _ => panic! {"torrent did not pause"},
    }

    first
        .hash()
        .resume(&api)
        .await
        .expect("did not resume torrent");
}

#[tokio::test]
async fn set_category() {
    let (api, torrent) = get_first_torrent().await;

    // wait for add_category test to create this test
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;

    dbg! {&torrent};
    let response = torrent.set_category(&api, "ADD_CATEGORY").await;
    dbg! {&response};
    response.unwrap();
}

#[tokio::test]
async fn add_category() {
    let api = default_api().await.unwrap();
    let response = api.add_category("ADD_CATEGORY", "E:\\testpath").await;
    dbg! {&response};
    response.unwrap();
}

#[tokio::test]
async fn get_all_categories() {
    let api = default_api().await.unwrap();
    let cats = api.get_all_categories().await;
    dbg! {&cats};
    cats.unwrap();
}

#[tokio::test]
async fn get_global_transfer_info() {
    let api = default_api().await.unwrap();
    let cats = api.get_global_transfer_info().await;
    dbg! {&cats};
    cats.unwrap();
}

#[tokio::test]
async fn get_alternate_speed_limits_state() {
    let api = default_api().await.unwrap();
    let cats = api.get_alternate_speed_limits_state().await;
    dbg! {&cats};
    cats.unwrap();
}
