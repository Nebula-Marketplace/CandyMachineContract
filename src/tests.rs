use cosmwasm_std::{Addr, Empty, Uint128, coins};
use cw_multi_test::{App, ContractWrapper, Executor};
use nft_multi_test;

use std::time::{self, UNIX_EPOCH};

use crate::{contract::*, msg::{InstantiateMsg, ExecuteMsg, Creator}, state::Phase, error::ContractError};

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
    let mut app = App::new(|router, _, storage| {
        router
            .bank
            .init_balance(storage, &Addr::unchecked("buyer"), coins(2010000, "inj"))
            .unwrap()
    });
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
            contract: String::from(&nft_contract),
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
                    price: Uint128::zero(), 
                    ends: Uint128::from(time::SystemTime::now().duration_since(UNIX_EPOCH).expect("time is broken").as_secs() + 1000), 
                    starts: Uint128::from(time::SystemTime::now().duration_since(UNIX_EPOCH).expect("time is broken").as_secs()), 
                    allocation: 5,
                    denom: "inj".to_string()
                }
            ],
            codeid: 49,
        }, 
        &vec![], 
        "Test", 
        None
    ).expect("contract failed to instantiate");

    // approve CM for transfer
    app.execute_contract(
        Addr::unchecked("owner"),
        Addr::unchecked(&nft_contract),
        &nft_multi_test::ExecuteMsg::<Option<Empty>>::ApproveAll { 
            operator: String::from(&candyMachine), 
            expires: None
        },
        &vec![]
    ).expect("Minting is borked");

    // mint a token
    app.execute_contract(
        Addr::unchecked("owner"), 
        candyMachine, 
        &ExecuteMsg::Mint { signature: "garbage".to_string() }, 
        &coins(2010000, "inj")
    ).expect("Minting is borked");
}

#[test]
fn allowlist() {
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
            contract: String::from(&nft_contract),
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
                    price: Uint128::zero(), 
                    ends: Uint128::from(time::SystemTime::now().duration_since(UNIX_EPOCH).expect("time is broken").as_secs() + 1000), 
                    starts: Uint128::from(time::SystemTime::now().duration_since(UNIX_EPOCH).expect("time is broken").as_secs()), 
                    allocation: 5,
                    denom: "inj".to_string()
                }
            ],
            codeid: 49,
        }, 
        &vec![], 
        "Test", 
        None
    ).expect("contract failed to instantiate");

    // approve CM for transfer
    app.execute_contract(
        Addr::unchecked("owner"),
        Addr::unchecked(&nft_contract),
        &nft_multi_test::ExecuteMsg::<Option<Empty>>::ApproveAll { 
            operator: String::from(&candyMachine), 
            expires: None
        },
        &vec![]
    ).expect("Minting is borked");

    // attempt to mint a token
    let err: ContractError = app.execute_contract(
        Addr::unchecked("random"), 
        candyMachine, 
        &ExecuteMsg::Mint { signature: "garbage".to_string() }, 
        &vec![]
    ).unwrap_err().downcast().unwrap();

    assert_eq!(
        ContractError::Unauthorized { reason: "not in allowlist".to_string() }, 
        err
    );
}

#[test]
fn mint_limit() {
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

    // mint two tokens
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
    app.execute_contract(
        Addr::unchecked("owner"),
        Addr::unchecked(&nft_contract),
        &nft_multi_test::ExecuteMsg::Mint(nft_multi_test::MintMsg::<Extension> {
            token_id: 1.to_string(),
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
            contract: String::from(&nft_contract),
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
                    allowed: vec!["random".to_string(),], 
                    price: Uint128::zero(), 
                    ends: Uint128::from(time::SystemTime::now().duration_since(UNIX_EPOCH).expect("time is broken").as_secs() + 1000), 
                    starts: Uint128::from(time::SystemTime::now().duration_since(UNIX_EPOCH).expect("time is broken").as_secs()), 
                    allocation: 1,
                    denom: "inj".to_string()
                }
            ],
            codeid: 49,
        }, 
        &vec![], 
        "Test", 
        None
    ).expect("contract failed to instantiate");

    // approve CM for transfer
    app.execute_contract(
        Addr::unchecked("owner"),
        Addr::unchecked(&nft_contract),
        &nft_multi_test::ExecuteMsg::<Option<Empty>>::ApproveAll { 
            operator: String::from(&candyMachine), 
            expires: None
        },
        &vec![]
    ).expect("approval is borked");

    // attempt to mint tokens
    // first should succeed, second should fail
    app.execute_contract(
        Addr::unchecked("random"), 
        Addr::unchecked(&candyMachine), 
        &ExecuteMsg::Mint { signature: "garbage".to_string() }, // first mint should succeed
        &vec![]
    ).expect("Could not mint from CM");

    let err: ContractError = app.execute_contract(
        Addr::unchecked("random"), 
        Addr::unchecked(&candyMachine), 
        &ExecuteMsg::Mint { signature: "garbage".to_string() }, // first mint should succeed
        &vec![]
    ).unwrap_err().downcast().unwrap();

    assert_eq!(
        ContractError::Unauthorized { reason: "max minted".to_string() }, 
        err
    );
}