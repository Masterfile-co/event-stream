pub fn factory_key(factory_address: &str) -> String {
    format!("factory:{}", factory_address)
}

pub enum DeploymentType {
    Channel,
    Splits,
    MysteryBoxDrop,
    Unknown,
}

pub fn deployment_key(deployment_address: &str, deployment_type: DeploymentType) -> String {
    let type_name = match deployment_type {
        DeploymentType::Channel => "channel",
        DeploymentType::Splits => "splits",
        DeploymentType::MysteryBoxDrop => "mystery_box_drop",
        DeploymentType::Unknown => "unknown",
    };

    format!("deployment:{}:{}", type_name, deployment_address)
}

pub fn event_key(tx_hash: &String, log_index: &u32) -> String {
    format!("event:{}:{}", tx_hash, log_index)
}
