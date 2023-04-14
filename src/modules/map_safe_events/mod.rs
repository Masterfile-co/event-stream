mod translate;

use substreams::{log, prelude::*};
use substreams_ethereum::{pb::eth::v2::Block, Event};
use translate::{
    added_owner, approve_hash, changed_fallback_handler, changed_guard, changed_threshold,
    disabled_module, enabled_module, multisig_transaction, removed_owner, safe_received,
    safe_setup, translate,
};

use crate::{
    abi::GnosisSafeL2::events,
    keyer,
    pb::masterfile::{
        deployment::v1::Deployment,
        safe::v1::{
            safe_event::{self, safe_multi_sig_transaction, ExecutionFailure, ExecutionSuccess},
            SafeEvents,
        },
    },
    utils::{extract_metadata, pretty_hex},
};

#[substreams::handlers::map]
pub fn map_safe_events(
    store: StoreGetProto<Deployment>,
    block: Block,
) -> Result<SafeEvents, substreams::errors::Error> {
    let mut events = vec![];

    for log in block.logs() {
        let address = pretty_hex(&log.address());

        if let Some(_) = store.get_last(&keyer::deployment_key(
            &address,
            keyer::DeploymentType::Channel,
        )) {
            log::debug!("Found deployment for {}", address);
            let metadata = extract_metadata(&log, &block);

            if let Some(event) = translate(&metadata, &log, &address, added_owner::translate) {
                events.push(event);
                continue;
            }

            if let Some(event) = translate(&metadata, &log, &address, approve_hash::translate) {
                events.push(event);
                continue;
            }

            if let Some(event) = translate(
                &metadata,
                &log,
                &address,
                changed_fallback_handler::translate,
            ) {
                events.push(event);
                continue;
            }

            if let Some(event) = translate(&metadata, &log, &address, changed_guard::translate) {
                events.push(event);
                continue;
            }

            if let Some(event) = translate(&metadata, &log, &address, changed_threshold::translate)
            {
                events.push(event);
                continue;
            }

            if let Some(event) = translate(&metadata, &log, &address, disabled_module::translate) {
                events.push(event);
                continue;
            }

            if let Some(event) = translate(&metadata, &log, &address, enabled_module::translate) {
                events.push(event);
                continue;
            }

            if let Some(event) = translate(&metadata, &log, &address, removed_owner::translate) {
                events.push(event);
                continue;
            }

            if let Some(event) = translate(&metadata, &log, &address, safe_received::translate) {
                events.push(event);
                continue;
            }

            if let Some(event) = translate(&metadata, &log, &address, safe_setup::translate) {
                events.push(event);
                continue;
            }

            // Multisig Transactions
            if let Some(event) = multisig_transaction::translate(&metadata, &log) {
                events.push(event);
                continue;
            }

            if let Some(execution_success) = events::ExecutionSuccess::match_and_decode(log) {
                events.iter_mut().for_each(|event| {
                    if let Some(safe_event::Event::SafeMultisigTransaction(tx_event)) =
                        &mut event.r#event
                    {
                        if tx_event.safe_tx_hash == pretty_hex(&execution_success.tx_hash) {
                            tx_event.r#result =
                                Some(safe_multi_sig_transaction::Result::ExecutionSuccess(
                                    ExecutionSuccess {
                                        safe_tx_hash: pretty_hex(&execution_success.tx_hash),
                                        payment: execution_success.payment.to_string(),
                                    },
                                ));
                        }
                    }
                });

                continue;
            }

            if let Some(execution_failure) = events::ExecutionFailure::match_and_decode(log) {
                events.iter_mut().for_each(|event| {
                    if let Some(safe_event::Event::SafeMultisigTransaction(tx_event)) =
                        &mut event.r#event
                    {
                        if tx_event.safe_tx_hash == pretty_hex(&execution_failure.tx_hash) {
                            tx_event.r#result =
                                Some(safe_multi_sig_transaction::Result::ExecutionFailure(
                                    ExecutionFailure {
                                        safe_tx_hash: pretty_hex(&execution_failure.tx_hash),
                                        payment: execution_failure.payment.to_string(),
                                    },
                                ));
                        }
                    }
                });

                continue;
            }
        }
    }

    Ok(SafeEvents { events })
}
