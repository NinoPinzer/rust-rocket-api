use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Root {
    pub deduplicationId: String,
    pub time: String,
    pub deviceInfo: DeviceInfo,
    pub devAddr: String,
    pub dr: u8,
    pub fPort: u8,
    pub data: String,
    pub rxInfo: Vec<RxInfo>,
    pub txInfo: TxInfo,
}

#[derive(Serialize, Deserialize)]
pub struct DeviceInfo {
    pub tenantId: String,
    pub tenantName: String,
    pub applicationId: String,
    pub applicationName: String,
    pub deviceProfileId: String,
    pub deviceProfileName: String,
    pub deviceName: String,
    pub devEui: String,
    pub tags: Tags,
}

#[derive(Serialize, Deserialize)]
pub struct Tags {
    pub key: String,
}

#[derive(Serialize, Deserialize)]
pub struct RxInfo {
    pub gatewayId: String,
    pub uplinkId: u32,
    pub rssi: i32,
    pub snr: f32,
    pub context: String,
    pub metadata: Metadata,
}

#[derive(Serialize, Deserialize)]
pub struct Metadata {
    pub region_name: String,
    pub region_common_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct TxInfo {
    pub frequency: u64,
    pub modulation: Modulation,
}

#[derive(Serialize, Deserialize)]
pub struct Modulation {
    pub lora: Lora,
}

#[derive(Serialize, Deserialize)]
pub struct Lora {
    pub bandwidth: u32,
    pub spreadingFactor: u8,
    pub codeRate: String,
}