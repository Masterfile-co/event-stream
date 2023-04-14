use crate::{
    abi::MysteryBoxDrop::events,
    pb::masterfile::drop::v1::drop_event::{Event, ListingSet},
};

pub fn translate(event: events::ListingSet) -> Event {
    Event::ListingSet(ListingSet {
        edition_number: event.edition_number.to_u64(),
        quantity: event.quantity.to_string(),
        price: event.price.to_string(),
    })
}
