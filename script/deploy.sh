#/bin/bash
set -e
set -o allexport
source "./script/var.conf"
set +o allexport

eval "${BASE_COMMAND} deploy --wasmFile ${AMM_CONTRACT_LOCATION} --accountId ${AMM_ACC_ADDR}"
