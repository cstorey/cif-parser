use chrono::{Date, NaiveTime};
use chrono_tz::Tz;
use nom::{
    branch::alt, bytes::streaming::*, character::is_space, character::streaming::*,
    combinator::map, IResult,
};

use crate::errors::CIFParseError;
use crate::helpers::{date, mandatory_str, string, time};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum FullOrUpdate {
    Full,
    Update,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Header<'a> {
    pub file_mainframe_identity: &'a str,
    // Thanks to DST shenanigans, we need to do our own handling of
    // times.
    pub extracted_date: Date<Tz>,
    pub extracted_time: NaiveTime,
    pub current_file: &'a str,
    pub last_file: Option<&'a str>,
    pub update_indicator: FullOrUpdate,
    pub version: &'a str,
    pub user_start_date: Date<Tz>,
    pub user_end_date: Date<Tz>,
}

pub(super) fn parse_header<'a>() -> impl Fn(&'a [u8]) -> IResult<&'a [u8], Header, CIFParseError> {
    |i: &'a [u8]| -> IResult<&'a [u8], Header, CIFParseError> {
        let (i, _) = tag("HD")(i)?;
        let (i, file_mainframe_identity) = mandatory_str("file_mainframe_identity", 20usize)(i)?;
        let (i, extract_date) = date()(i)?;
        let (i, extract_time) = time()(i)?;
        let (i, current_file) = mandatory_str("current_file", 7usize)(i)?;
        let (i, last_file) = string(7usize)(i)?;
        let (i, update_indicator) = alt((
            map(char('U'), |_| FullOrUpdate::Update),
            map(char('F'), |_| FullOrUpdate::Full),
        ))(i)?;
        let (i, version) = mandatory_str("version", 1usize)(i)?;
        let (i, user_start_date) = date()(i)?;
        let (i, user_end_date) = date()(i)?;
        let (i, _spare) = take_while_m_n(20, 20, is_space)(i)?;

        Ok((
            i,
            Header {
                file_mainframe_identity: file_mainframe_identity,
                extracted_date: extract_date,
                extracted_time: extract_time,
                current_file: current_file,
                last_file: last_file,
                update_indicator: update_indicator,
                version: version,
                user_start_date: user_start_date,
                user_end_date: user_end_date,
            },
        ))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_parse_full_header() {
        let p = parse_header();
        let hdr =
            b"HDTPS.UDFROC1.PD1907050507191939DFROC2S       FA050719040720                    ";
        let (rest, _val) = p(hdr).expect("parse_header");
        assert_eq!(String::from_utf8_lossy(rest), "");
    }
}
