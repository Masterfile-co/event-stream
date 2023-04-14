mod translate;

use substreams::{errors::Error, prelude::*};
use substreams_ethereum::pb::eth::v2::Block;
use translate::{
    cancel_control_transfer, control_transfer, create_split, distribute_erc20, distribute_eth,
    initiate_control_transfer, translate, update_split,
};

use crate::{
    pb::masterfile::{deployment::v1::Deployment, splits::v1::SplitEvents},
    utils::extract_metadata,
};

#[substreams::handlers::map]
fn map_split_events(store: StoreGetProto<Deployment>, block: Block) -> Result<SplitEvents, Error> {
    let mut events = vec![];

    for log in block.logs() {
        let metadata = extract_metadata(&log, &block);

        if let Some(event) = translate(&store, &metadata, &log, cancel_control_transfer::translate)
        {
            events.push(event);
            continue;
        }

        if let Some(event) = translate(&store, &metadata, &log, control_transfer::translate) {
            events.push(event);
            continue;
        }

        if let Some(event) = translate(&store, &metadata, &log, create_split::translate) {
            events.push(event);
            continue;
        }

        if let Some(event) = translate(&store, &metadata, &log, distribute_erc20::translate) {
            events.push(event);
            continue;
        }

        if let Some(event) = translate(&store, &metadata, &log, distribute_eth::translate) {
            events.push(event);
            continue;
        }

        if let Some(event) = translate(
            &store,
            &metadata,
            &log,
            initiate_control_transfer::translate,
        ) {
            events.push(event);
            continue;
        }

        if let Some(event) = translate(&store, &metadata, &log, update_split::translate) {
            events.push(event);
            continue;
        }
    }

    Ok(SplitEvents { events })
}
