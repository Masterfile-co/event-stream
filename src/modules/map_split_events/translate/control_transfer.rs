use crate::{
    abi::SplitMain::events,
    pb::masterfile::splits::v1::split_event::{ControlTransfer, Event},
    utils::pretty_hex,
};

pub fn translate(event: events::ControlTransfer) -> Event {
    Event::ControlTransfer(ControlTransfer {
        split: pretty_hex(&event.split),
        previous_controller: pretty_hex(&event.previous_controller),
        new_controller: pretty_hex(&event.new_controller),
    })
}
