use crate::{
    abi::MysteryBoxDrop::events,
    pb::masterfile::drop::v1::drop_event::{Event, Transfer},
    utils::pretty_hex,
};

pub fn translate(event: events::Transfer) -> Event {
    Event::Transfer(Transfer {
        from: pretty_hex(&event.from),
        to: pretty_hex(&event.to),
        token_id: event.token_id.to_string(),
    })
}
