use qbittorrent as qbit;
use reqwest::Url;
use tokio;

use clap::{App, Arg};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let matches = App::new("qBit CLI")
        .version("0.1")
        .about("Control qBit from CLI")
        .arg(
            Arg::with_name("url")
                .long("url")
                .value_name("URL")
                .help("Sets qBit base URL (e.g. https://127.0.0.1/qbit)")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("user")
                .short("u")
                .long("user")
                .value_name("USER")
                .help("Sets qBit username")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("pass")
                .short("p")
                .long("pass")
                .value_name("PASS")
                .help("Sets qBit password")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("b")
                .short("b")
                .long("basic")
                .help("Uses basic authentication"),
        )
        .subcommand(App::new("tagerror")
            .about("Tags torrents that trackers are not working")
            .arg(Arg::with_name("tag")
                .short("t")
                .default_value("notworking")
                .takes_value(true)
                .help("Uses the specified tag(s) (split by commas)"))
            .arg(Arg::with_name("all")
                .short("a")
                .help("Tags all non-working torrents (By default, trackers with no error messages are excluded)")))
        .get_matches();

    let _api: qbit::api::Api = qbit::api::Api::new(
        matches.is_present("b"),
        matches.value_of("user").unwrap(),
        matches.value_of("pass").unwrap(),
        Url::parse(matches.value_of("url").unwrap()).unwrap(),
    )
    .await
    .unwrap();

    if let Some(ref matches) = matches.subcommand_matches("tagerror") {
        _api.tag_error_trackers(matches.value_of("tag").unwrap(), matches.is_present("all"))
            .await
            .unwrap();
    }

    ()
}
