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
        let (i, tiploc) = mandatory_str(7usize)(i)?;
        let (i, _) = mandatory_str(2usize)(i)?; // `capitals`
        let (i, nlc) = mandatory_str(6usize)(i)?;
        let (i, nlc_check) = mandatory_str(1usize)(i)?;
        let (i, tps_description) = mandatory_str(26usize)(i)?;
        let (i, stanox) = mandatory_str(5usize)(i)?;
        let (i, _po_code) = mandatory_str(4usize)(i)?;
        let (i, crs) = string(3usize)(i)?;
        let (i, nlc_desc) = string(16usize)(i)?;
        let (i, new_tiploc) = string(7usize)(i)?;
        let (i, _spare) = take_while_m_n(1, 1, is_space)(i)?;

        Ok((
            i,
            TiplocAmend {
                tiploc: tiploc,
                nlc: nlc,
                nlc_check: nlc_check,
                tps_description: tps_description,
                stanox: stanox,
                crs: crs,
                nlc_desc: nlc_desc,
                new_tiploc: new_tiploc,
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
        let p = complete(parse_tiploc_amend());
        let hdr =
            b"TAMBRK94200590970AMILLBROOK SIG E942        86536   0                           ";
        assert_eq!(80, hdr.len());
        let (rest, insert) = p(hdr).expect("parse_header");
        assert_eq!(String::from_utf8_lossy(rest), "");
        assert_eq!(
            insert,
            TiplocAmend {
                tiploc: "MBRK942".into(),
                nlc: "590970".into(),
                nlc_check: "A".into(),
                tps_description: "MILLBROOK SIG E942".into(),
                stanox: "86536".into(),
                crs: None,
                nlc_desc: None,
                new_tiploc: None,
            }
        )
    }
}
