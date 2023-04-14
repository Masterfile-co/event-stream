use crate::{
    abi::GnosisSafeL2::events,
    pb::masterfile::safe::v1::safe_event::{Event, RemovedOwner},
    utils::pretty_hex,
};

pub fn translate(event: events::RemovedOwner) -> Event {
    Event::RemovedOwner(RemovedOwner {
        owner: pretty_hex(&event.owner),
    })
}
