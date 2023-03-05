#/bin/bash
set -e

# !!! CHANGE THIS path to the path printed after deploying kurtosis near pacakge — https://docs.near.org/develop/testing/kurtosis-localnet#setup-environment-variables
NEAR_CLI_LOCALNET_KEY_PATH="/Users/christopher.chong/.neartosis/2023-03-04T15.02.01/validator-key.json"

BASE_COMMAND='NEAR_ENV="local" NEAR_CLI_LOCALNET_NETWORK_ID="localnet" NEAR_NODE_URL="http://127.0.0.1:8332" NEAR_CLI_LOCALNET_KEY_PATH=${NEAR_CLI_LOCALNET_KEY_PATH} NEAR_WALLET_URL="http://127.0.0.1:8334" NEAR_HELPER_URL="http://127.0.0.1:8330" NEAR_HELPER_ACCOUNT="test.near" NEAR_EXPLORER_URL="http://127.0.0.1:8331" near'

MASTER_ACC_ADDR="test.near"
FT_REGISTER_DEPOSIT=0.00125
FT_TRANFER_DEPOSIT_YOCTO=1

ETH_ACC_ADDR="eth.test.near"
ETH_ACC_INIT_NEAR=100
ETH_NAME="Ethereum"
ETH_SYMBOL="ETH"
ETH_DECIMAL=8
ETH_TOTAL_SUPPLY="1000000000000000"

SOL_ACC_ADDR="sol.test.near"
SOL_ACC_INIT_NEAR=100
SOL_NAME="Solana"
SOL_SYMBOL="SOL"
SOL_DECIMAL=6
SOL_TOTAL_SUPPLY="1000000000000000"

LP_ACC_ADDR="lp.test.near"
LP_ACC_INIT_NEAR=1000
LP_ACC_INIT_ETH="1000000000000"
LP_ACC_INIT_SOL="10000000000"

USER_ACC_ADDR="user.test.near"
USER_ACC_INIT_NEAR=1000
USER_ACC_INIT_ETH="10000000000"

FUNGILE_TOKEN_CONTRACT_LOCATION="./res/fungible_token.wasm"
FUNGILE_TOKEN_SPEC="ft-1.0.0"

eval "${BASE_COMMAND} create-account ${ETH_ACC_ADDR} --masterAccount ${MASTER_ACC_ADDR} --initialBalance ${ETH_ACC_INIT_NEAR}"
eval "${BASE_COMMAND} create-account ${SOL_ACC_ADDR} --masterAccount ${MASTER_ACC_ADDR} --initialBalance ${SOL_ACC_INIT_NEAR}"
eval "${BASE_COMMAND} create-account ${LP_ACC_ADDR} --masterAccount ${MASTER_ACC_ADDR} --initialBalance ${LP_ACC_INIT_NEAR}"
eval "${BASE_COMMAND} create-account ${USER_ACC_ADDR} --masterAccount ${MASTER_ACC_ADDR} --initialBalance ${USER_ACC_INIT_NEAR}"

eval "${BASE_COMMAND} deploy --wasmFile ${FUNGILE_TOKEN_CONTRACT_LOCATION} --accountId ${ETH_ACC_ADDR}"
eval "${BASE_COMMAND} call ${ETH_ACC_ADDR} new '{\"owner_id\": \"$ETH_ACC_ADDR\", \"total_supply\": \"$ETH_TOTAL_SUPPLY\", \"metadata\": { \"spec\": \"$FUNGILE_TOKEN_SPEC\", \"name\": \"$ETH_NAME\", \"symbol\": \"$ETH_SYMBOL\", \"decimals\": $ETH_DECIMAL }}' --accountId ${ETH_ACC_ADDR}"

eval "${BASE_COMMAND} call ${ETH_ACC_ADDR} storage_deposit '{\"account_id\": \"$LP_ACC_ADDR\"}' --accountId ${ETH_ACC_ADDR} --amount ${FT_REGISTER_DEPOSIT}"
eval "${BASE_COMMAND} call ${ETH_ACC_ADDR} ft_transfer '{\"receiver_id\": \"$LP_ACC_ADDR\", \"amount\": \"$LP_ACC_INIT_ETH\"}' --accountId ${ETH_ACC_ADDR} --depositYocto ${FT_TRANFER_DEPOSIT_YOCTO}"

eval "${BASE_COMMAND} call ${ETH_ACC_ADDR} storage_deposit '{\"account_id\": \"$USER_ACC_ADDR\"}' --accountId ${ETH_ACC_ADDR} --amount ${FT_REGISTER_DEPOSIT}"
eval "${BASE_COMMAND} call ${ETH_ACC_ADDR} ft_transfer '{\"receiver_id\": \"$USER_ACC_ADDR\", \"amount\": \"$USER_ACC_INIT_ETH\"}' --accountId ${ETH_ACC_ADDR} --depositYocto ${FT_TRANFER_DEPOSIT_YOCTO}"

eval "${BASE_COMMAND} deploy --wasmFile ${FUNGILE_TOKEN_CONTRACT_LOCATION} --accountId ${SOL_ACC_ADDR}"
eval "${BASE_COMMAND} call ${SOL_ACC_ADDR} new '{\"owner_id\": \"$SOL_ACC_ADDR\", \"total_supply\": \"$SOL_TOTAL_SUPPLY\", \"metadata\": { \"spec\": \"$FUNGILE_TOKEN_SPEC\", \"name\": \"$SOL_NAME\", \"symbol\": \"$SOL_SYMBOL\", \"decimals\": $SOL_DECIMAL }}' --accountId ${SOL_ACC_ADDR}"

eval "${BASE_COMMAND} call ${SOL_ACC_ADDR} storage_deposit '{\"account_id\": \"$LP_ACC_ADDR\"}' --accountId ${SOL_ACC_ADDR} --amount ${FT_REGISTER_DEPOSIT}"
eval "${BASE_COMMAND} call ${SOL_ACC_ADDR} ft_transfer '{\"receiver_id\": \"$LP_ACC_ADDR\", \"amount\": \"$LP_ACC_INIT_SOL\"}' --accountId ${SOL_ACC_ADDR} --depositYocto ${FT_TRANFER_DEPOSIT_YOCTO}"
