use std::net::SocketAddr;

use anyhow::{anyhow, Result};
use s2n_quic::{client::Connect, Client};
use tokio::io::{copy, stdin, stdout};

const CERT_PEM: &str = include_str!("../fixtures/cert.pem");

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::builder()
        .with_tls(CERT_PEM)?
        .with_io("0.0.0.0:0")?
        .start()
        .map_err(|e| anyhow!("Failed to start client. Error: {e}"))?;

    let addr: SocketAddr = "127.0.0.1:4433".parse()?;
    let connect = Connect::new(addr).with_server_name("localhost");
    let mut connection = client.connect(connect).await?;

    // ensure the connection doesn't time out with inactivity
    connection.keep_alive(true)?;

    // open a new stream and split the receiving and sending sides
    let stream = connection.open_bidirectional_stream().await?;
    let (mut receive_stream, mut send_stream) = stream.split();

    println!("Connected to {}", connection.remote_addr()?);

    // spawn a task that copies responses from the server to stdout
    tokio::spawn(async move {
        let mut stdout = stdout();
        if let Err(e) = copy(&mut receive_stream, &mut stdout).await {
            println!("Failed to copy data from server. Error: {e}");
        }
    });

    // copy data from stdin and send it to the server
    let mut stdin = stdin();
    copy(&mut stdin, &mut send_stream).await?;

    Ok(())
}
