mod log;
mod protocol;
mod service;
mod utils;

use anyhow::Result;
use bytes::Bytes;
use futures::{SinkExt, StreamExt};
use tokio::net::TcpListener;
use tokio::io::AsyncWriteExt;
use tracing::*;

use protocol::*;
use service::*;

#[tokio::main]
async fn main() -> Result<()> {
    log::init_log();

    let addr = "0:9527";
    let listener = TcpListener::bind(addr).await?;
    info!(port = listener.local_addr()?.port(), "listen at:");

    let service = Service::new(1i32);
    loop {
        let (conn, remote) = listener.accept().await?;
        info!("client connected {}", remote);
        let mut transport = bind_transport(conn);
        let svc = service.clone();

        tokio::spawn(async move {
            while let Some(Ok(data)) = transport.next().await {
                info!(len = data.len(), "get data");
                let rsp = svc.handle(data).await;
                if rsp.is_err() {
                    break;
                }
            }
            transport.into_inner().shutdown().await;
        });
    }
    Ok(())
}
