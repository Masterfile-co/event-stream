use crate::{
    abi::MysteryBoxDrop::events,
    pb::masterfile::drop::v1::drop_event::{Event, RandomnessRequested},
};

pub fn translate(event: events::RandomnessRequested) -> Event {
    Event::RandomnessRequested(RandomnessRequested {
        token_id: event.token_id.to_string(),
        request_id: event.request_id.to_string(),
    })
}
