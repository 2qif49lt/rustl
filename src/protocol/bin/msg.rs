use super::Header;
// use super::protocol::Header;  // follows are alright
// use crate::msg::protocol::Header;
// use crate::msg::Header;

#[derive(Debug, Copy, Clone)]
#[repr(C, packed)]
pub struct MsgAliveReq {
    pub head: Header,
    pub echo: u32,
}

#[derive(Debug, Copy, Clone)]
#[repr(C, packed)]
pub struct MsgAliveRsp {
    pub head: Header,
    pub echo: u32,
}
