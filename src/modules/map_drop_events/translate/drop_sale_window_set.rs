use crate::{
    abi::MysteryBoxDrop::events,
    pb::masterfile::drop::v1::drop_event::{DropSaleWindowSet, Event},
};

pub fn translate(event: events::DropSaleWindowSet) -> Event {
    Event::DropSaleWindowSet(DropSaleWindowSet {
        drop_start_date: event.drop_start_date.to_string(),
        drop_end_date: event.drop_end_date.to_string(),
    })
}
