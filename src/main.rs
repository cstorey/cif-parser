use env_logger;
use log::*;
use std::borrow::Cow;
use std::fs::File;
use std::marker::PhantomData;
use std::path::PathBuf;

use failure::*;
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
    Association(Association<'a>),
    BasicSchedule(BasicSchedule<'a>),
    ScheduleExtra(ScheduleExtra<'a>),
    LocationOrigin(LocationOrigin<'a>),
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

#[derive(Debug, Clone, Eq, PartialEq)]
enum TransactionType {
    New,
    Delete,
    Revise,
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum AssociationSTP {
    Cancellation,
    New,
    Overlay,
    Permanent,
}
#[derive(Debug, Clone, Eq, PartialEq)]
struct Association<'a> {
    _phantom: PhantomData<&'a [u8]>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct BasicSchedule<'a> {
    transaction_type: TransactionType,
    uid: Cow<'a, str>,
    start_date: Cow<'a, str>,
    end_date: Cow<'a, str>,
    days: Cow<'a, str>,
    bank_holiday: Cow<'a, str>,
    status: Cow<'a, str>,
    category: Cow<'a, str>,
    identity: Cow<'a, str>,
    headcode: Cow<'a, str>,

    service_code: Cow<'a, str>,
    speed: Cow<'a, str>,
    seating_class: Cow<'a, str>,
    sleepers: Cow<'a, str>,
    reservations: Cow<'a, str>,
    catering: Cow<'a, str>,
    branding: Cow<'a, str>,
    stp: AssociationSTP,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct ScheduleExtra<'a> {
    uic_code: Cow<'a, str>,
    atoc_code: Cow<'a, str>,
    applicable_timetable_code: Cow<'a, str>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct LocationOrigin<'a> {
    tiploc: Cow<'a, str>,
    scheduled_departure_time: Cow<'a, str>,
    public_departure: Cow<'a, str>,
    platform: Cow<'a, str>,
    line: Cow<'a, str>,
    eng_allowance: Cow<'a, str>,
    path_allowance: Cow<'a, str>,
    activity: Cow<'a, str>,
    perf_allowance: Cow<'a, str>,
}

fn parse<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], Record, E> {
    let p = alt((
        map(parse_header(), Record::Header),
        map(parse_tiploc_insert(), Record::TiplocInsert),
        map(parse_tiploc_amend(), Record::TiplocAmend),
        map(parse_association(), Record::Association),
        map(parse_basic_schedule(), Record::BasicSchedule),
        map(parse_schedule_extra(), Record::ScheduleExtra),
        map(parse_location_origin(), Record::LocationOrigin),
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

fn parse_association<'a, E: ParseError<&'a [u8]>>(
) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], Association, E> {
    |i: &'a [u8]| -> IResult<&'a [u8], Association, E> {
        let (i, _) = tag("AA")(i)?;
        let (i, _spare) = take(78usize)(i)?;

        Ok((
            i,
            Association {
                _phantom: PhantomData,
            },
        ))
    }
}

fn parse_basic_schedule<'a, E: ParseError<&'a [u8]>>(
) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], BasicSchedule, E> {
    |i: &'a [u8]| -> IResult<&'a [u8], BasicSchedule, E> {
        let (i, _) = tag("BS")(i)?;
        let (i, ttype) = alt((
            map(char('N'), |_| TransactionType::New),
            map(char('D'), |_| TransactionType::Delete),
            map(char('R'), |_| TransactionType::Revise),
        ))(i)?;
        let (i, uid) = take(6usize)(i)?;
        let (i, start_date) = take(6usize)(i)?;
        let (i, end_date) = take(6usize)(i)?;
        let (i, days) = take(7usize)(i)?; // Bit string?
        let (i, bank_holiday) = take(1usize)(i)?;
        let (i, status) = take(1usize)(i)?;
        let (i, category) = take(2usize)(i)?;
        let (i, identity) = take(4usize)(i)?;
        let (i, headcode) = take(4usize)(i)?;
        let (i, _) = take(1usize)(i)?;
        let (i, service_code) = take(8usize)(i)?;
        let (i, _portion_id) = take(1usize)(i)?;
        let (i, _power_type) = take(3usize)(i)?;
        let (i, _timing_load) = take(4usize)(i)?;
        let (i, speed) = take(3usize)(i)?;
        let (i, _operating_characteristics) = take(6usize)(i)?;
        let (i, seating_class) = take(1usize)(i)?;
        let (i, sleepers) = take(1usize)(i)?;
        let (i, reservations) = take(1usize)(i)?;
        let (i, _connection) = take(1usize)(i)?;
        let (i, catering) = take(4usize)(i)?;
        let (i, branding) = take(4usize)(i)?;
        let (i, _spare) = take_while_m_n(1, 1, is_space)(i)?;
        let (i, stp) = alt((
            map(char('C'), |_| AssociationSTP::Cancellation),
            map(char('N'), |_| AssociationSTP::New),
            map(char('O'), |_| AssociationSTP::Overlay),
            map(char('P'), |_| AssociationSTP::Permanent),
        ))(i)?;

        Ok((
            i,
            BasicSchedule {
                transaction_type: ttype,
                uid: String::from_utf8_lossy(uid),
                start_date: String::from_utf8_lossy(start_date),
                end_date: String::from_utf8_lossy(end_date),
                days: String::from_utf8_lossy(days),
                bank_holiday: String::from_utf8_lossy(bank_holiday),
                status: String::from_utf8_lossy(status),
                category: String::from_utf8_lossy(category),
                identity: String::from_utf8_lossy(identity),
                headcode: String::from_utf8_lossy(headcode),
                service_code: String::from_utf8_lossy(service_code),
                speed: String::from_utf8_lossy(speed),
                seating_class: String::from_utf8_lossy(seating_class),
                sleepers: String::from_utf8_lossy(sleepers),
                reservations: String::from_utf8_lossy(reservations),
                catering: String::from_utf8_lossy(catering),
                branding: String::from_utf8_lossy(branding),
                stp: stp,
            },
        ))
    }
}

fn parse_schedule_extra<'a, E: ParseError<&'a [u8]>>(
) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], ScheduleExtra, E> {
    |i: &'a [u8]| -> IResult<&'a [u8], ScheduleExtra, E> {
        let (i, _) = tag("BX")(i)?;
        let (i, _traction_class) = take(4usize)(i)?;
        let (i, uic_code) = take(5usize)(i)?;
        let (i, atoc_code) = take(2usize)(i)?;
        let (i, applicable_timetable_code) = take(1usize)(i)?;
        let (i, _reserved) = take(8usize)(i)?;
        let (i, _reserved) = take(1usize)(i)?;
        let (i, _spare) = take_while_m_n(57, 57, is_space)(i)?;

        Ok((
            i,
            ScheduleExtra {
                uic_code: String::from_utf8_lossy(uic_code),
                atoc_code: String::from_utf8_lossy(atoc_code),
                applicable_timetable_code: String::from_utf8_lossy(applicable_timetable_code),
            },
        ))
    }
}
fn parse_location_origin<'a, E: ParseError<&'a [u8]>>(
) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], LocationOrigin, E> {
    |i: &'a [u8]| -> IResult<&'a [u8], LocationOrigin, E> {
        let (i, _) = tag("LO")(i)?; // 1-2
        let (i, tiploc) = take(8usize)(i)?; // 3-10
        let (i, scheduled_departure_time) = take(5usize)(i)?; // 11-15
        let (i, public_departure) = take(4usize)(i)?; // 16-19
        let (i, platform) = take(3usize)(i)?; // 20-22
        let (i, line) = take(3usize)(i)?; // 23-25
        let (i, eng_allowance) = take(2usize)(i)?;
        let (i, path_allowance) = take(2usize)(i)?;
        let (i, activity) = take(12usize)(i)?;
        let (i, perf_allowance) = take(2usize)(i)?;
        let (i, _spare) = take_while_m_n(37, 37, is_space)(i)?;

        Ok((
            i,
            LocationOrigin {
                tiploc: String::from_utf8_lossy(tiploc),
                scheduled_departure_time: String::from_utf8_lossy(scheduled_departure_time),
                public_departure: String::from_utf8_lossy(public_departure),
                platform: String::from_utf8_lossy(platform),
                line: String::from_utf8_lossy(line),
                eng_allowance: String::from_utf8_lossy(eng_allowance),
                path_allowance: String::from_utf8_lossy(path_allowance),
                activity: String::from_utf8_lossy(activity),
                perf_allowance: String::from_utf8_lossy(perf_allowance),
            },
        ))
    }
}

fn main() -> Fallible<()> {
    env_logger::init();
    let opts = Opts::from_args();

    for f in opts.files {
        let fp = File::open(f)?;
        let mmap = unsafe { Mmap::map(&fp)? };
        let mut i: &[u8] = &mmap;
        loop {
            match parse::<VerboseError<_>>(&i) {
                Ok((rest, val)) => {
                    i = rest;
                    info!("Ok: {:#?}", val)
                }

                Err(Err::Incomplete(need)) => {
                    error!("Needed: {:?}", need);
                    return Result::Err(failure::err_msg("Not enough data"));
                }
                Err(Err::Error(err)) => {
                    error!("Error:");
                    show_error(err);
                    return Result::Err(failure::err_msg("Parser error"));
                }
                Err(Err::Failure(err)) => {
                    error!("Failure:");
                    show_error(err);
                    return Result::Err(failure::err_msg("Parser failure"));
                }
            }
        }
    }

    Ok(())
}

fn show_error(err: VerboseError<&[u8]>) {
    const SNIPPET_LEN: usize = 240;
    for (i, kind) in err.errors {
        error!(
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
        let p = parse_header::<VerboseError<_>>();
        let hdr =
            b"HDTPS.UDFROC1.PD1907050507191939DFROC2S       FA050719040720                    ";
        let (rest, _val) = p(hdr).expect("parse_header");
        assert_eq!(String::from_utf8_lossy(rest), "");
    }

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
    #[test]
    fn should_parse_association() {
        let p = complete(parse_association::<VerboseError<_>>());
        let hdr =
            b"AANY80987Y808801601041602121111100JJSPRST     TP                               P";
        assert_eq!(80, hdr.len());
        let (rest, insert) = p(hdr).expect("parse_header");
        assert_eq!(String::from_utf8_lossy(rest), "");
        assert_eq!(
            insert,
            Association {
                _phantom: PhantomData,
            }
        )
    }
    #[test]
    fn should_parse_basic_schedule() {
        let p = complete(parse_basic_schedule::<VerboseError<_>>());
        let i = b"BSRG828851510191510231100100 POO2N75    113575825 DMUE   090      S            O";
        assert_eq!(80, i.len());
        let (rest, val) = p(i).expect("parse");
        assert_eq!(String::from_utf8_lossy(rest), "");
        assert_eq!(
            val,
            BasicSchedule {
                transaction_type: TransactionType::Revise,
                uid: "G82885".into(),
                start_date: "151019".into(),
                end_date: "151023".into(),
                days: "1100100".into(),
                bank_holiday: " ".into(),
                status: "P".into(),
                category: "OO".into(),
                identity: "2N75".into(),
                headcode: "    ".into(),
                service_code: "13575825".into(),
                speed: "090".into(),
                seating_class: "S".into(),
                sleepers: " ".into(),
                reservations: " ".into(),
                catering: "    ".into(),
                branding: "    ".into(),
                stp: AssociationSTP::Overlay,
            }
        )
    }
    #[test]
    fn should_parse_schedule_extra() {
        let p = complete(parse_schedule_extra::<VerboseError<_>>());
        let i = b"BX         SEY                                                                  ";
        assert_eq!(80, i.len());
        let (rest, val) = p(i).expect("parse");
        assert_eq!(String::from_utf8_lossy(rest), "");
        assert_eq!(
            val,
            ScheduleExtra {
                uic_code: "     ".into(),
                atoc_code: "SE".into(),
                applicable_timetable_code: "Y".into(),
            }
        )
    }
    #[test]
    fn should_parse_location_origin() {
        let p = parse_location_origin::<VerboseError<_>>();
        let i = b"LOCHRX    0015 00156  FL     TB                                                 ";
        assert_eq!(80, i.len());
        let (rest, val) = p(i).expect("parse");
        let rest = String::from_utf8_lossy(rest);
        assert_eq!(
            (val, &*rest),
            (
                LocationOrigin {
                    tiploc: "CHRX    ".into(),
                    scheduled_departure_time: "0015 ".into(),
                    public_departure: "0015".into(),
                    platform: "6  ".into(),
                    line: "FL ".into(),
                    eng_allowance: "  ".into(),
                    path_allowance: "  ".into(),
                    activity: "TB          ".into(),
                    perf_allowance: "  ".into(),
                },
                "",
            )
        )
    }
}
