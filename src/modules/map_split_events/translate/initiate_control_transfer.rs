use crate::{
    abi::SplitMain::events,
    pb::masterfile::splits::v1::split_event::{Event, InitiateControlTransfer},
    utils::pretty_hex,
};

pub fn translate(event: events::InitiateControlTransfer) -> Event {
    Event::InitiateControlTransfer(InitiateControlTransfer {
        split: pretty_hex(&event.split),
        new_potential_controller: pretty_hex(&event.new_potential_controller),
    })
}
