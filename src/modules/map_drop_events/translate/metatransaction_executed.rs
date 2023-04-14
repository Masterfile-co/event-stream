use crate::{
    abi::MysteryBoxDrop::events,
    pb::masterfile::drop::v1::drop_event::{Event, MetaTransactionExecuted},
    utils::pretty_hex,
};

pub fn translate(event: events::MetaTransactionExecuted) -> Event {
    Event::MetaTransactionExecuted(MetaTransactionExecuted {
        user_address: pretty_hex(&event.user_address),
        relayer_address: pretty_hex(&event.relayer_address),
        function_signature: pretty_hex(&event.function_signature),
    })
}
