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
mod schedule;
mod schedule_extra;
mod tiploc;
mod tiploc_amend;
mod tiploc_insert;
mod trailer;

pub use association::Association;
pub use basic_schedule::{BasicSchedule, ScheduleCancellation};
pub use change_en_route::ChangeEnRoute;
pub use errors::CIFParseError;
pub use header::Header;
pub use location_intermediate::LocationIntermediate;
pub use location_origin::LocationOrigin;
pub use location_terminating::LocationTerminating;
pub use schedule::Schedule;
pub use schedule_extra::ScheduleExtra;
pub use tiploc::Tiploc;
pub use tiploc_amend::TiplocAmend;
pub use tiploc_insert::TiplocInsert;
pub use trailer::Trailer;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Record<'a> {
    Header(Header<'a>),
    TiplocInsert(TiplocInsert<'a>),
    TiplocAmend(TiplocAmend<'a>),
    Association(Association<'a>),
    Schedule(Schedule<'a>),
    ScheduleCancellation(ScheduleCancellation<'a>),
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
pub enum STP {
    New,
    Overlay,
    Permanent,
}

pub fn parse<'a>(i: &'a [u8]) -> IResult<&'a [u8], Record, CIFParseError> {
    let p = alt((
        map(header::parse_header(), Record::Header),
        map(tiploc_insert::parse_tiploc_insert(), Record::TiplocInsert),
        map(tiploc_amend::parse_tiploc_amend(), Record::TiplocAmend),
        map(association::parse_association(), Record::Association),
        map(schedule::parse_schedule(), Record::Schedule),
        map(
            basic_schedule::parse_schedule_cancellation(),
            Record::ScheduleCancellation,
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
            std::str::from_utf8(other).map_err(CIFParseError::into_unrecoverable)?,
        ))
    }
}
