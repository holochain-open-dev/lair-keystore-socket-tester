use clap::Parser;
use lair_keystore_api::dependencies::sodoken::BufRead;
use lair_keystore_api::ipc_keystore_connect;

use url::Url;

/// Verify a holochain conductor is running and has a reachable app websocket at a given URL
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Lair Keystore Socket URL
    socket_url: String,

    /// Lair Keystore Password
    password: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let socket_url_parsed = Url::parse(args.socket_url.clone().as_str()).unwrap_or_else(|_| panic!("socket_url {:?} is not a valid URL",
            args.socket_url.clone()));
    let password_bufread: BufRead = args.password.as_bytes().into();

    let _lair_client = ipc_keystore_connect(socket_url_parsed, password_bufread)
        .await
        .map_err(|e| format!("Failed to connect to lair keystore {:?}", e))
        .unwrap();

    println!(
        "Successfully connected to lair keystore at {:?}",
        args.socket_url
    );
}
