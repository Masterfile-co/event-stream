use crate::{
    abi::MysteryBoxDrop::events,
    pb::masterfile::drop::v1::drop_event::{DropCreated, Event},
    utils::pretty_hex,
};

pub fn translate(event: events::DropCreated) -> Event {
    Event::DropCreated(DropCreated {
        creator: pretty_hex(&event.creator),
        name: event.name,
        symbol: event.symbol,
        series: event.series,
        volume: event.volume.to_u64(),
    })
}
