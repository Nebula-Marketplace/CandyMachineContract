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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Mint {
    pub token_id: String,
    pub owner: String,
    pub phase: i8
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema, Eq)]
pub struct Phase {
    pub name: String,
    pub allowed: Vec<String>,
    pub price: Uint128,
    pub ends: Uint128,
    pub starts: Uint128,
    pub allocation: i32,
}

pub const STATE: Item<State> = Item::new("state");
pub const MINTS: Item<Vec<Mint>> = Item::new("mints");