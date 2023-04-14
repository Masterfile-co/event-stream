use crate::{
    abi::MysteryBoxDrop::events,
    pb::masterfile::drop::v1::drop_event::{Event, MysteryBoxSet},
};

pub fn translate(event: events::MysteryBoxSet) -> Event {
    Event::MysteryBoxSet(MysteryBoxSet {
        probabilities: event.probabilities.iter().map(|x| x.to_u64()).collect(),
    })
}
