use nom::{
    branch::alt, bytes::streaming::take, character::streaming::*, combinator::map,
    sequence::terminated, IResult,
};

mod association;
mod basic_schedule;
mod change_en_route;
mod errors;
mod header;
mod helpers;
mod location_intermediate;
mod location_origin;
mod location_terminating;
mod reader;
mod schedule_extra;
mod tiploc;
mod tiploc_amend;
mod tiploc_insert;
mod trailer;

pub use association::Association;
pub use basic_schedule::BasicSchedule;
pub use change_en_route::ChangeEnRoute;
pub use errors::CIFParseError;
pub use header::Header;
pub use location_intermediate::LocationIntermediate;
pub use location_origin::LocationOrigin;
pub use location_terminating::LocationTerminating;
pub use reader::{Reader, ReaderError, ReaderResult};
pub use schedule_extra::ScheduleExtra;
pub use tiploc::Tiploc;
pub use tiploc_amend::TiplocAmend;
pub use tiploc_insert::TiplocInsert;
pub use trailer::Trailer;

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Record<'a> {
    Header(Header),
    TiplocInsert(TiplocInsert),
    TiplocAmend(TiplocAmend),
    Association(Association),
    Schedule(BasicSchedule<'a>),
    ScheduleExtra(ScheduleExtra<'a>),
    LocationOrigin(LocationOrigin<'a>),
    LocationIntermediate(LocationIntermediate<'a>),
    LocationTerminating(LocationTerminating<'a>),
    ChangeEnRoute(ChangeEnRoute<'a>),
    Trailer(Trailer),
    Unrecognised(&'a str),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TransactionType {
    New,
    Delete,
    Revise,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Stp {
    Cancellation,
    New,
    Overlay,
    Permanent,
}

pub fn parse<'a>(i: &'a [u8]) -> IResult<&'a [u8], Record, CIFParseError> {
    let p = alt((
        map(basic_schedule::parse_basic_schedule(), Record::Schedule),
        map(
            schedule_extra::parse_schedule_extra(),
            Record::ScheduleExtra,
        ),
        map(
            location_origin::parse_location_origin(),
            Record::LocationOrigin,
        ),
        map(
            location_intermediate::parse_location_intermediate(),
            Record::LocationIntermediate,
        ),
        map(
            change_en_route::parse_change_en_route(),
            Record::ChangeEnRoute,
        ),
        map(
            location_terminating::parse_location_terminating(),
            Record::LocationTerminating,
        ),
        map(trailer::parse_trailer(), Record::Trailer),
        map(parse_unrecognised(), Record::Unrecognised),
    ));
    terminated(p, char('\n'))(i)
}

fn parse_unrecognised<'a>() -> impl Fn(&'a [u8]) -> IResult<&'a [u8], &'a str, CIFParseError> {
    |i: &'a [u8]| -> IResult<&'a [u8], &'a str, CIFParseError> {
        let (i, other) = take(80usize)(i)?;

        Ok((
            i,
            std::str::from_utf8(other).map_err(CIFParseError::from_unrecoverable)?,
        ))
    }
}
