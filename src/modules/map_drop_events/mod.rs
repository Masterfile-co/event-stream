mod translate;

use crate::{
    keyer,
    pb::masterfile::{deployment::v1::Deployment, drop::v1::DropEvents},
    utils::{extract_metadata, pretty_hex},
};
use substreams::{errors::Error, log, prelude::*};
use substreams_ethereum::pb::eth::v2::Block;
use translate::{
    approval, approval_for_all, drop_created, drop_sale_window_set, edition_set, listing_set,
    metatransaction_executed, mystery_box_set, primary_sale, randomness_received,
    randomness_requested, token_edition_set, transfer, translate,
};

#[substreams::handlers::map]
pub fn map_drop_events(
    store: StoreGetProto<Deployment>,
    block: Block,
) -> Result<DropEvents, Error> {
    let mut events = vec![];

    for log in block.logs() {
        let address = pretty_hex(&log.address());

        if let Some(_) = store.get_last(&keyer::deployment_key(
            &address,
            keyer::DeploymentType::Channel,
        )) {
            log::debug!("Found deployment for {}", address);
            let metadata = extract_metadata(&log, &block);

            if let Some(event) = translate(&metadata, &log, &address, approval_for_all::translate) {
                events.push(event);
                continue;
            }

            if let Some(event) = translate(&metadata, &log, &address, approval::translate) {
                events.push(event);
                continue;
            }

            if let Some(event) = translate(&metadata, &log, &address, drop_created::translate) {
                events.push(event);
                continue;
            }

            if let Some(event) =
                translate(&metadata, &log, &address, drop_sale_window_set::translate)
            {
                events.push(event);
                continue;
            }

            if let Some(event) = translate(&metadata, &log, &address, edition_set::translate) {
                events.push(event);
                continue;
            }

            if let Some(event) = translate(&metadata, &log, &address, listing_set::translate) {
                events.push(event);
                continue;
            }

            if let Some(event) = translate(
                &metadata,
                &log,
                &address,
                metatransaction_executed::translate,
            ) {
                events.push(event);
                continue;
            }

            if let Some(event) = translate(&metadata, &log, &address, mystery_box_set::translate) {
                events.push(event);
                continue;
            }

            if let Some(event) = translate(&metadata, &log, &address, primary_sale::translate) {
                events.push(event);
                continue;
            }

            if let Some(event) =
                translate(&metadata, &log, &address, randomness_received::translate)
            {
                events.push(event);
                continue;
            }

            if let Some(event) =
                translate(&metadata, &log, &address, randomness_requested::translate)
            {
                events.push(event);
                continue;
            }

            if let Some(event) = translate(&metadata, &log, &address, token_edition_set::translate)
            {
                events.push(event);
                continue;
            }

            if let Some(event) = translate(&metadata, &log, &address, transfer::translate) {
                events.push(event);
                continue;
            }
        }
    }

    Ok(DropEvents { events })
}
