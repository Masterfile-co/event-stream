use crate::pb::masterfile::{
    drop::v1::DropEvents,
    factory::v1::FactoryEvents,
    output::v1::{masterfile_event, MasterfileEvent, MasterfileEvents},
    registry::v1::RegistryEvents,
    safe::v1::SafeEvents,
    splits::v1::SplitEvents,
};

use substreams::errors::Error;

#[substreams::handlers::map]
fn map_output_events(
    registry_events: RegistryEvents,
    factory_events: FactoryEvents,
    drop_events: DropEvents,
    safe_events: SafeEvents,
    split_events: SplitEvents,
) -> Result<MasterfileEvents, Error> {
    let mut events = vec![];

    registry_events.events.iter().for_each(|event| {
        events.push(MasterfileEvent {
            event: Some(masterfile_event::Event::Registry(event.clone())),
        })
    });
    factory_events.events.iter().for_each(|event| {
        events.push(MasterfileEvent {
            event: Some(masterfile_event::Event::Factory(event.clone())),
        })
    });
    drop_events.events.iter().for_each(|event| {
        events.push(MasterfileEvent {
            event: Some(masterfile_event::Event::Drop(event.clone())),
        })
    });
    safe_events.events.iter().for_each(|event| {
        events.push(MasterfileEvent {
            event: Some(masterfile_event::Event::Safe(event.clone())),
        })
    });
    split_events.events.iter().for_each(|event| {
        events.push(MasterfileEvent {
            event: Some(masterfile_event::Event::Splits(event.clone())),
        })
    });

    // Sort events by block log index
    events.sort_by_key(|event| match &event.event {
        Some(masterfile_event::Event::Registry(event)) => {
            event.metadata.as_ref().unwrap().block_number
        }
        Some(masterfile_event::Event::Factory(event)) => {
            event.metadata.as_ref().unwrap().block_number
        }
        Some(masterfile_event::Event::Drop(event)) => event.metadata.as_ref().unwrap().block_number,
        Some(masterfile_event::Event::Safe(event)) => event.metadata.as_ref().unwrap().block_number,
        Some(masterfile_event::Event::Splits(event)) => {
            event.metadata.as_ref().unwrap().block_number
        }
        None => 0,
    });

    Ok(MasterfileEvents { events })
}
