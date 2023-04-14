use crate::{
    abi::MysteryBoxDrop::events,
    pb::masterfile::drop::v1::drop_event::{Event, RandomnessReceived},
};

pub fn translate(event: events::RandomnessReceived) -> Event {
    Event::RandomnessReceived(RandomnessReceived {
        token_id: event.token_id.to_string(),
        request_id: event.request_id.to_string(),
        randomness: event.randomness.to_string(),
    })
}
