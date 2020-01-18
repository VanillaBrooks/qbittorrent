use super::api::Api;

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

// references versions
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
