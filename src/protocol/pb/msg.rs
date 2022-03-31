#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAliveReq {
    #[prost(uint32, tag="1")]
    pub echo: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAliveRsp {
    #[prost(uint32, tag="1")]
    pub echo: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgHelloReq {
    #[prost(uint32, tag="1")]
    pub hello: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgHelloRsp {
    #[prost(uint32, tag="1")]
    pub rst: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgByeReq {
    #[prost(uint32, tag="1")]
    pub bye: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgByeRsp {
    #[prost(uint32, tag="1")]
    pub rst: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgExampleReq {
    #[prost(message, repeated, tag="1")]
    pub opts: ::prost::alloc::vec::Vec<Opt>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgExampleRsp {
    #[prost(uint32, tag="1")]
    pub rst: u32,
    #[prost(string, tag="2")]
    pub desc: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Opt {
    #[prost(oneof="opt::Data", tags="1, 2")]
    pub data: ::core::option::Option<opt::Data>,
}
/// Nested message and enum types in `Opt`.
pub mod opt {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Data {
        #[prost(message, tag="1")]
        One(super::One),
        #[prost(message, tag="2")]
        Two(super::Two),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct One {
    #[prost(uint32, tag="1")]
    pub x: u32,
    #[prost(uint32, tag="2")]
    pub y: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Two {
    #[prost(string, tag="1")]
    pub a: ::prost::alloc::string::String,
    #[prost(enumeration="two::TwoType", tag="2")]
    pub ttype: i32,
}
/// Nested message and enum types in `Two`.
pub mod two {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum TwoType {
        First = 0,
        Second = 1,
    }
}
