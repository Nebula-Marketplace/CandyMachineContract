#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
#[allow(deprecated)]
use cosmwasm_std::{
    to_binary, 
    Binary, 
    Deps, 
    DepsMut, 
    Env, 
    MessageInfo, 
    Response, 
    StdResult, 
    Coin
};
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
use crate::state::{State, STATE, Phase};

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
        phases: msg.phases,
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
        ExecuteMsg::Mint { signature } => execute::mint(deps, &info, env, signature),
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct User {
    pub minted: Vec<i32>,
    pub allowed: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Tokens {
    owner: String,
    limit: i32,
    start_after: Option<String>
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct GetOwnedQuery {
    pub tokens: Tokens
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct GetOwnedQueryResponse {
    pub ids: Vec<String>
}

pub mod execute {
    use cosmwasm_std::{Decimal, BankMsg, Uint128};

    #[allow(unused_imports)]
    use crate::state;

    use super::*;

    // Need to pass a signed message from the contract verifier. this will be built into the mint site.
    /*
    message = {token_id: 1, pub: "address"}
    signature = sign(message)

    then pass that signature into the mint message, where we can verify it.
    This mechanism will protect against botting, as it means you have to mint through a website.
    Custom sites will init their contract with their own verifier, hence why this is not hardcoded.
    */
    #[allow(unused_variables)]
    pub fn mint(deps: DepsMut, info: &MessageInfo, _env: Env, signature: String) -> Result<Response, ContractError> {
        let mut s = STATE.load(deps.storage)?;

        if info.funds[0].denom != "inj" {
            return Err(ContractError::Unauthorized { reason: "wrong denom".to_string() });
        }

        // check if the phase is correct
        if s.phases[s.current_phase as usize].ends.u128() <= _env.block.time.seconds().into() {
            s.current_phase += 1;
            STATE.save(deps.storage, &s)?; // save the new state, just in case the remainder of the call fails. this will make querying faster too.
        }

        // check if enough funds were passed
        if info.funds[0].amount < s.phases[s.current_phase as usize].price + (s.phases[s.current_phase as usize].price * Decimal::percent(3)) {
            return Err(ContractError::InsufficientFunds {});
        }

        // check if the sender is allowed to mint in this phase
        if s.phases[s.current_phase as usize].allowed.contains(&info.sender.as_str().to_string()) == false && s.phases[s.current_phase as usize].allowed[0] != "*".to_string() {
            return Err(ContractError::Unauthorized { reason: "not in allowlist".to_string() });
        }

        let t: GetOwnedQueryResponse = deps.querier.query_wasm_smart(&s.contract, &GetOwnedQuery {
            tokens: Tokens {
                owner: info.sender.as_str().to_string(),
                limit: 30,
                start_after: None
            }
        })?;

        // check if the user has already minted the max amount of tokens
        if t.ids.len() >= s.phases[s.current_phase as usize].allocation as usize {
            return Err(ContractError::Unauthorized { reason: "max minted".to_string() });
        }
        
        s.last_minted += 1;

        STATE.save(deps.storage, &s)?;

        #[allow(deprecated)]
        Ok(
            Response::new()
            .add_attribute("action", "mint")
            .add_attribute("token", &s.last_minted.to_string())
            .add_message(
                MsgExecuteContract { 
                    contract_addr: s.contract, 
                    msg: to_binary(
                        &Tmessage{ 
                            transfer_nft: SendTokenMsg { 
                                recipient: info.sender.as_str().to_string(), 
                                token_id: (&s.last_minted - 1).to_string() // TODO: get token id
                            }
                        }
                    ).unwrap(),
                    funds: vec![] 
                }
            ).add_message(BankMsg::Send { to_address: s.owner, amount: vec![ Coin {
                amount: Uint128::from(info.funds[0].clone().amount.u128() - (s.phases[s.current_phase as usize].price * Decimal::percent(3)).u128()),
                denom: info.funds[0].clone().denom,
            }]})
            .add_message(BankMsg::Send { to_address: "inj1q7juqp9sw4sjahshanryw2a4qhenlmev9ygpm7".to_string(), amount: vec![Coin { denom: "inj".to_string(), amount: s.phases[s.current_phase as usize].price * Decimal::percent(3)}] })
        )
    }

    
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    #[allow(deprecated)]
    match msg {
        QueryMsg::GetMetadata {} => to_binary(&query::get_metadata(deps)?),
        QueryMsg::GetPhase {} => to_binary(&query::get_phase(deps, env)?),
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
            minted: state.last_minted,
            supply: state.supply,
            contract: state.contract,
            phases: state.phases
        })
    }

    pub fn get_phase(deps: Deps, env: Env) -> StdResult<Phase> {
        let state = STATE.load(deps.storage)?;
        let mut current = state.phases[state.current_phase as usize].clone();
        if current.ends.u128() < env.block.time.seconds().into() {
            current = state.phases[state.current_phase as usize + 1].clone();
        }
        Ok(current)
    }
}