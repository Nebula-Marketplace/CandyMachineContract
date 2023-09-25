use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Uint128;
use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct State {
    pub collection: String,
    pub contract: String,
    pub description: String,
    pub symbol: String,
    pub logo_uri: String,
    pub banner_uri: String,
    pub supply: i32,
    pub owner: String,
    pub phases: Vec<Phase>, 
    pub fee_paid: bool,
    pub last_minted: i32,
    pub current_phase: i8,
    pub stopped: bool
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema, Eq)]
pub struct Phase {
    pub allowed: Vec<String>,
    pub price: Uint128,
    pub ends: i128,
    pub starts: i128,
    pub allocation: i32,
}

pub const STATE: Item<State> = Item::new("state");