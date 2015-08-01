
#![feature(ip_addr)]

#[macro_use] extern crate nom;
use std::str;
use std::borrow::{ToOwned};
use std::convert::{From};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use nom::{IResult, be_u8, be_u16, be_u32};

fn take_rest(input: &[u8]) -> IResult<&[u8], &[u8]> {
    IResult::Done(b"", input)
}


struct Message<'a> {
    op: u8,
    htype: u8,
    hlen: u8,
    hops: u8,
    xid: u32,
    secs: u16,
    flags: u16,
    ciaddr: IpAddr,
    yiaddr: IpAddr,
    siaddr: IpAddr,
    giaddr: IpAddr,
    chaddr: Vec<u8>,  // 16 bytes
    sname: &'a str,  // 64 bytes
    file: &'a str,  // 128 bytes
    options: Option<Vec<u8>>,
}

fn null_terminated_slice_to_string(bytes: &[u8]) -> Result<&str, String> {
    let pos = match bytes.iter().position(|b| *b == 0u8) {
        Some(p) => p,
        None => return Err("NO NULL TERMINATION FOUND".into()),
    };
    match str::from_utf8(&bytes[0..pos]) {
        Ok(s) => Ok(s),
        Err(_) => Err("Could not get utf8 from bytes".into()),
    }
}

#[test]
fn test_parse_message() {
    let test_message: Vec<u8> = vec![
        1u8,                            // op
        2,                              // htype
        3,                              // hlen
        4,                              // ops
        5,
        6,
        7,
        8,                              // xid
        9,
        10,                             // secs
        11,
        12,                             // flags
        13,
        14,
        15,
        16,                             // ciaddr
        17,
        18,
        19,
        20,                             // yiaddr
        21,
        22,
        23,
        24,                             // siaddr
        25,
        26,
        27,
        28,                             // giaddr
        29,
        20,
        21,
        22,
        23,
        24,
        25,
        26,
        27,
        28,
        29,
    ];
}

named!(parse_message(&'a [u8]) -> Message<'a>,
    chain!(
        pop: be_u8 ~
        phtype: be_u8 ~
        phlen: be_u8 ~
        phops: be_u8 ~
        pxid: be_u32 ~
        psecs: be_u16 ~
        pflags: be_u16 ~
        pciaddr: be_u32 ~
        pyiaddr: be_u32 ~
        psiaddr: be_u32 ~
        pgiaddr: be_u32 ~
        pchaddr: take!(16) ~
        psname: map_res!(take!(64), null_terminated_slice_to_string) ~
        pfile: map_res!(take!(128), null_terminated_slice_to_string) ~
        poptions: take_rest,
    ||{
        Message {
            op: pop,
            htype: phtype,
            hlen: phlen,
            hops: phops,
            xid: pxid,
            secs: psecs,
            flags: pflags,
            ciaddr: IpAddr::V4(Ipv4Addr::from(pciaddr)),
            yiaddr: IpAddr::V4(Ipv4Addr::from(pyiaddr)),
            siaddr: IpAddr::V4(Ipv4Addr::from(psiaddr)),
            giaddr: IpAddr::V4(Ipv4Addr::from(pgiaddr)),
            chaddr: pchaddr.to_owned(),
            sname: psname,
            file: pfile,
            options: Some(poptions.to_owned()),
        }
    }
    )
);

#[test]
fn test_take_rest() {
    named!(parts<&[u8],(&str,&str)>,
        chain!(
            key: map_res!(tag!("abcd"), str::from_utf8) ~
            tag!(":") ~
            value: map_res!(take_rest, str::from_utf8),
            || {(key, value)}
        )
    );

    assert_eq!(parts(b"abcd:thisistherestofthestring"), IResult::Done(&b""[..], ("abcd", "thisistherestofthestring")));
}
