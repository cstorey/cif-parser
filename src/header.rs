use std::borrow::Cow;

use nom::{
    branch::alt, bytes::streaming::*, character::is_space, character::streaming::*,
    combinator::map, IResult,
};

use crate::errors::CIFParseError;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum FullOrUpdate {
    Full,
    Update,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Header<'a> {
    file_mainframe_identity: Cow<'a, str>,
    extract_date: Cow<'a, str>,
    extract_time: Cow<'a, str>,
    current_file: Cow<'a, str>,
    last_file: Cow<'a, str>,
    update_indicator: FullOrUpdate,
    version: Cow<'a, str>,
    user_start_date: Cow<'a, str>,
    user_end_date: Cow<'a, str>,
}

pub(super) fn parse_header<'a>() -> impl Fn(&'a [u8]) -> IResult<&'a [u8], Header, CIFParseError> {
    |i: &'a [u8]| -> IResult<&'a [u8], Header, CIFParseError> {
        let (i, _) = tag("HD")(i)?;
        let (i, file_mainframe_identity) = take(20usize)(i)?;
        let (i, extract_date) = take(6usize)(i)?;
        let (i, extract_time) = take(4usize)(i)?;
        let (i, current_file) = take(7usize)(i)?;
        let (i, last_file) = take(7usize)(i)?;
        let (i, update_indicator) = alt((
            map(char('U'), |_| FullOrUpdate::Update),
            map(char('F'), |_| FullOrUpdate::Full),
        ))(i)?;
        let (i, version) = take(1usize)(i)?;
        let (i, user_start_date) = take(6usize)(i)?;
        let (i, user_end_date) = take(6usize)(i)?;
        let (i, _spare) = take_while_m_n(20, 20, is_space)(i)?;

        Ok((
            i,
            Header {
                file_mainframe_identity: String::from_utf8_lossy(file_mainframe_identity),
                extract_date: String::from_utf8_lossy(extract_date),
                extract_time: String::from_utf8_lossy(extract_time),
                current_file: String::from_utf8_lossy(current_file),
                last_file: String::from_utf8_lossy(last_file),
                update_indicator: update_indicator,
                version: String::from_utf8_lossy(version),
                user_start_date: String::from_utf8_lossy(user_start_date),
                user_end_date: String::from_utf8_lossy(user_end_date),
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