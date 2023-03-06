# near-automated-market-maker

A simple automated market maker smart contract for NEAR blockchain.

## Features

- Automated market maker based on the constant product formula `x * y = k` (ref [here](https://jeiwan.net/posts/programming-defi-uniswap-1/))
- Supports swapping between two fungible tokens of arbitray decimals, but only limited to between 1 and 24 decimal points to prevent overflow
- Supports single liquidity provider, who needs to be the owner of the smart contract
- Tokens are being deposited into the smart contract's account itself, so the owner account cannot be the same as the smart contract account
- New wallets / subaccounts are not created by the smart contract to store the tokens because subaccounts cannot be controlled by the smart contract, making it not possible for the smart contract to transfer fund on behalf (ref [here](https://docs.near.org/develop/contracts/actions#create-a-sub-account))
- Slippage and overflow error detection mechanism
- Smart contract storage is constant as there is no growing state, so no management needed
- The smart contract used to create fungible tokens used in this project is forked from [here](https://github.com/near-examples/FT)

## Setup

This project has been tested on NEAR's local development environment using Kurtosis. However, it should be easy to tweak the [config](./script/var.conf) to deploy to testnet or mainnet.

1. Follow the instruction in NEAR's local development guide — make sure to finish the `Setup` [section](https://docs.near.org/develop/testing/kurtosis-localnet#testing).
2. Git clone this project.
3. Change the value of `NEAR_CLI_LOCALNET_KEY_PATH` in [config](./script/var.conf) to the value printed out by Kurtosis deployment log in step 1 above.
4. Change the value of `BASE_COMMAND` in [config](./script/var.conf) to the value `alias local_near=` printed out by Kurtosis deployment log in step 1 above.
5. Run the following at the top level of this project directory.

```bash
./script/setup.sh
```

6. The command above does the following things by following the config values in the [config](./script/var.conf)

- Set up 5 accounts: 2 for fungible token smart contracts, 1 for this AMM smart contract, 1 for the liquidity provider/owner, 1 for a user
- Deploy and initalise both fungible token smart contracts
- Whitelist all the other users for the fungible tokens
- Transfer some fungible tokens from their smart contracts to some users following the values defined in [config](./script/var.conf)

## Build

Run the following at the top level of this project directory.

```bash
./script/build.sh
```

## Deploy

Run the following at the top level of this project directory — which deploy this AMM contract in its own account.

```bash
./script/deploy.sh
```

## Interact

1. Initialise the smart contract by running the following at the top level of this project directory.

```bash
./script/initialise.sh
```

2. Provide liquidity as the lp to the AMM smart contract by running the following (please change the values below if the config has been changed).

```bash
local_near call eth.test.near ft_transfer_call '{"receiver_id": "amm.test.near", "amount": "10000000000", "msg": "lp_deposit"}' --gas "300000000000000" --accountId lp.test.near --depositYocto 1
```

3. Provide liquidity for another token.

```bash
local_near call sol.test.near ft_transfer_call '{"receiver_id": "amm.test.near", "amount": "100000000", "msg": "lp_deposit"}' --gas "300000000000000" --accountId lp.test.near --depositYocto 1
```

4. The contract is fully functional now — view the contract's metadata.

```bash
local_near view amm.test.near get_metadata
```

5. Swap token as the user.

```bash
local_near call eth.test.near ft_transfer_call '{"receiver_id": "amm.test.near", "amount": "100000000", "msg": "swap"}' --gas "300000000000000" --accountId user.test.near --depositYocto 1
```

6. Continue swapping or providing liquidity!

## Testing

1. Run the following in this [directory](./contract/) to trigger unit tests.

```bash
cargo test
```

2. Using the Interact steps above, one can manually test out different scenarios — when one wishes to reset this AMM smart contract, i.e. removing all the states/data stored, one can run the following command at the top project directory (BEWARE THIS REMOVES ALL TOKENS AND DATA HELD BY THE SMART CONTRACT!!!)

```bash
./script/reset_amm.sh
```

The command above will also recreate the smart contract account, whitelist it for all the fungible tokens and redeploy the smart contract.

3. [Integration test](https://docs.near.org/sdk/rust/testing/integration-tests) using different NEAR's SDK to simulate contract interation can also be written — it was not implemented due to time contraints.

## Potential future implementations

- Fees
- Liquidity provider tokens
- Ability to withdraw liquidity
- More than 2 tokens
- More than 1 liquidity provider
- More than 1 pool
- Different AMM formula
