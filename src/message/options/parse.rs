use message::options::{Options};
use message::{Result, Error};
use nom::{be_u8};

use num::FromPrimitive;

pub fn parse(bytes: &[u8]) -> Result<Vec<Options>> {
    Ok(vec![])
}

named!(dhcp_option<&[u8], (Option<Options>, Vec<u8>)>, alt!(
        chain!(
            tag!([0u8]),
            || { (Some(Options::Pad), vec![]) }
        ) |
        chain!(
            tag!([255u8]),
            || { (Some(Options::End), vec![]) }
        ) |
        chain!(
            code: be_u8 ~
            data: length_value!(be_u8, be_u8),
            || { (FromPrimitive::from_u8(code), data) }
        )
    )
);
