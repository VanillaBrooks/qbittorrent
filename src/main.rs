use qbittorrent;
use qbittorrent::api;
use tokio;

#[tokio::main]
async fn main() {
    let api_ = api::Api::new("brooks", "brooksbrooksbrooks", "http://localhost:9952")
        .await
        .unwrap();

    let x = api_.get_torrent_list().await.unwrap();
    let y = &x[0];
    dbg! {y};

    // let z = y.trackers(&api_).await;
    // dbg! {&z};
    let z = y.resume(&api_).await;
    dbg! {&z};


}
