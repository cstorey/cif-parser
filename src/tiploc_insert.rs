use std::borrow::Cow;

use nom::{bytes::streaming::*, IResult};

use crate::errors::CIFParseError;
use crate::helpers::*;
use crate::tiploc::Tiploc;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TiplocInsert<'a> {
    pub tiploc: Tiploc<'a>,
    pub nlc: Cow<'a, str>,
    pub nlc_check: Cow<'a, str>,
    pub tps_description: Cow<'a, str>,
    pub stanox: Cow<'a, str>,
    pub crs: Option<Cow<'a, str>>,
    pub nlc_desc: Option<Cow<'a, str>>,
}

pub(super) fn parse_tiploc_insert<'a>(
) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], TiplocInsert, CIFParseError> {
    |i: &'a [u8]| -> IResult<&'a [u8], TiplocInsert, CIFParseError> {
        let (i, _) = tag("TI")(i)?;
        let (i, tiploc) = Tiploc::parse(i)?;
        let (i, _) = string(2usize)(i)?; // `capitals`
        let (i, nlc) = mandatory(string(6usize))(i)?;
        let (i, nlc_check) = mandatory(string(1usize))(i)?;
        let (i, tps_description) = mandatory(string(26usize))(i)?;
        let (i, stanox) = mandatory(string(5usize))(i)?;
        let (i, _po_code) = string(4usize)(i)?;
        let (i, crs) = string(3usize)(i)?;
        let (i, nlc_desc) = string(16usize)(i)?;
        let (i, _spare) = string(8)(i)?;

        Ok((
            i,
            TiplocInsert {
                tiploc: tiploc,
                nlc: nlc.into(),
                nlc_check: nlc_check.into(),
                tps_description: tps_description.into(),
                stanox: stanox.into(),
                crs: crs.map(Into::into),
                nlc_desc: nlc_desc.map(Into::into),
            },
        ))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_parse_tiploc_insert() {
        let p = parse_tiploc_insert();
        let hdr =
            b"TIBLTNODR24853600DBOLTON-UPON-DEARNE        24011   0BTDBOLTON ON DEARNE        ";
        assert_eq!(80, hdr.len());
        let (rest, insert) = p(hdr).expect("parse_header");
        assert_eq!(String::from_utf8_lossy(rest), "");
        assert_eq!(
            insert,
            TiplocInsert {
                tiploc: Tiploc::from_str("BLTNODR"),
                nlc: "853600".into(),
                nlc_check: "D".into(),
                tps_description: "BOLTON-UPON-DEARNE".into(),
                stanox: "24011".into(),
                crs: Some("BTD".into()),
                nlc_desc: Some("BOLTON ON DEARNE".into()),
            }
        )
    }

    #[test]
    fn should_parse_example_2() {
        let insert =
            b"TIAACHEN 00081601LAACHEN                    00005   0                           ";
        let _ = parse_tiploc_insert()(insert).expect("parse insert");
    }
}
