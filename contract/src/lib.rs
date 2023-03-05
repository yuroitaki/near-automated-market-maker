mod error;
mod token;

use near_contract_standards::fungible_token::{
    metadata::FungibleTokenMetadata,
    receiver::FungibleTokenReceiver,
};
use near_sdk::{
    AccountId, BorshStorageKey, PanicOnDefault, Promise, PromiseOrValue, PromiseResult,
    borsh::{self, BorshDeserialize, BorshSerialize},
    env,
    json_types::U128,
    log, near_bindgen,
    serde::{Deserialize, Serialize},
    serde_json, Balance,
    store::Vector,
};
use std::fmt;

use error::*;
use token::{Token, TokenMetadata, ext_fungible_token};

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct ContractMetadata {
    tokens: Vec<TokenMetadata>,
    ratio: u128,
}

#[derive(BorshStorageKey, BorshSerialize)]
pub(crate) enum StorageKey {
    Token,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    owner_address: AccountId,
    tokens: Vector<Token>,
    constant_product: u128,
    functional: bool,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(owner_id: AccountId, token_a_id: AccountId, token_b_id: AccountId) -> Self {
        assert_ne!(owner_id, env::current_account_id(), "{}", OWNER_CANNOT_BE_CONTRACT_ACCOUNT_ITSELF);
        assert_ne!(token_a_id, token_b_id, "{}", DUPLICATE_TOKENS);

        Self::get_token_metadata(token_a_id.clone());
        Self::get_token_metadata(token_b_id.clone());

        let mut tokens = Vector::new(StorageKey::Token);
        tokens.push(Token::new(token_a_id));
        tokens.push(Token::new(token_b_id));

        Self {
            owner_address: owner_id,
            tokens,
            constant_product: 0,
            functional: false,
        }
    }

    pub fn get_metadata(&self) -> ContractMetadata {
        assert!(self.functional, "{}", AMM_NOT_FUNCTIONAL_YET);
        let ratio = self.tokens
            .iter()
            .fold(1u128, |acc, token| {
                token.get_canonical_balance().checked_div(acc).expect(INTERNAL_OVERFLOW_ERROR)
            });
        ContractMetadata {
            tokens: self.tokens.iter().map(|token| token.get_metadata()).collect(),
            ratio,
        }
    }

    fn deposit(&mut self, token_in: AccountId, amount: Balance) {
        self.tokens
            .iter_mut()
            .find(|token| token.check_address(&token_in))
            .expect(INVALID_LP_DEPOSIT_TOKEN)
            .set_balance(amount);

        match self.functional {
            false => {
                if self.tokens.iter().all(|token| token.get_balance() > 0) {
                    self.set_constant_product();
                    log!("Turning on the engine!");
                    self.functional = true;            
                }
            },
            true => {
                self.set_constant_product();
            }
        };
    }

    fn set_constant_product(&mut self) {
        self.constant_product = self.tokens
            .iter()
            .fold(1u128, |acc, token| {
                token.get_canonical_balance().checked_mul(acc).expect(INTERNAL_OVERFLOW_ERROR)
            })
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
        log!("Received callback from {}'s ft_metadata cross contract call!", token_id);
        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Successful(value) => {
                if let Ok(metadata) = serde_json::from_slice::<FungibleTokenMetadata>(&value) {
                    self.tokens
                        .iter_mut()
                        .find(|token| token.check_address(&token_id))
                        .expect(PROMISE_WRONG_VALUE_RECEIVED)
                        .set_metadata(metadata);
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
enum FungibleTokenReceiverMessage {
    LPDeposit,
    Swap,
}

impl fmt::Display for FungibleTokenReceiverMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FungibleTokenReceiverMessage::LPDeposit => write!(f, "lp_deposit"),
            FungibleTokenReceiverMessage::Swap => write!(f, "swap"),
        }
    }
}

#[near_bindgen]
impl FungibleTokenReceiver for Contract {
    fn ft_on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128> {
        let token_in = env::predecessor_account_id();
        log!("Received token {} from {}!", token_in, sender_id);
        
        if msg == FungibleTokenReceiverMessage::LPDeposit.to_string() {
            assert_eq!(sender_id, self.owner_address, "{}", INVALID_LP_DEPOSIT_SENDER);
            assert!(
                self.tokens.iter().any(|token| token.check_address(&token_in)),
                "{}",
                INVALID_LP_DEPOSIT_TOKEN,
            );
            assert!(amount > U128(0), "{}", INVALID_LP_DEPOSIT_AMOUNT);
            self.deposit(token_in, amount.0);
            PromiseOrValue::Value(U128(0))
        } else {
            env::panic_str(INVALID_TOKEN_RECEIVER_MESSAGE);
        }
    }
}
