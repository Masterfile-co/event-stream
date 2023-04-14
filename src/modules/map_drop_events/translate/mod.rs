use substreams_ethereum::{block_view::LogView, Event};

use crate::pb::masterfile::{
    common::v1::TransactionMetadata,
    drop::v1::{drop_event, DropEvent},
};

pub mod approval;
pub mod approval_for_all;
pub mod drop_created;
pub mod drop_sale_window_set;
pub mod edition_set;
pub mod listing_set;
pub mod metatransaction_executed;
pub mod mystery_box_set;
pub mod primary_sale;
pub mod randomness_received;
pub mod randomness_requested;
pub mod token_edition_set;
pub mod transfer;

pub fn translate<T: Event>(
    metadata: &Option<TransactionMetadata>,
    log: &LogView,
    address: &String,
    map: fn(T) -> drop_event::Event,
) -> Option<DropEvent> {
    match T::match_and_decode(log) {
        Some(event) => Some(DropEvent {
            r#event: Some(map(event)),
            metadata: metadata.clone(),
            address: address.clone(),
            ordinal: log.block_index() as u64,
        }),
        None => None,
    }
}
