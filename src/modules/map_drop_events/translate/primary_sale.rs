use crate::{
    abi::MysteryBoxDrop::events,
    pb::masterfile::drop::v1::drop_event::{Event, PrimarySale},
    utils::pretty_hex,
};

pub fn translate(event: events::PrimarySale) -> Event {
    Event::PrimarySale(PrimarySale {
        buyer: pretty_hex(&event.buyer),
        recipient: pretty_hex(&event.recipient),
        edition_number: event.edition_number.to_u64(),
        quantity: event.quantity.to_string(),
        total_price: event.total_price.to_string(),
    })
}
