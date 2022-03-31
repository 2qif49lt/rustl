#![allow(dead_code)]

pub const CMD_MIN: u32 = 1000;
pub const CMD_MAX: u32 = 2000;

pub const CMD_ALIVE_REQ: u32 = CMD_MIN+ 10;
pub const CMD_ALIVE_RSP: u32 = CMD_ALIVE_REQ + 1;

pub const CMD_HELLO_REQ: u32 = CMD_MIN + 20;
pub const CMD_HELLO_RSP: u32 = CMD_HELLO_REQ + 1;

pub const CMD_BYE_REQ: u32 = CMD_MIN + 30;
pub const CMD_BYE_RSP: u32 = CMD_BYE_REQ + 1;

pub const CMD_EXAMPLE_REQ: u32 = CMD_MIN + 40;
pub const CMD_EXAMPLE_RSP: u32 = CMD_EXAMPLE_REQ + 1;
