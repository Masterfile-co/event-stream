use crate::{
    abi::GnosisSafeL2::events,
    pb::masterfile::safe::v1::safe_event::{ChangedThreshold, Event},
};

pub fn translate(event: events::ChangedThreshold) -> Event {
    Event::ChangedThreshold(ChangedThreshold {
        threshold: event.threshold.to_u64(),
    })
}
