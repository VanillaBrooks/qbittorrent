use qbittorrent;
use qbittorrent::api;

fn main() {
    let api_ = api::Api::new("brooks", "brooksbrooksbrooks", "http://localhost:8080").unwrap();
    // dbg!{api_};
    // dbg!{api_.application_version()};
    // dbg!{api_.api_version()};
    // dbg!{api_.build_info()};
    let log = api::LogRequest::default();
    dbg! {api_.get_log(&log)};
}
