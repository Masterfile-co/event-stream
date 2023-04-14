use crate::{
    abi::GnosisSafeL2::events,
    pb::masterfile::safe::v1::safe_event::{ChangedFallbackHandler, Event},
    utils::pretty_hex,
};

pub fn translate(event: events::ChangedFallbackHandler) -> Event {
    Event::ChangedFallbackHandler(ChangedFallbackHandler {
        handler: pretty_hex(&event.handler),
    })
}
