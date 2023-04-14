use crate::{
    abi::MysteryBoxDrop::events,
    pb::masterfile::drop::v1::drop_event::{Event, TokenEditionSet},
};

pub fn translate(event: events::TokenEditionSet) -> Event {
    Event::TokenEditionSet(TokenEditionSet {
        from_token_id: event.from_token_id.to_string(),
        to_token_id: event.to_token_id.to_string(),
        edition_number: event.edition_number.to_u64(),
    })
}
