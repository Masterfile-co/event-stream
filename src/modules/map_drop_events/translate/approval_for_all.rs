use crate::{
    abi::MysteryBoxDrop::events,
    pb::masterfile::drop::v1::drop_event::{ApprovalForAll, Event},
    utils::pretty_hex,
};

pub fn translate(event: events::ApprovalForAll) -> Event {
    Event::ApprovalForAll(ApprovalForAll {
        owner: pretty_hex(&event.owner),
        operator: pretty_hex(&event.operator),
        approved: event.approved,
    })
}
