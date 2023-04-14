use crate::{
    abi::GnosisSafeL2::events,
    pb::masterfile::safe::v1::safe_event::{DisabledModule, Event},
    utils::pretty_hex,
};

pub fn translate(event: events::DisabledModule) -> Event {
    Event::DisabledModule(DisabledModule {
        module: pretty_hex(&event.module),
    })
}
