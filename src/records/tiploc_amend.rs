use std::borrow::Cow;

use nom::{bytes::streaming::*, character::is_space, error::*, IResult};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TiplocAmend<'a> {
    tiploc: Cow<'a, str>,
    nlc: Cow<'a, str>,
    nlc_check: Cow<'a, str>,
    tps_description: Cow<'a, str>,
    stanox: Cow<'a, str>,
    crs: Cow<'a, str>,
    nlc_desc: Cow<'a, str>,
    new_tiploc: Cow<'a, str>,
}

pub(super) fn parse_tiploc_amend<'a, E: ParseError<&'a [u8]>>(
) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], TiplocAmend, E> {
    |i: &'a [u8]| -> IResult<&'a [u8], TiplocAmend, E> {
        let (i, _) = tag("TA")(i)?;
        let (i, tiploc) = take(7usize)(i)?;
        let (i, _) = take(2usize)(i)?; // `capitals`
        let (i, nlc) = take(6usize)(i)?;
        let (i, nlc_check) = take(1usize)(i)?;
        let (i, tps_description) = take(26usize)(i)?;
        let (i, stanox) = take(5usize)(i)?;
        let (i, _po_code) = take(4usize)(i)?;
        let (i, crs) = take(3usize)(i)?;
        let (i, nlc_desc) = take(16usize)(i)?;
        let (i, new_tiploc) = take(7usize)(i)?;
        let (i, _spare) = take_while_m_n(1, 1, is_space)(i)?;

        Ok((
            i,
            TiplocAmend {
                tiploc: String::from_utf8_lossy(tiploc),
                nlc: String::from_utf8_lossy(nlc),
                nlc_check: String::from_utf8_lossy(nlc_check),
                tps_description: String::from_utf8_lossy(tps_description),
                stanox: String::from_utf8_lossy(stanox),
                crs: String::from_utf8_lossy(crs),
                nlc_desc: String::from_utf8_lossy(nlc_desc),
                new_tiploc: String::from_utf8_lossy(new_tiploc),
            },
        ))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use nom::combinator::complete;

    #[test]
    fn should_parse_tiploc_amend() {
        let p = complete(parse_tiploc_amend::<VerboseError<_>>());
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
                tps_description: "MILLBROOK SIG E942        ".into(),
                stanox: "86536".into(),
                crs: "   ".into(),
                nlc_desc: "                ".into(),
                new_tiploc: "       ".into(),
            }
        )
    }
}
