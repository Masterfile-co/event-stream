use crate::{
    abi::SplitMain::events,
    pb::masterfile::splits::v1::split_event::{CancelControlTransfer, Event},
    utils::pretty_hex,
};

pub fn translate(event: events::CancelControlTransfer) -> Event {
    Event::CancelControlTransfer(CancelControlTransfer {
        split: pretty_hex(&event.split),
    })
}
