use crate::*;

mod header;
mod tiploc_amend;
mod tiploc_insert;

pub use header::Header;
pub use tiploc_amend::TiplocAmend;
pub use tiploc_insert::TiplocInsert;

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

pub fn parse<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], records::Record, E> {
    use records::*;
    let p = alt((
        map(header::parse_header(), Record::Header),
        map(tiploc_insert::parse_tiploc_insert(), Record::TiplocInsert),
        map(tiploc_amend::parse_tiploc_amend(), Record::TiplocAmend),
        map(parse_association(), Record::Association),
        map(parse_basic_schedule(), Record::BasicSchedule),
        map(parse_schedule_extra(), Record::ScheduleExtra),
        map(parse_location_origin(), Record::LocationOrigin),
        map(parse_location_intermediate(), Record::LocationIntermediate),
        map(parse_location_terminating(), Record::LocationTerminating),
        map(parse_change_en_route(), Record::ChangeEnRoute),
        map(parse_trailer(), Record::Trailer),
    ));
    terminated(p, char('\n'))(i)
}
