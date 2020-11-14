use nom::{bytes::streaming::*, character::is_space, IResult};

use crate::errors::CIFParseError;
use crate::helpers::{mandatory_str, string};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TiplocAmend<'a> {
    pub tiploc: &'a str,
    pub nlc: &'a str,
    pub nlc_check: &'a str,
    pub tps_description: &'a str,
    pub stanox: &'a str,
    pub crs: Option<&'a str>,
    pub nlc_desc: Option<&'a str>,
    pub new_tiploc: Option<&'a str>,
}

pub(super) fn parse_tiploc_amend<'a>(
) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], TiplocAmend, CIFParseError> {
    |i: &'a [u8]| -> IResult<&'a [u8], TiplocAmend, CIFParseError> {
        let (i, _) = tag("TA")(i)?;
        let (i, tiploc) = mandatory_str("tiploc", 7usize)(i)?;
        let (i, _) = mandatory_str("_", 2usize)(i)?; // `capitals`
        let (i, nlc) = mandatory_str("nlc", 6usize)(i)?;
        let (i, nlc_check) = mandatory_str("nlc_check", 1usize)(i)?;
        let (i, tps_description) = mandatory_str("tps_description", 26usize)(i)?;
        let (i, stanox) = mandatory_str("stanox", 5usize)(i)?;
        let (i, _po_code) = mandatory_str("_po_code", 4usize)(i)?;
        let (i, crs) = string(3usize)(i)?;
        let (i, nlc_desc) = string(16usize)(i)?;
        let (i, new_tiploc) = string(7usize)(i)?;
        let (i, _spare) = take_while_m_n(1, 1, is_space)(i)?;

        Ok((
            i,
            TiplocAmend {
                tiploc,
                nlc,
                nlc_check,
                tps_description,
                stanox,
                crs,
                nlc_desc,
                new_tiploc,
            },
        ))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use nom::combinator::complete;

    #[test]
    fn should_parse_tiploc_amend_a() {
        let mut p = complete(parse_tiploc_amend());
        let hdr =
            b"TAMBRK94200590970AMILLBROOK SIG E942        86536   0                           ";
        assert_eq!(80, hdr.len());
        let (rest, insert) = p(hdr).expect("parse_header");
        assert_eq!(String::from_utf8_lossy(rest), "");
        assert_eq!(
            insert,
            TiplocAmend {
                tiploc: "MBRK942",
                nlc: "590970",
                nlc_check: "A",
                tps_description: "MILLBROOK SIG E942",
                stanox: "86536",
                crs: None,
                nlc_desc: None,
                new_tiploc: None,
            }
        )
    }
}
