pub mod added_owner;
pub mod approve_hash;
pub mod changed_fallback_handler;
pub mod changed_guard;
pub mod changed_threshold;
pub mod disabled_module;
pub mod enabled_module;
pub mod multisig_transaction;
pub mod removed_owner;
pub mod safe_received;
pub mod safe_setup;

use crate::pb::masterfile::{
    common::v1::TransactionMetadata,
    safe::v1::{safe_event, SafeEvent},
};
use substreams_ethereum::{block_view::LogView, Event};
use tiny_keccak::{Hasher, Keccak};

pub fn translate<T: Event>(
    metadata: &Option<TransactionMetadata>,
    log: &LogView,
    address: &String,
    map: fn(T) -> safe_event::Event,
) -> Option<SafeEvent> {
    match T::match_and_decode(log) {
        Some(event) => Some(SafeEvent {
            r#event: Some(map(event)),
            metadata: metadata.clone(),
            address: address.clone(),
            ordinal: log.block_index() as u64,
        }),
        None => None,
    }
}

pub fn keccak256<S>(bytes: S) -> [u8; 32]
where
    S: AsRef<[u8]>,
{
    let mut output = [0u8; 32];
    let mut hasher = Keccak::v256();
    hasher.update(bytes.as_ref());
    hasher.finalize(&mut output);
    output
}
