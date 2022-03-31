mod proc;

use async_trait::async_trait;
use std::sync::Arc;

use crate::protocol::CMD_BYE_REQ;
use crate::protocol::CMD_EXAMPLE_REQ;
use crate::protocol::Header;
use crate::protocol::MsgAliveReq;
use crate::protocol::MsgByeReq;
use crate::protocol::MsgExampleReq;
use crate::protocol::MsgHelloReq;
use crate::protocol::header_len;
use prost::Message;
use anyhow::Result;

#[async_trait]
pub trait Handler {
    // type Rsp;
    async fn proc(&self, opt: u32) -> Result<Option<Vec<u8>>>;
}

pub struct Service<Hdr> {
    inner: Arc<ServiceInner<Hdr>>,
}


impl<Hdr> Service<Hdr> {
    pub fn new(hdr: Hdr) -> Self {
        Self {
            inner: Arc::new(ServiceInner { hdr }),
        }
    }
    pub async fn handle(&self, data: Vec<u8>) -> Result<Option<Vec<u8>>> {
        dispatch(data).await
    }
}

impl<Hdr> Clone for Service<Hdr> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}
pub struct ServiceInner<Hdr> {
    hdr: Hdr,
}

async fn dispatch(data: Vec<u8>) -> Result<Option<Vec<u8>>> {
    let h = Header::from_bytes(&data[..header_len()])?;
    let rsp = match h.cmd {
        CMD_ALIVE_REQ => {
            let req = MsgAliveReq::decode(&data[header_len()..])?;
            req.proc(h.opt).await?
        },
        CMD_HELLO_REQ => {
            let req = MsgHelloReq::decode(&data[header_len()..])?;
            req.proc(h.opt).await?
        },
        CMD_BYE_REQ => {
            let req = MsgByeReq::decode(&data[header_len()..])?;
            req.proc(h.opt).await?
        },
        CMD_EXAMPLE_REQ => {
            let req = MsgExampleReq::decode(&data[header_len()..])?;
            req.proc(h.opt).await?
        },
        _ => Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "cmd invalid"))?
    };
    Ok(rsp)
}