use qbittorrent;
use qbittorrent::api;
use tokio;

#[tokio::main]
async fn main() {
    let api_: api::Api = api::Api::new("admin", "adminadminadmin", "http://localhost:9952")
        .await
        .unwrap();

    dbg! {&api_};

    // let x = api_.get_torrent_list().await;//.unwrap();
    // dbg! {&x};
    // let x = x.unwrap();
    // let y = &x[0];
    // dbg! {&y};

    // dbg!{api_.toggle_alternative_speed_limits().await};

    // let z = y.trackers(&api_).await;
    // dbg! {&z};
    // let z = y.resume(&api_).await;
    // dbg! {&z};

    let data = api::TorrentDownloadBuilder::default()
        .urls("magnet:?xt=urn:btih:4328f76ad774bf1c7a90a5007e0369a6a55c0404&dn=Hookuphotshot+-+E226+-+Viva+Athena+720p+HD+mp4&tr=http%3A%2F%2Ftracker.trackerfix.com%3A80%2Fannounce&tr=udp%3A%2F%2F9.rarbg.me%3A2730&tr=udp%3A%2F%2F9.rarbg.to%3A2760")
        .build()
        .unwrap();

    let x = api_.add_new_torrent(&data).await;
    dbg! {x};
}
