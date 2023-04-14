use crate::{
    abi::GnosisSafeL2::events,
    pb::masterfile::safe::v1::safe_event::{ChangedGuard, Event},
    utils::pretty_hex,
};

pub fn translate(event: events::ChangedGuard) -> Event {
    Event::ChangedGuard(ChangedGuard {
        guard: pretty_hex(&event.guard),
    })
}
