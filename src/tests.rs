use cosmwasm_std::{Addr, Empty, Uint128, Coin, coins};
use cw_multi_test::{App, ContractWrapper, Executor};
use nft_multi_test;

use std::time::{self, UNIX_EPOCH};

use crate::{contract::*, msg::{InstantiateMsg, ExecuteMsg, Creator}, state::Phase};

pub type Extension = Option<Empty>;

#[test]
fn init() {
    let mut app = App::default();

    let code = ContractWrapper::new(execute, instantiate, query);
    let code_id = app.store_code(Box::new(code));
    let nft_id = app.store_code(nft_multi_test::cw721_contract());
    
    let nft_contract = app.instantiate_contract(
        nft_id,
        Addr::unchecked("owner"),
        &nft_multi_test::InstantiateMsg {
            name: "Nebula NFT".to_string(),
            symbol: "NFT".to_string(),
            minter: "owner".to_string()
        },
        &vec![],
        "Nebula NFT",
        None
    ).expect("couldn't instantiate nft contract");

    app.instantiate_contract(
        code_id, 
        Addr::unchecked("owner"), 
        &InstantiateMsg {
            collection: "collection".to_string(),
            contract: nft_contract.to_string(),
            description: "description".to_string(),
            symbol: "symbol".to_string(),
            logo_uri: "logo_uri".to_string(),
            banner_uri: "banner_uri".to_string(),
            supply: 100,
            creators: vec![Creator {
                address: "creator".to_string(),
                share: 100,
            }],
            basis_points: 100,
            phases: vec![],
            codeid: 2619,
        }, 
        &vec![], 
        "Test", 
        None
    ).expect("contract failed to instantiate");
    println!("contract instantiated")
}

#[test]
fn mint() {
    let mut app = App::default();
    let code = ContractWrapper::new(execute, instantiate, query);
    let code_id = app.store_code(Box::new(code));
    let nft_id = app.store_code(nft_multi_test::cw721_contract());

    // instantiate cw721 contract
    let nft_contract = app.instantiate_contract(
        nft_id,
        Addr::unchecked("owner"),
        &nft_multi_test::InstantiateMsg {
            name: "Nebula NFT".to_string(),
            symbol: "NFT".to_string(),
            minter: "owner".to_string()
        },
        &vec![],
        "Nebula NFT",
        None
    ).expect("couldn't instantiate nft contract");

    // mint a token
    app.execute_contract(
        Addr::unchecked("owner"),
        Addr::unchecked(&nft_contract),
        &nft_multi_test::ExecuteMsg::Mint(nft_multi_test::MintMsg::<Extension> {
            token_id: 0.to_string(),
            owner: "owner".to_string(),
            token_uri: Some("token_uri".to_string()),
            extension: None
        }),
        &vec![]
    ).expect("Minting is borked");

    // instantiate candymachine 
    let candyMachine = app.instantiate_contract(
        code_id, 
        Addr::unchecked("owner"), 
        &InstantiateMsg {
            collection: "collection".to_string(),
            contract: String::from(nft_contract),
            description: "description".to_string(),
            symbol: "symbol".to_string(),
            logo_uri: "logo_uri".to_string(),
            banner_uri: "banner_uri".to_string(),
            supply: 100,
            creators: vec![Creator {
                address: "creator".to_string(),
                share: 100,
            }],
            basis_points: 100,
            phases: vec![
                Phase { 
                    name: "public".to_string(), 
                    allowed: vec!["owner".to_string(),], 
                    price: Uint128::from(1000000000000 as u64), 
                    ends: Uint128::from(time::SystemTime::now().duration_since(UNIX_EPOCH).expect("time is broken").as_secs() + 1000), 
                    starts: Uint128::from(time::SystemTime::now().duration_since(UNIX_EPOCH).expect("time is broken").as_secs()), 
                    allocation: 5
                }
            ],
            codeid: 49,
        }, 
        &vec![], 
        "Test", 
        None
    ).expect("contract failed to instantiate");

    // mint a token
    app.execute_contract(
        Addr::unchecked("owner"), 
        candyMachine, 
        &ExecuteMsg::Mint { signature: "garbage".to_string() }, 
        &coins(1000000000000, "inj")
    ).expect("Minting is borked");
} 