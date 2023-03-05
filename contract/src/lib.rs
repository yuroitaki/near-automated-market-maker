mod error;
mod token;

use near_contract_standards::fungible_token::metadata::FungibleTokenMetadata;
use near_sdk::{
    AccountId, PanicOnDefault, Promise, PromiseResult,
    borsh::{self, BorshDeserialize, BorshSerialize},
    env,
    log,
    near_bindgen,
    serde::{Deserialize, Serialize},
    serde_json,
};

use error::*;
use token::{Token, TokenMetadata, ext_fungible_token};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    owner_address: AccountId,
    token_a: Token,
    token_b: Token,
    constant_product: u128,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(owner_id: AccountId, token_a_id: AccountId, token_b_id: AccountId) -> Self {
        assert_ne!(owner_id, env::current_account_id(), "{}", OWNER_CANNOT_BE_CONTRACT_ACCOUNT_ITSELF);
        assert_ne!(token_a_id, token_b_id, "{}", DUPLICATE_TOKENS);
        Self::get_token_metadata(token_a_id.clone());
        Self::get_token_metadata(token_b_id.clone());
        Self {
            owner_address: owner_id,
            token_a: Token::new(token_a_id),
            token_b: Token::new(token_b_id),
            constant_product: 0,
        }
    }

    pub fn get_metadata(&self) -> ContractMetadata {
        let ratio = self.token_a.get_balance() as f64 / self.token_b.get_balance() as f64;
        ContractMetadata {
            token_a: self.token_a.get_metadata(),
            token_b: self.token_b.get_metadata(),
            ratio,
        }
    }

    fn get_token_metadata(token_id: AccountId) -> Promise {
        ext_fungible_token::ext(token_id.clone())
            .ft_metadata()
            .then(
                Self::ext(env::current_account_id())
                .post_fungible_token_metadata(token_id)
            )
    }

    #[private]
    pub fn post_fungible_token_metadata(&mut self, token_id: AccountId) {
        assert_eq!(env::promise_results_count(), 1, "{}", PROMISE_TOO_MANY_RESULTS);
        log!("Received callback from ft_metadata cross contract call!");
        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Successful(value) => {
                if let Ok(metadata) = serde_json::from_slice::<FungibleTokenMetadata>(&value) {
                    if token_id.as_str() == self.token_a.get_address().as_str() {
                        self.token_a.set_metadata(metadata);
                    } else if token_id.as_str() == self.token_b.get_address().as_str() {
                        self.token_b.set_metadata(metadata);
                    } else {
                        env::panic_str(PROMISE_WRONG_VALUE_RECEIVED);
                    }
                } else {
                    env::panic_str(PROMISE_WRONG_VALUE_RECEIVED);
                }
            },
            PromiseResult::Failed => env::panic_str(PROMISE_CALL_FAILED),
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct ContractMetadata {
    token_a: TokenMetadata,
    token_b: TokenMetadata,
    ratio: f64,
}
