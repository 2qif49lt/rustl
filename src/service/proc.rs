use super::Handler;
use crate::protocol::*;
use crate::{header_len, Header};

use anyhow::Result;
use async_trait::async_trait;
use bytes::{BufMut, Bytes, BytesMut};
use prost::Message;
use rand::Rng;
use tokio::time::{sleep, Duration};

async fn cost() {
    let dura = rand::thread_rng().gen_range(50..100);
    sleep(Duration::from_millis(dura)).await;
}

fn serialize<Body: Message>(cmd: u32, opt: u32, body: Body) -> Bytes {
    let h = Header::new(cmd, header_len() as u32 + body.encoded_len() as u32, opt);
    let mut buf = BytesMut::new();
    buf.reserve(h.size as usize);
    buf.put_slice(h.as_bytes());
    body.encode(&mut buf).unwrap();
    buf.freeze()
}

#[async_trait]
impl Handler for MsgAliveReq {
    async fn proc(&self, opt: u32) -> Result<Option<Vec<u8>>> {
        let m = MsgAliveRsp { echo: self.echo };
        cost().await;
        let rsp = serialize(CMD_ALIVE_RSP, 0, m);
        Ok(Some(rsp.to_vec()))
    }
}

#[async_trait]
impl Handler for MsgHelloReq {
    async fn proc(&self, opt: u32) -> Result<Option<Vec<u8>>> {
        let m = MsgHelloRsp { rst: 0 };
        cost().await;
        let rsp = serialize(CMD_HELLO_RSP, 0, m);
        Ok(Some(rsp.to_vec()))
    }
}

#[async_trait]
impl Handler for MsgByeReq {
    async fn proc(&self, opt: u32) -> Result<Option<Vec<u8>>> {
        let m = MsgByeRsp { rst: 0 };
        cost().await;
        let rsp = serialize(CMD_BYE_RSP, 0, m);
        Ok(Some(rsp.to_vec()))
    }
}

#[async_trait]
impl Handler for MsgExampleReq {
    async fn proc(&self, opt: u32) -> Result<Option<Vec<u8>>> {
        let m = MsgExampleRsp { rst: 0 , desc: "succ".to_string()};
        cost().await;
        let rsp = serialize(CMD_EXAMPLE_RSP, 0, m);
        Ok(Some(rsp.to_vec()))
    }
}
