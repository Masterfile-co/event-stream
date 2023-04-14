use crate::{
    abi::SplitMain::events,
    pb::masterfile::splits::v1::split_event::{DistributeEth, Event},
    utils::pretty_hex,
};

pub fn translate(event: events::DistributeEth) -> Event {
    Event::DistributeEth(DistributeEth {
        split: pretty_hex(&event.split),
        amount: event.amount.to_string(),
        distributor: pretty_hex(&event.distributor_address),
    })
}
