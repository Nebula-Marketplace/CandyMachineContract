#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
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
    GetPhaseResponse,
    // MintingInfo,
};
use crate::state::{State, STATE, Phase};

use serde::{Deserialize, Serialize};

// use serde_json::from_slice;

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
        stopped: false,
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
        ExecuteMsg::Update { start, end, phase } => execute::set_phase_times(deps, env, phase, start, end, info.sender.to_string()),
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct User {
    pub address: String,
    pub phases: Vec<u32>,
}

pub mod execute {
    use cosmwasm_std::{Decimal, Uint128};

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

        // check if the phase is correct
        if s.phases[s.current_phase as usize].ends <= _env.block.time.seconds().into() {
            s.current_phase += 1;
        }

        // check if enough funds were passed
        if info.funds[0].amount < s.phases[s.current_phase as usize].price + (s.phases[s.current_phase as usize].price * Decimal::percent(3)) {
            return Err(ContractError::InsufficientFunds {});
        }

        // check if the sender is allowed to mint in this phase
        if s.phases[s.current_phase as usize].allowed.contains(&info.sender.as_str().to_string()) == false && s.phases[s.current_phase as usize].allowed[0] != "*".to_string() {
            return Err(ContractError::Unauthorized {});
        }

        // let user = deps.storage.get(info.sender.as_bytes());

        // let mut data: User;

        // if user.is_none() { 
        //     data = User {
        //         address: info.sender.to_string(),
        //         phases: vec![]
        //     };
        //     while data.phases.len() <= s.current_phase as usize {
        //         data.phases.push(0);
        //     }
        // } else {
        //     data = from_slice(&user.unwrap()).expect("Could not deserialize user data");
        // }

        // if data.phases[s.current_phase as usize] >= s.phases[s.current_phase as usize].allocation as u32 {
        //     return Err(ContractError::Unauthorized {});
        // }

        // data.phases[s.current_phase as usize] += 1;
        s.last_minted += 1;

        // deps.storage.set(info.sender.as_bytes(), &to_binary(&data).unwrap());
        STATE.save(deps.storage, &s)?;

        return Ok(
            Response::new()
            .add_attribute("action", "mint")
            .add_attribute("token", (&s.last_minted - 1).to_string())
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
            )
        );
    }

    pub fn set_phase_times(deps: DepsMut, _env: Env, phase: usize, start: Uint128, end: Uint128, sender: String) -> Result<Response, ContractError> {
        let mut s = STATE.load(deps.storage)?;

        if s.owner != sender {
            return Err(ContractError::Unauthorized {});
        }

        s.phases[phase].starts = start.u128() as i128;
        s.phases[phase].ends = end.u128() as i128;

        STATE.save(deps.storage, &s)?;

        return Ok(Response::new());
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetMetadata {} => to_binary(&query::get_metadata(deps)?),
        QueryMsg::GetPhase {} => to_binary(&query::get_phase(deps)?),
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
            contract: state.contract,
            phases: state.phases
        })
    }

    pub fn get_phase(deps: Deps) -> StdResult<Phase> {
        let state = STATE.load(deps.storage)?;
        let current = state.phases[state.current_phase as usize].clone();
        Ok(current)
    }
}