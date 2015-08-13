#![feature(ip_addr)]
#[macro_use] extern crate nom;

/// DHCP Protocol, RFC-2131
/// 
/// This crate implements DHCP message parsing, compatible with RFC 2131

mod message;

pub use message::Message;

