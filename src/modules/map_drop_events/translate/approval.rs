use crate::{
    abi::MysteryBoxDrop::events,
    pb::masterfile::drop::v1::drop_event::{Approval, Event},
    utils::pretty_hex,
};

pub fn translate(event: events::Approval) -> Event {
    Event::Approval(Approval {
        owner: pretty_hex(&event.owner),
        approved: pretty_hex(&event.approved),
        token_id: event.token_id.to_string(),
    })
}
