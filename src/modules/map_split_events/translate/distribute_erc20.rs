use crate::{
    abi::SplitMain::events,
    pb::masterfile::splits::v1::split_event::{DistributeErc20, Event},
    utils::pretty_hex,
};

pub fn translate(event: events::DistributeErc20) -> Event {
    Event::DistributeErc20(DistributeErc20 {
        split: pretty_hex(&event.split),
        token: pretty_hex(&event.token),
        amount: event.amount.to_string(),
        distributor: pretty_hex(&event.distributor_address),
    })
}
