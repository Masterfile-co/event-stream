use crate::{
    keyer::{self, DeploymentType},
    pb::masterfile::{
        deployment::v1::{
            deployment::Type, ChannelDeployment, Deployment, MysteryBoxDrop, SplitDeployment,
        },
        factory::v1::{factory_event, FactoryEvents},
    },
};
use substreams::prelude::*;

#[substreams::handlers::store]
fn store_deployments(events: FactoryEvents, store: StoreSetProto<Deployment>) {
    for factory_event in events.events {
        if let Some(event) = factory_event.event {
            match event {
                factory_event::Event::ChannelDeployed(channel_deployed) => {
                    store.set(
                        factory_event.ordinal,
                        keyer::deployment_key(&channel_deployed.channel, DeploymentType::Channel),
                        &Deployment {
                            address: channel_deployed.channel,
                            version: channel_deployed.version,
                            ordinal: factory_event.ordinal,
                            r#type: Some(Type::Channel(ChannelDeployment {})),
                        },
                    );
                }
                factory_event::Event::SplitDeployed(split_deployed) => {
                    store.set(
                        factory_event.ordinal,
                        keyer::deployment_key(&split_deployed.split, DeploymentType::Splits),
                        &Deployment {
                            address: split_deployed.split,
                            version: split_deployed.version,
                            ordinal: factory_event.ordinal,
                            r#type: Some(Type::Split(SplitDeployment {})),
                        },
                    );
                }
                factory_event::Event::MysteryBoxDropDeployed(mystery_box_drop_deployed) => {
                    store.set(
                        factory_event.ordinal,
                        keyer::deployment_key(
                            &mystery_box_drop_deployed.drop,
                            DeploymentType::MysteryBoxDrop,
                        ),
                        &Deployment {
                            address: mystery_box_drop_deployed.drop,
                            ordinal: factory_event.ordinal,
                            version: mystery_box_drop_deployed.version,
                            r#type: Some(Type::MysteryBoxDrop(MysteryBoxDrop {})),
                        },
                    );
                }
                _ => {}
            }
        }
    }
}
