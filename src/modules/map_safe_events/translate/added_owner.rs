use crate::{
    abi::GnosisSafeL2::events,
    pb::masterfile::safe::v1::safe_event::{AddedOwner, Event},
    utils::pretty_hex,
};

pub fn translate(event: events::AddedOwner) -> Event {
    Event::AddedOwner(AddedOwner {
        owner: pretty_hex(&event.owner),
    })
}
