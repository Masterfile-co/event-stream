use crate::{
    keyer,
    pb::masterfile::{
        factory::v1::{
            factory::{ChannelFactory, MysteryBoxDropFactory, SplitsFactory, Type as FactoryType},
            Factory,
        },
        registry::v1::{registry_event, RegistryEvents},
    },
};
use substreams::prelude::*;

#[substreams::handlers::store]
fn store_factories(events: RegistryEvents, store: StoreSetProto<Factory>) {
    for registry_event in events.events {
        if let Some(event) = registry_event.event {
            match event {
                registry_event::Event::FactoryAdded(factory_added) => {
                    store.set(
                        registry_event.ordinal,
                        keyer::factory_key(&factory_added.factory),
                        &Factory {
                            address: factory_added.factory,
                            ordinal: registry_event.ordinal,
                            version: factory_added.version,
                            r#type: map_factory_type(&factory_added.name),
                        },
                    );
                }
                _ => {}
            }
        }
    }
}

const CHANNEL_TYPE: &str = "0x446e88dcc2f366f48c68cb0da4f16d5c3aa0bd3060e71140482c0cc6bd95d989"; // keccak256(CHANNEL)
const SPLITS_TYPE: &str = "0x7672f8498473759579af06bd97e96383fed5dbe521f62fc2207b9880b99310b8"; // keccak256(SPLITS)
const EDITIONS_DIRECT_DROP_TYPE: &str =
    "0x5e6d027d6940d67ee65a43ec72aae651cf17d4b4891bab124cad5d406f6e7a10"; // keccak256(EDITIONS_DIRECT_DROP)
const MYSTERY_BOX_DROP_TYPE: &str =
    "0x1e145aed25958a5bbfb09bf63f3c650f71c22749950eae9289d7e2a76812c7fd"; // keccak256(MYSTERY_BOX_DROP)

pub fn map_factory_type(name: &str) -> Option<FactoryType> {
    match name {
        CHANNEL_TYPE => Some(FactoryType::Channel(ChannelFactory {})),
        SPLITS_TYPE => Some(FactoryType::Splits(SplitsFactory {})),
        MYSTERY_BOX_DROP_TYPE => Some(FactoryType::MysteryBoxDrop(MysteryBoxDropFactory {})),
        _ => None,
    }
}
