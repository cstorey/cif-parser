use bytes::Bytes;

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
pub enum Record {
    Header(Header),
    TiplocInsert(TiplocInsert),
    TiplocAmend(TiplocAmend),
    Association(Association),
    Schedule(BasicSchedule),
    ScheduleExtra(ScheduleExtra),
    LocationOrigin(LocationOrigin),
    LocationIntermediate(LocationIntermediate),
    LocationTerminating(LocationTerminating),
    ChangeEnRoute(ChangeEnRoute),
    Trailer(Trailer),
    Unrecognised(Bytes),
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
