use crate::{
    abi::SplitMain::events,
    pb::masterfile::splits::v1::split_event::{CreateSplit, Event, SplitAllocation},
    utils::pretty_hex,
};

pub fn translate(event: events::CreateSplit) -> Event {
    Event::CreateSplit(CreateSplit {
        split: pretty_hex(&event.split),
        distributor_fee: event.distributor_fee.to_string(),
        controller: pretty_hex(&event.controller),
        allocations: event
            .percent_allocations
            .iter()
            .zip(event.accounts.iter())
            .map(|(percent, account)| SplitAllocation {
                account: pretty_hex(account),
                percent_allocation: percent.to_u64(),
            })
            .collect(),
    })
}
