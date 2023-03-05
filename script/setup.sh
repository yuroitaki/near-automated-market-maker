#/bin/bash
set -e
set -o allexport
source "./script/var.conf"
set +o allexport

eval "${BASE_COMMAND} create-account ${TOKEN_A_ACC_ADDR} --masterAccount ${MASTER_ACC_ADDR} --initialBalance ${TOKEN_A_ACC_INIT_NEAR}"
eval "${BASE_COMMAND} create-account ${TOKEN_B_ACC_ADDR} --masterAccount ${MASTER_ACC_ADDR} --initialBalance ${TOKEN_B_ACC_INIT_NEAR}"
eval "${BASE_COMMAND} create-account ${LP_ACC_ADDR} --masterAccount ${MASTER_ACC_ADDR} --initialBalance ${LP_ACC_INIT_NEAR}"
eval "${BASE_COMMAND} create-account ${USER_ACC_ADDR} --masterAccount ${MASTER_ACC_ADDR} --initialBalance ${USER_ACC_INIT_NEAR}"
eval "${BASE_COMMAND} create-account ${AMM_ACC_ADDR} --masterAccount ${MASTER_ACC_ADDR} --initialBalance ${AMM_ACC_INIT_NEAR}"

eval "${BASE_COMMAND} deploy --wasmFile ${FUNGIBLE_TOKEN_CONTRACT_LOCATION} --accountId ${TOKEN_A_ACC_ADDR}"
eval "${BASE_COMMAND} call ${TOKEN_A_ACC_ADDR} new '{\"owner_id\": \"$TOKEN_A_ACC_ADDR\", \"total_supply\": \"$TOKEN_A_TOTAL_SUPPLY\", \"metadata\": { \"spec\": \"$FUNGIBLE_TOKEN_CONTRACT_SPEC\", \"name\": \"$TOKEN_A_NAME\", \"symbol\": \"$TOKEN_A_SYMBOL\", \"decimals\": $TOKEN_A_DECIMAL }}' --accountId ${TOKEN_A_ACC_ADDR}"

eval "${BASE_COMMAND} call ${TOKEN_A_ACC_ADDR} storage_deposit '{\"account_id\": \"$LP_ACC_ADDR\"}' --accountId ${TOKEN_A_ACC_ADDR} --amount ${FT_REGISTER_DEPOSIT}"
eval "${BASE_COMMAND} call ${TOKEN_A_ACC_ADDR} ft_transfer '{\"receiver_id\": \"$LP_ACC_ADDR\", \"amount\": \"$LP_ACC_INIT_ETH\"}' --accountId ${TOKEN_A_ACC_ADDR} --depositYocto ${FT_TRANFER_DEPOSIT_YOCTO}"

eval "${BASE_COMMAND} call ${TOKEN_A_ACC_ADDR} storage_deposit '{\"account_id\": \"$USER_ACC_ADDR\"}' --accountId ${TOKEN_A_ACC_ADDR} --amount ${FT_REGISTER_DEPOSIT}"
eval "${BASE_COMMAND} call ${TOKEN_A_ACC_ADDR} ft_transfer '{\"receiver_id\": \"$USER_ACC_ADDR\", \"amount\": \"$USER_ACC_INIT_ETH\"}' --accountId ${TOKEN_A_ACC_ADDR} --depositYocto ${FT_TRANFER_DEPOSIT_YOCTO}"

eval "${BASE_COMMAND} call ${TOKEN_A_ACC_ADDR} storage_deposit '{\"account_id\": \"$AMM_ACC_ADDR\"}' --accountId ${TOKEN_A_ACC_ADDR} --amount ${FT_REGISTER_DEPOSIT}"

eval "${BASE_COMMAND} deploy --wasmFile ${FUNGIBLE_TOKEN_CONTRACT_LOCATION} --accountId ${TOKEN_B_ACC_ADDR}"
eval "${BASE_COMMAND} call ${TOKEN_B_ACC_ADDR} new '{\"owner_id\": \"$TOKEN_B_ACC_ADDR\", \"total_supply\": \"$TOKEN_B_TOTAL_SUPPLY\", \"metadata\": { \"spec\": \"$FUNGIBLE_TOKEN_CONTRACT_SPEC\", \"name\": \"$TOKEN_B_NAME\", \"symbol\": \"$TOKEN_B_SYMBOL\", \"decimals\": $TOKEN_B_DECIMAL }}' --accountId ${TOKEN_B_ACC_ADDR}"

eval "${BASE_COMMAND} call ${TOKEN_B_ACC_ADDR} storage_deposit '{\"account_id\": \"$LP_ACC_ADDR\"}' --accountId ${TOKEN_B_ACC_ADDR} --amount ${FT_REGISTER_DEPOSIT}"
eval "${BASE_COMMAND} call ${TOKEN_B_ACC_ADDR} ft_transfer '{\"receiver_id\": \"$LP_ACC_ADDR\", \"amount\": \"$LP_ACC_INIT_SOL\"}' --accountId ${TOKEN_B_ACC_ADDR} --depositYocto ${FT_TRANFER_DEPOSIT_YOCTO}"

eval "${BASE_COMMAND} call ${TOKEN_B_ACC_ADDR} storage_deposit '{\"account_id\": \"$USER_ACC_ADDR\"}' --accountId ${TOKEN_B_ACC_ADDR} --amount ${FT_REGISTER_DEPOSIT}"
eval "${BASE_COMMAND} call ${TOKEN_B_ACC_ADDR} storage_deposit '{\"account_id\": \"$AMM_ACC_ADDR\"}' --accountId ${TOKEN_B_ACC_ADDR} --amount ${FT_REGISTER_DEPOSIT}"
