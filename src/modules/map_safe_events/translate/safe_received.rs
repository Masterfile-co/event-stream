use crate::{
    abi::GnosisSafeL2::events,
    pb::masterfile::safe::v1::safe_event::{Event, SafeReceived},
    utils::pretty_hex,
};

pub fn translate(event: events::SafeReceived) -> Event {
    Event::SafeReceived(SafeReceived {
        sender: pretty_hex(&event.sender),
        value: event.value.to_string(),
    })
}
