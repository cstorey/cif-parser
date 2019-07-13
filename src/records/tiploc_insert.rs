use std::borrow::Cow;

use nom::{bytes::streaming::*, character::is_space, error::*, IResult};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TiplocInsert<'a> {
    tiploc: Cow<'a, str>,
    nlc: Cow<'a, str>,
    nlc_check: Cow<'a, str>,
    tps_description: Cow<'a, str>,
    stanox: Cow<'a, str>,
    crs: Cow<'a, str>,
    nlc_desc: Cow<'a, str>,
}

pub(super) fn parse_tiploc_insert<'a, E: ParseError<&'a [u8]>>(
) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], TiplocInsert, E> {
    |i: &'a [u8]| -> IResult<&'a [u8], TiplocInsert, E> {
        let (i, _) = tag("TI")(i)?;
        let (i, tiploc) = take(7usize)(i)?;
        let (i, _) = take(2usize)(i)?; // `capitals`
        let (i, nlc) = take(6usize)(i)?;
        let (i, nlc_check) = take(1usize)(i)?;
        let (i, tps_description) = take(26usize)(i)?;
        let (i, stanox) = take(5usize)(i)?;
        let (i, _po_code) = take(4usize)(i)?;
        let (i, crs) = take(3usize)(i)?;
        let (i, nlc_desc) = take(16usize)(i)?;
        let (i, _spare) = take_while_m_n(8, 8, is_space)(i)?;

        Ok((
            i,
            TiplocInsert {
                tiploc: String::from_utf8_lossy(tiploc),
                nlc: String::from_utf8_lossy(nlc),
                nlc_check: String::from_utf8_lossy(nlc_check),
                tps_description: String::from_utf8_lossy(tps_description),
                stanox: String::from_utf8_lossy(stanox),
                crs: String::from_utf8_lossy(crs),
                nlc_desc: String::from_utf8_lossy(nlc_desc),
            },
        ))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_parse_tiploc_insert() {
        let p = parse_tiploc_insert::<VerboseError<_>>();
        let hdr =
            b"TIBLTNODR24853600DBOLTON-UPON-DEARNE        24011   0BTDBOLTON ON DEARNE        ";
        assert_eq!(80, hdr.len());
        let (rest, insert) = p(hdr).expect("parse_header");
        assert_eq!(String::from_utf8_lossy(rest), "");
        assert_eq!(
            insert,
            TiplocInsert {
                tiploc: "BLTNODR".into(),
                nlc: "853600".into(),
                nlc_check: "D".into(),
                tps_description: "BOLTON-UPON-DEARNE        ".into(),
                stanox: "24011".into(),
                crs: "BTD".into(),
                nlc_desc: "BOLTON ON DEARNE".into(),
            }
        )
    }
}
