use qbittorrent;
use qbittorrent::api;
use qbittorrent::error::Error;
use qbittorrent::traits::*;
use tokio;

#[tokio::main]
async fn main() {
    let api: api::Api = api::Api::new("admin", "adminadminadmin", "http://localhost:8080")
        .await
        .unwrap();
}
