use serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone)]
pub struct DataStructFork {
    pub owner: String,
    pub cid: String,
    pub data_key: String,
}

#[derive(Serialize, Clone)]
pub struct BeatOwnership {
    pub url: String,
    pub owner: String,
}

#[derive(Serialize, Deserialize)]
pub struct OpenSeaAttributes {
    pub display_type: String,
    pub trait_type: String,
    pub value: i32,
}
