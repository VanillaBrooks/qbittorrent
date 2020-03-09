use qbittorrent as qbit;
use tokio;

#[tokio::main]
async fn main() {
    let _api: qbit::api::Api =
        qbit::api::Api::new("admin", "adminadmin", "http://192.168.86.139:8080")
            .await
            .unwrap();
    let torrents = _api.get_torrent_list().await;
    dbg! {torrents};
}
