#![allow(dead_code)]

use bytes::{BufMut,Buf};
use prost::Message;
use std::io;



#[derive(Debug, Copy, Clone, PartialEq, Eq)]
// #[repr(C, packed)] // https://github.com/rust-lang/rust/issues/82523 这样要求header字段本身以对齐。
#[repr(C)]
pub struct Header {
    pub cmd: u32,
    pub size: u32, // header + body
    pub opt: u32,
}

pub const fn header_len() -> usize {
    std::mem::size_of::<Header>()
}

impl Header {
    pub fn new(cmd: u32, size: u32, opt: u32) -> Self {
        Header {
            cmd,
            size,
            opt
        }
    }

    pub fn from_bytes(s: &[u8]) -> Result<Self, io::Error> {
        if s.len() != std::mem::size_of::<Header>() {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "need 12"));
        }
        let p = s.as_ptr();
        let p = p as *const Header;
        unsafe {
            Ok(*p)
        }
    }

    pub fn from_buf(s: &mut dyn Buf) -> Result<Self, io::Error> {
        if s.remaining() < header_len() {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "at least 12 bytes need"));
        }
        let a = &mut [0u8; header_len()];
        s.copy_to_slice(a);
        Self::from_bytes(a)
    }

    pub fn as_bytes(&self) -> &[u8] {
        let p: *const Header = self;
        let p = p as *const u8;
        unsafe{ 
            std::slice::from_raw_parts(p, std::mem::size_of::<Header>()) 
        }
    }
}

pub trait MarshalMsg {
    fn marshal_field(&self, cmd: u32, opt: u32, dst: &mut dyn BufMut);
    fn marshal_header(&self, header: &Header, dst: &mut dyn BufMut);
}

impl<T> MarshalMsg for T where T: Message {
    fn marshal_field(&self, cmd: u32, opt: u32, dst: &mut dyn BufMut) {
        let header = Header::new(cmd,
            (self.encoded_len() + std::mem::size_of::<Header>()) as u32,
            opt);
        dst.put_slice(header.as_bytes());
        dst.put_slice(self.encode_to_vec().as_slice());
    }

    fn marshal_header(&self, header: &Header, dst: &mut dyn BufMut) {
        dst.put_slice(header.as_bytes());
        dst.put_slice(self.encode_to_vec().as_slice());
    }
}

#[cfg(test)]
mod tests {
    use bytes::{BytesMut, BufMut};
    use super::*;
    use crate::protocol::MsgHelloReq;
    use prost::Message;

    #[test]
    fn header_asbytes() {
        let h = Header::new(0x01020304, 0, 0);
        let s = h.as_bytes();
        let mut a = [0u8; 12];
        a[0] = 0x4;
        a[1] = 0x3;
        a[2] = 0x2;
        a[3] = 0x1;
        assert_eq!(s, a);

        let n = Header::from_bytes(&a).unwrap();
        assert_eq!(h, n);
    }


    fn msg_marshal() {
        let mut buf = BytesMut::with_capacity(1024);
        let msg = MsgHelloReq{
          hello: 1  
        };
        let h = Header::new(0x1234, (header_len()+ msg.encoded_len()) as u32, 0x89AB);
        buf.put_slice(h.as_bytes());
        msg.encode(&mut buf).unwrap();

        let nh = Header::from_buf(&mut buf).unwrap();
        let nm =  MsgHelloReq::decode(buf).unwrap();
        
        assert_eq!(h, nh);
        assert_eq!(msg, nm);

        let mut h = Header::new(1,2,3);

        let s = &mut h.size;
        *s = 10;

        assert_eq!(h.size, 10);
    }
}