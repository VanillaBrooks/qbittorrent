use qbittorrent;
use qbittorrent::api;
use qbittorrent::traits::*;
use tokio;

#[tokio::main]
async fn main() {
    let api_: api::Api = api::Api::new("admin", "adminadminadmin", "http://localhost:4356")
        .await
        .unwrap();

    dbg! {&api_};

    let x = api_.get_torrent_list().await.unwrap();
    let single = vec![x[0].hash().clone(), x[1].hash().clone()];

    dbg! {&single};

    let ans = single.resume(&api_).await;

    dbg! {ans};
}
