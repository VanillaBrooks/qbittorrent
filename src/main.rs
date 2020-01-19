use qbittorrent;
use qbittorrent::api;
use qbittorrent::traits::*;
use tokio;

#[tokio::main]
async fn main() {
    let api_: api::Api = api::Api::new("admin", "adminadminadmin", "http://localhost:8080")
        .await
        .unwrap();

    dbg! {&api_};

    let x = api_.get_torrent_list().await.unwrap();
    // let single = vec![x[0].hash().clone(), x[1].hash().clone()];
    let single: &qbittorrent::data::Torrent = &x[0];

    dbg! {&single};

    // let ans = single.resume(&api_).await;
    // let tags = vec!["TEST_TAG_1".into(), "TEST_TAG_2".into()];
    // let ans = x.add_tag(&api_, &tags.as_slice()).await;
    let ans = single.add_tag(&api_, &["test".into(), "_____another_tag".into()]).await;

    dbg!{ans};
}
