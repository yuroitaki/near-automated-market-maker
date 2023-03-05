#!/bin/bash
set -e
set -o allexport
source "./script/var.conf"
set +o allexport

# !!! DANGER — This method will delete the AMM account. Beneficiary account must already be initialized in order to transfer all Near tokens or these will be lost.
# !!! Make sure to send all fungible tokens or NFTs that AMM owns to the beneficiary account prior to deleting, as this method will only transfer NEAR tokens.
eval "${BASE_COMMAND} delete ${AMM_ACC_ADDR} ${MASTER_ACC_ADDR}"

eval "${BASE_COMMAND} create-account ${AMM_ACC_ADDR} --masterAccount ${MASTER_ACC_ADDR} --initialBalance ${AMM_ACC_INIT_NEAR}"

eval "${BASE_COMMAND} call ${TOKEN_A_ACC_ADDR} storage_deposit '{\"account_id\": \"$AMM_ACC_ADDR\"}' --accountId ${TOKEN_A_ACC_ADDR} --amount ${FT_REGISTER_DEPOSIT}"
eval "${BASE_COMMAND} call ${TOKEN_B_ACC_ADDR} storage_deposit '{\"account_id\": \"$AMM_ACC_ADDR\"}' --accountId ${TOKEN_B_ACC_ADDR} --amount ${FT_REGISTER_DEPOSIT}"

eval "${BASE_COMMAND} deploy --wasmFile ${AMM_CONTRACT_LOCATION} --accountId ${AMM_ACC_ADDR}"
