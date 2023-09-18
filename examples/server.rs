use anyhow::{anyhow, Result};
use s2n_quic::Server;

const CERT_PEM: &str = include_str!("../fixtures/cert.pem");
const KEY_PEM: &str = include_str!("../fixtures/key.pem");

#[tokio::main]
async fn main() -> Result<()> {
    let addr = "127.0.0.1:4433";
    let mut server = Server::builder()
        .with_tls((CERT_PEM, KEY_PEM))?
        .with_io(addr)?
        .start()
        .map_err(|e| anyhow!("Failed to start server. Error: {e}"))?;
    println!("Server listening on {}", addr);

    while let Some(mut conn) = server.accept().await {
        println!("Accepted connection from {:?}", conn.remote_addr()?);

        tokio::spawn(async move {
            while let Ok(Some(mut stream)) = conn.accept_bidirectional_stream().await {
                println!(
                    "Accepted bidirectional stream from {:?}",
                    stream.connection().remote_addr()?
                );

                // echo back
                tokio::spawn(async move {
                    while let Ok(Some(data)) = stream.receive().await {
                        stream.send(data).await?;
                    }
                    Ok::<(), anyhow::Error>(())
                });
            }
            Ok::<(), anyhow::Error>(())
        });
    }
    Ok(())
}
