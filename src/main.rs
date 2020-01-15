use qbittorrent;
use qbittorrent::api;
use tokio;

#[tokio::main]
async fn main() {
    let api_ = api::Api::new("admin", "adminadminadmin", "http://localhost:9952")
        .await
        .unwrap();

    dbg!{&api_};

    let x = api_.get_torrent_list().await;//.unwrap();
    dbg! {&x};
    // let x = x.unwrap();
    // let y = &x[0];
    // dbg! {&y};

    // dbg!{api_.toggle_alternative_speed_limits().await};

    // let z = y.trackers(&api_).await;
    // dbg! {&z};
    // let z = y.resume(&api_).await;
    // dbg! {&z};
}