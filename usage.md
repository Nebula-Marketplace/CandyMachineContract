## Instantiating
On Inj testnet, our codeid is [NOT UPLOADED]
On Inj mainnet, our codeid is [NOT UPLOADED]

# How to use
Mint all NFTs from cw721 contract using the following message: 
```js
ExecuteContract {
    sender: account,
    contract: cw721_contract,
    funds: [],
    msg: {
        mint: {
            owner: account,
            metadata_uri: url,
            royalties: {
                seller_fee_basis_points: 100, // 1%
                creators: [
                    {address: account, share: 50},
                    {address: account2, share: 50}
                ],
                primary_sell_happened: true
            }
        }
    }
}
``` 
You'll need to execute this once for every NFT in the collection. You can bundle up to 10 of these messages into one transaction. The Cw721 contract can be minted by anyone, not just the owner. 

Next, you must instantiate the CandyMachine contract. If possible, you can put this in the last transaction, or have the owner sign it. use the init message as defined in actions. 

After instantiation, have the creator sign an ApproveAll message allowing the candy machine to send out the NFTs. Use the following message:
```js
MsgExecuteContract {
    sender: account,
    contract: cw721_contract,
    funds: [],
    msg: {
        ApproveAll: {
            operator: CandyMachine_contract,
            expires: utc_timestamp // set this timestamp a few days after mint for safety, or a few years from now.  
        }
    }
}
```

# Actions
This contract at v0.0.1 has the following actions:

## Minting 
To mint an NFT is pretty simple, simply execute the following transaction 
```js
MsgExecuteContract {
    sender: minter,
    contract: candy_machine,
    funds: [
        Coin { // use the Cosmwasm Coin type
            denom: "inj",
            amount: ((price * 10e18) + price * 0.03) // in the case of free mint, just pass 0.03, unless fee_paid is true on CM
        }
    ],
    msg: {
        Mint: {}
    }
}
```

## Pausing a mint 
To pause the mint in case of an enmergency, simply execute the pause function from the admin account.
```js
MsgExecuteContract {
    sender: admin,
    contract: candy_machine,
    funds: [],
    msg: {
        Pause: {}
    }
}
```

## Changing mint phase time
You can alter a mint phase's time after instantiation in case of catastrophe, etc. 
```js
MsgExecuteContract {
    sender: admin,
    contract: candy_machine,
    funds: [],
    msg: {
        UpdatePhaseTime: {
            index: 0, // which phase is it by index, not name. 
            new_start: i128,
            new_end: i128
        }
    }
}
```