#!/bin/bash
set -e
set -o allexport
source "./script/var.conf"
set +o allexport

eval "${BASE_COMMAND} call ${AMM_ACC_ADDR} new '{\"owner_id\": \"$LP_ACC_ADDR\", \"token_a_id\": \"$TOKEN_A_ACC_ADDR\", \"token_b_id\": \"$TOKEN_B_ACC_ADDR\"}' --accountId ${LP_ACC_ADDR}"
