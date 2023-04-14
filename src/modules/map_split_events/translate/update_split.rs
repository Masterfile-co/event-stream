use crate::{
    abi::SplitMain::events,
    pb::masterfile::splits::v1::split_event::{Event, SplitAllocation, UpdateSplit},
    utils::pretty_hex,
};
pub fn translate(event: events::UpdateSplit) -> Event {
    Event::UpdateSplit(UpdateSplit {
        split: pretty_hex(&event.split),
        distributor_fee: event.distributor_fee.to_string(),
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
