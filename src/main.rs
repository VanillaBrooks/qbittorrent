use qbittorrent as qbit;
use tokio;

#[tokio::main]
async fn main() {
    let _api: qbit::api::Api =
        qbit::api::Api::new("admin", "adminadminadmin", "http://localhost:8080")
            .await
            .unwrap();
}
