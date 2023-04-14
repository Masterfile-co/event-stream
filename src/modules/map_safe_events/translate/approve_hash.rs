use crate::{
    abi::GnosisSafeL2::events,
    pb::masterfile::safe::v1::safe_event::{ApproveHash, Event},
    utils::pretty_hex,
};

pub fn translate(event: events::ApproveHash) -> Event {
    Event::ApproveHash(ApproveHash {
        owner: pretty_hex(&event.owner),
        hash: pretty_hex(&event.approved_hash),
    })
}
