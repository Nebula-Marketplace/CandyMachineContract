#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Addr, BankMsg};
use cw2::set_contract_version;
use cosmwasm_std::WasmMsg::Execute as MsgExecuteContract;

use crate::error::ContractError;
use crate::msg::{
    ExecuteMsg, 
    GetMetadataResponse, 
    InstantiateMsg, 
    QueryMsg, 
    Tmessage, 
    SendTokenMsg, 
    // MintingInfo,
};
use crate::state::{State, STATE};

use serde::{Deserialize, Serialize};

// version info for migration info
const CONTRACT_NAME: &str = "Nebula CandyMachine";
const CONTRACT_VERSION: &str = "0.0.1";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct GetOwnerResponse {
    pub owner: String,
    pub approvals: Vec<String>,
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let contractAddress = msg.contract;
    let state = State {
        collection: msg.collection,
        contract: contractAddress.clone(),
        symbol: msg.symbol,
        description: msg.description,
        logo_uri: msg.logo_uri,
        banner_uri: msg.banner_uri,
        supply: msg.supply,
        owner: deps.querier.query_wasm_contract_info(contractAddress).unwrap().creator,
        phases: vec![],
        fee_paid: false,
        last_minted: 0,
        current_phase: 0,
        stopped: true,
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
    )
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Mint { } => execute::mint(deps, &info, env),
    }
}

pub mod execute {
    use cosmwasm_std::{Uint128, coins, WasmMsg};

    #[allow(unused_imports)]
    use crate::state;

    use super::*;

    pub fn mint(deps: DepsMut, info: &MessageInfo, _env: Env) -> Result<Response, ContractError> {
        let s = STATE.load(deps.storage)?;

        if info.funds[0].amount != s.phases[s.current_phase as usize].price {
            return Err(ContractError::InsufficientFunds {});
        }
        
        return Ok(
            Response::new()
            .add_attribute("action", "delist")
            .add_message(
                MsgExecuteContract { 
                    contract_addr: s.contract, 
                    msg: to_binary(
                        &Tmessage{ 
                            transfer_nft: SendTokenMsg { 
                                recipient: info.sender.as_str().to_string(), 
                                token_id: "1".to_string() // TODO: get token id
                            }
                        }
                    ).unwrap(),
                    funds: vec![] 
                }
            )
        );
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetMetadata {} => to_binary(&query::get_metadata(deps)?),
    }
}

pub mod query {
    use super::*;

    pub fn get_metadata(deps: Deps) -> StdResult<GetMetadataResponse> {
        let state = STATE.load(deps.storage)?;
        Ok(GetMetadataResponse {
            collection: state.collection,
            symbol: state.symbol,
            description: state.description,
            logo_uri: state.logo_uri,
            banner_uri: state.banner_uri,
            supply: state.supply,
            contract: state.contract
        })
    }
}

