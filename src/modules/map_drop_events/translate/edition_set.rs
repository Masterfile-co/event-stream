use crate::{
    abi::MysteryBoxDrop::events,
    pb::masterfile::drop::v1::drop_event::{EditionSet, Event},
    utils::pretty_hex,
};

pub fn translate(event: events::EditionSet) -> Event {
    Event::EditionSet(EditionSet {
        edition_number: event.edition_number.to_u64(),
        recipient: pretty_hex(&event.recipient),
        arweave_cid: pretty_hex(&event.arweave_cid), // TODO! Check formatting
    })
}
