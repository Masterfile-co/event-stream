pub mod cancel_control_transfer;
pub mod control_transfer;
pub mod create_split;
pub mod distribute_erc20;
pub mod distribute_eth;
pub mod initiate_control_transfer;
pub mod update_split;

use crate::{
    keyer,
    pb::masterfile::{
        common::v1::TransactionMetadata,
        deployment::v1::Deployment,
        splits::v1::{split_event, SplitEvent},
    },
    utils::pretty_hex,
};
use substreams::{prelude::*, store::StoreGetProto};
use substreams_ethereum::{block_view::LogView, Event};

pub fn translate<T: Event>(
    store: &StoreGetProto<Deployment>,
    metadata: &Option<TransactionMetadata>,
    log: &LogView,
    map: fn(T) -> split_event::Event,
) -> Option<SplitEvent> {
    let event = match T::match_and_decode(log) {
        Some(event) => Some(SplitEvent {
            r#event: Some(map(event)),
            metadata: metadata.clone(),
            address: pretty_hex(&log.address()),
            ordinal: log.block_index() as u64,
        }),
        None => None,
    };

    if let Some(_) = store.get_last(keyer::deployment_key(
        &extract_split_address(&event).unwrap_or_default(),
        keyer::DeploymentType::Splits,
    )) {
        event
    } else {
        None
    }
}

fn extract_split_address(event: &Option<SplitEvent>) -> Option<String> {
    if let Some(event) = event {
        if let Some(event) = event.r#event.as_ref() {
            return match event {
                split_event::Event::CancelControlTransfer(event) => Some(event.split.clone()),
                split_event::Event::ControlTransfer(event) => Some(event.split.clone()),
                split_event::Event::CreateSplit(event) => Some(event.split.clone()),
                split_event::Event::DistributeErc20(event) => Some(event.split.clone()),
                split_event::Event::DistributeEth(event) => Some(event.split.clone()),
                split_event::Event::InitiateControlTransfer(event) => Some(event.split.clone()),
                split_event::Event::UpdateSplit(event) => Some(event.split.clone()),
            };
        }
    }

    None
}
