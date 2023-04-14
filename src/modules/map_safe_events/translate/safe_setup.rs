use crate::{
    abi::GnosisSafeL2::events,
    pb::masterfile::safe::v1::safe_event::{Event, SafeOwner, SafeSetup},
    utils::pretty_hex,
};

pub fn translate(event: events::SafeSetup) -> Event {
    Event::SafeSetup(SafeSetup {
        initiator: pretty_hex(&event.initiator),
        threshold: event.threshold.to_u64(),
        initializer: pretty_hex(&event.initializer),
        fallback_handler: pretty_hex(&event.fallback_handler),
        owners: event
            .owners
            .iter()
            .map(|o| SafeOwner {
                address: pretty_hex(o),
            })
            .collect(),
    })
}
