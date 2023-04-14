use crate::{
    abi::GnosisSafeL2::events,
    pb::masterfile::safe::v1::safe_event::{EnabledModule, Event},
    utils::pretty_hex,
};

pub fn translate(event: events::EnabledModule) -> Event {
    Event::EnabledModule(EnabledModule {
        module: pretty_hex(&event.module),
    })
}
