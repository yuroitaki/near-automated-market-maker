pub const OWNER_CANNOT_BE_CONTRACT_ACCOUNT_ITSELF: &str =
    "The contract owner who is the liquidity provider cannot be the contract account itself";
pub const DUPLICATE_TOKENS: &str = "Token A and Token B cannot be the same.";
pub const SWAPPER_CANNOT_BE_CONTRACT_ACCOUNT_ITSELF: &str =
    "The person who wishes to swap the token cannot be the contract account itself";


pub const PROMISE_TOO_MANY_RESULTS: &str =
    "Cross contract call returned more than one promise result.";
pub const PROMISE_WRONG_VALUE_RECEIVED: &str = "Cross contract call returned invalid value.";
pub const PROMISE_CALL_FAILED: &str = "Cross contract call failed.";

pub const INVALID_TOKEN_DECIMAL: &str = "Token decimal is either too small (<1) or too big (>24).";
pub const INTERNAL_OVERFLOW_ERROR: &str =
    "There is an internal error when calculating due to overflow.";
pub const INTERNAL_INDEX_ERROR: &str =
    "There is an internal error when calculating due to index out of range.";

pub const AMM_NOT_FUNCTIONAL_YET: &str = "This AMM contract is not yet fully functional — most likely because liquidity has not been provided.";
pub const TOKEN_METADATA_NOT_INITIALISED: &str = "Token metadata has not been initialised.";
pub const INVALID_TOKEN_BALANCE: &str = "Token balance has reached 0.";
pub const SLIPPAGE: &str = "Token out is 0 — slippage happened!";

pub const INVALID_TOKEN_RECEIVER_MESSAGE: &str =
    "Invalid fungible token token receiver message, should be either 'lp_deposit' or 'swap'.";
pub const INVALID_LP_DEPOSIT_SENDER: &str = "lp_deposit sender is not the owner of this AMM.";
pub const INVALID_TOKEN_TRANSFERRED: &str =
    "Transferred token is not one of two tokens set on this AMM.";
pub const INVALID_AMOUNT_TRANSFERRED: &str = "Transferred amount cannot be zero.";
