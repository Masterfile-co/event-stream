use crate::{
    abi::{
        ChannelFactory::events::ChannelDeployed, MysteryBoxDropFactory::events::DropDeployed,
        SplitsFactory::events::SplitDeployed,
    },
    keyer,
    pb::masterfile::factory::v1::{factory_event, Factory, FactoryEvent, FactoryEvents},
    utils::{extract_metadata, pretty_hex},
};
use substreams::prelude::*;
use substreams::{errors::Error, log};
use substreams_ethereum::{pb::eth::v2::Block, Event};

#[substreams::handlers::map]
fn map_factory_events(block: Block, store: StoreGetProto<Factory>) -> Result<FactoryEvents, Error> {
    let mut events = vec![];

    for log in block.logs() {
        let address = pretty_hex(&log.address());
        if let Some(factory) = store.get_last(&keyer::factory_key(&address)) {
            if let Some(event) = ChannelDeployed::match_and_decode(log) {
                log::info!("Channel deployed: {:?}", event);
                events.push(FactoryEvent {
                    address: address.clone(),
                    event: Some(factory_event::Event::ChannelDeployed(
                        factory_event::ChannelDeployed {
                            channel: pretty_hex(&event.channel),
                            deployer: pretty_hex(&event.deployer),
                            version: factory.version,
                        },
                    )),
                    metadata: extract_metadata(&log, &block),
                    ordinal: log.block_index() as u64,
                });
            }
            if let Some(event) = SplitDeployed::match_and_decode(log) {
                log::info!("Splits deployed: {:?}", event);
                events.push(FactoryEvent {
                    address: address.clone(),
                    event: Some(factory_event::Event::SplitDeployed(
                        factory_event::SplitDeployed {
                            split: pretty_hex(&event.split),
                            channel: pretty_hex(&event.channel),
                            version: factory.version,
                        },
                    )),
                    metadata: extract_metadata(&log, &block),
                    ordinal: log.block_index() as u64,
                });
            }
            if let Some(event) = DropDeployed::match_and_decode(log) {
                log::info!("Drop deployed: {:?}", event);
                events.push(FactoryEvent {
                    address: address.clone(),
                    event: Some(factory_event::Event::MysteryBoxDropDeployed(
                        factory_event::MysteryBoxDropDeployed {
                            drop: pretty_hex(&event.drop),
                            channel: pretty_hex(&event.channel),
                            conduit: pretty_hex(&event.conduit),
                            version: factory.version,
                        },
                    )),
                    metadata: extract_metadata(&log, &block),
                    ordinal: log.block_index() as u64,
                });
            }
        }
    }

    Ok(FactoryEvents { events })
}
