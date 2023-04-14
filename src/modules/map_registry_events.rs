use common::utils::pretty_hex;
use substreams::Hex;
use substreams::{errors::Error, log};
use substreams_ethereum::{pb::eth::v2::Block, Event};

use crate::abi::Registry::events;
use crate::pb::masterfile::registry::v1::{
    registry_event::{
        self, DeploymentAdded, FactoryAdded, RoleAdminChanged, RoleGranted, RoleRevoked,
    },
    RegistryEvent, RegistryEvents,
};
use crate::utils::extract_metadata;

#[substreams::handlers::map]
fn map_registry_events(param: String, block: Block) -> Result<RegistryEvents, Error> {
    let mut events = vec![];

    let factory_address = &Hex::decode(&param).unwrap();

    for log in block.logs() {
        if log.address() == factory_address {
            if let Some(event) = events::FactoryAdded::match_and_decode(log) {
                log::info!("FactoryAdded: {:?}", event);
                events.push(RegistryEvent {
                    r#event: Some(registry_event::Event::FactoryAdded(FactoryAdded {
                        version: event.version.to_u64(),
                        name: pretty_hex(&event.name),
                        factory: pretty_hex(&event.factory),
                    })),
                    address: pretty_hex(&factory_address),
                    ordinal: log.block_index() as u64,
                    metadata: extract_metadata(&log, &block),
                });
            }
            if let Some(event) = events::DeploymentAdded::match_and_decode(log) {
                log::info!("DeploymentAdded: {:?}", event);
                events.push(RegistryEvent {
                    r#event: Some(registry_event::Event::DeploymentAdded(DeploymentAdded {
                        deployment: pretty_hex(&event.deployment),
                        name: pretty_hex(&event.name),
                    })),
                    address: pretty_hex(&factory_address),
                    ordinal: log.block_index() as u64,
                    metadata: extract_metadata(&log, &block),
                });
            }
            if let Some(event) = events::RoleAdminChanged::match_and_decode(log) {
                log::info!("RoleAdminChanged: {:?}", event);
                events.push(RegistryEvent {
                    r#event: Some(registry_event::Event::RoleAdminChanged(RoleAdminChanged {
                        role: pretty_hex(&event.role),
                        previous_admin_role: pretty_hex(&event.previous_admin_role),
                        new_admin_role: pretty_hex(&event.new_admin_role),
                    })),
                    address: pretty_hex(&factory_address),
                    ordinal: log.block_index() as u64,
                    metadata: extract_metadata(&log, &block),
                });
            }
            if let Some(event) = events::RoleGranted::match_and_decode(log) {
                log::info!("RoleGranted: {:?}", event);
                events.push(RegistryEvent {
                    r#event: Some(registry_event::Event::RoleGranted(RoleGranted {
                        role: pretty_hex(&event.role),
                        account: pretty_hex(&event.account),
                        sender: pretty_hex(&event.sender),
                    })),
                    address: pretty_hex(&factory_address),
                    ordinal: log.block_index() as u64,
                    metadata: extract_metadata(&log, &block),
                });
            }
            if let Some(event) = events::RoleRevoked::match_and_decode(log) {
                log::info!("RoleRevoked: {:?}", event);
                events.push(RegistryEvent {
                    r#event: Some(registry_event::Event::RoleRevoked(RoleRevoked {
                        role: pretty_hex(&event.role),
                        account: pretty_hex(&event.account),
                        sender: pretty_hex(&event.sender),
                    })),
                    address: pretty_hex(&factory_address),
                    ordinal: log.block_index() as u64,
                    metadata: extract_metadata(&log, &block),
                });
            }
        }
    }

    Ok(RegistryEvents { events })
}
