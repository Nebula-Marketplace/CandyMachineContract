use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Uint128;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::state::Phase;

#[cw_serde]
pub struct InstantiateMsg {
    pub collection: String,
    pub contract: String,
    pub description: String,
    pub symbol: String,
    pub logo_uri: String,
    pub banner_uri: String,
    pub supply: i32,
    pub creators: Vec<Creator>,
    pub basis_points: i8, // 100 basis points = 1% of list price
    pub phases: Vec<Phase>,
    pub codeid: u64, // Cw721 code id. needs to have same mint interface as talis 
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Royalties {
    pub seller_fee_basis_points: i8,
    pub creators: Vec<Creator>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct OwnerOf {
    pub token_id: String,
} 

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct GetApprovals {
    token_id: String
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct MintingInfo {

}

// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
// pub struct ownerOfWrapper {
//     pub owner_of: OwnerOf
// }

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Creator {
    pub address: String,
    pub share: i8
}

#[cw_serde]
pub enum ExecuteMsg {
    Mint { signature: String},
    // Update { start: Uint128, end: Uint128, phase: u8 }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, JsonSchema)]
pub struct Tmessage {
    pub transfer_nft: SendTokenMsg
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, JsonSchema)]
pub struct Rmessage {
    pub revoke: Revoke
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, JsonSchema)]
pub struct SendTokenMsg {
    pub recipient: String,
    pub token_id: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, JsonSchema)]
pub struct Revoke {
    pub spender: String,
    pub token_id: String
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(GetMetadataResponse)]
    GetMetadata {},
    
    #[returns(GetPhaseResponse)]
    GetPhase {},
}

// We define a custom struct for each query response
#[cw_serde]
pub struct GetMetadataResponse {
    pub collection: String,
    pub description: String,
    pub symbol: String,
    pub logo_uri: String,
    pub banner_uri: String,
    pub minted: i32,
    pub supply: i32,
    pub contract: String,
    pub phases: Vec<Phase>,
}

#[cw_serde]
pub struct GetListedResponse {
    pub number: i32,
    pub listed: Vec<NFT>
}

#[cw_serde]
pub struct GetPhaseResponse {
    pub current: i8,
    pub price: Uint128,
    pub ends: i128,
    pub started: i128,
    pub allowlist: Vec<String>,
    pub name: String
}

#[cw_serde]
pub struct NFT {
    pub id: String,
    pub uri: String,
    pub owner: String,
    pub is_listed: bool
}