use std::borrow::Cow;
use std::fs::File;
use std::path::PathBuf;

use failure::Fallible;
use memmap::Mmap;
use nom::{
    branch::alt,
    bytes::streaming::*,
    character::is_space,
    character::streaming::*,
    combinator::{cut, map, opt},
    error::*,
    sequence::terminated,
    Err, IResult,
};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "cif-parser", about = "CIF file parser")]
struct Opts {
    files: Vec<PathBuf>,
}
#[derive(Debug, Clone, Eq, PartialEq)]
enum Record<'a> {
    Header(Header<'a>),
    TiplocInsert(TiplocInsert<'a>),
    TiplocAmend(TiplocAmend<'a>),
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum FullOrUpdate {
    Full,
    Update,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Header<'a> {
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

#[derive(Debug, Clone, Eq, PartialEq)]
struct TiplocInsert<'a> {
    tiploc: Cow<'a, str>,
    nlc: Cow<'a, str>,
    nlc_check: Cow<'a, str>,
    tps_description: Cow<'a, str>,
    stanox: Cow<'a, str>,
    crs: Cow<'a, str>,
    nlc_desc: Cow<'a, str>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct TiplocAmend<'a> {
    tiploc: Cow<'a, str>,
    nlc: Cow<'a, str>,
    nlc_check: Cow<'a, str>,
    tps_description: Cow<'a, str>,
    stanox: Cow<'a, str>,
    crs: Cow<'a, str>,
    nlc_desc: Cow<'a, str>,
    new_tiploc: Cow<'a, str>,
}

fn parse<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], Record, E> {
    let p = alt((
        map(parse_header(), Record::Header),
        map(parse_tiploc_insert(), Record::TiplocInsert),
        map(parse_tiploc_amend(), Record::TiplocAmend),
    ));
    terminated(p, char('\n'))(i)
}

fn parse_header<'a, E: ParseError<&'a [u8]>>() -> impl Fn(&'a [u8]) -> IResult<&'a [u8], Header, E>
{
    |i: &'a [u8]| -> IResult<&'a [u8], Header, E> {
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

fn parse_tiploc_insert<'a, E: ParseError<&'a [u8]>>(
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

fn parse_tiploc_amend<'a, E: ParseError<&'a [u8]>>(
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

fn main() -> Fallible<()> {
    let opts = Opts::from_args();

    for f in opts.files {
        let fp = File::open(f)?;
        let mmap = unsafe { Mmap::map(&fp)? };
        let mut i: &[u8] = &mmap;
        loop {
            match parse::<VerboseError<_>>(&i) {
                Ok((rest, val)) => {
                    i = rest;
                    println!("Ok: {:#?}", val)
                }

                Err(Err::Incomplete(need)) => {
                    println!("Needed: {:?}", need);
                    break;
                }
                Err(Err::Error(err)) => {
                    println!("Error:");
                    show_error(err);
                    break;
                }
                Err(Err::Failure(err)) => {
                    println!("Failure:");
                    show_error(err);
                    break;
                }
            }
        }
    }

    Ok(())
}

fn show_error(err: VerboseError<&[u8]>) {
    const SNIPPET_LEN: usize = 120;
    for (i, kind) in err.errors {
        println!(
            "Err: {:?}: {:?}{}",
            kind,
            String::from_utf8_lossy(&i[..SNIPPET_LEN]),
            if i.len() < SNIPPET_LEN { "" } else { "…" }
        );
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use nom::combinator::complete;

    #[test]
    fn should_parse_full_header() {
        let p = complete(parse_header::<VerboseError<_>>());
        let hdr =
            b"HDTPS.UDFROC1.PD1907050507191939DFROC2S       FA050719040720                    ";
        let (rest, _val) = p(hdr).expect("parse_header");
        assert_eq!(rest, b"");
    }

    #[test]
    fn should_parse_tiploc_insert() {
        let p = parse_tiploc_insert::<VerboseError<_>>();
        let hdr =
            b"TIBLTNODR24853600DBOLTON-UPON-DEARNE        24011   0BTDBOLTON ON DEARNE        ";
        assert_eq!(80, hdr.len());
        let (rest, insert) = p(hdr).expect("parse_header");
        assert_eq!(rest, b"");
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
    #[test]
    fn should_parse_tiploc_amend() {
        let p = complete(parse_tiploc_amend::<VerboseError<_>>());
        let hdr =
            b"TAMBRK94200590970AMILLBROOK SIG E942        86536   0                           ";
        assert_eq!(80, hdr.len());
        let (rest, insert) = p(hdr).expect("parse_header");
        assert_eq!(rest, b"");
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
