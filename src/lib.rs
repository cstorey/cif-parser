use nom::{
    branch::alt, character::streaming::*, combinator::map, error::*, sequence::terminated, IResult,
};

mod association;
mod basic_schedule;
mod change_en_route;
mod header;
mod location_intermediate;
mod location_origin;
mod location_terminating;
mod schedule_extra;
mod tiploc_amend;
mod tiploc_insert;
mod trailer;

pub use association::Association;
pub use basic_schedule::BasicSchedule;
pub use change_en_route::ChangeEnRoute;
pub use header::Header;
pub use location_intermediate::LocationIntermediate;
pub use location_origin::LocationOrigin;
pub use location_terminating::LocationTerminating;
pub use schedule_extra::ScheduleExtra;
pub use tiploc_amend::TiplocAmend;
pub use tiploc_insert::TiplocInsert;
pub use trailer::Trailer;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Record<'a> {
    Header(Header<'a>),
    TiplocInsert(TiplocInsert<'a>),
    TiplocAmend(TiplocAmend<'a>),
    Association(Association<'a>),
    BasicSchedule(BasicSchedule<'a>),
    ScheduleExtra(ScheduleExtra<'a>),
    LocationOrigin(LocationOrigin<'a>),
    LocationIntermediate(LocationIntermediate<'a>),
    LocationTerminating(LocationTerminating<'a>),
    ChangeEnRoute(ChangeEnRoute<'a>),
    Trailer(Trailer),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TransactionType {
    New,
    Delete,
    Revise,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum STP {
    Cancellation,
    New,
    Overlay,
    Permanent,
}

pub fn parse<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], Record, E> {
    let p = alt((
        map(header::parse_header(), Record::Header),
        map(tiploc_insert::parse_tiploc_insert(), Record::TiplocInsert),
        map(tiploc_amend::parse_tiploc_amend(), Record::TiplocAmend),
        map(association::parse_association(), Record::Association),
        map(
            basic_schedule::parse_basic_schedule(),
            Record::BasicSchedule,
        ),
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
            location_terminating::parse_location_terminating(),
            Record::LocationTerminating,
        ),
        map(
            change_en_route::parse_change_en_route(),
            Record::ChangeEnRoute,
        ),
        map(trailer::parse_trailer(), Record::Trailer),
    ));
    terminated(p, char('\n'))(i)
}
