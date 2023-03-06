mod error;
mod token;
mod util;

use near_contract_standards::fungible_token::{
    metadata::FungibleTokenMetadata, receiver::FungibleTokenReceiver,
};
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    env,
    ext_contract,
    json_types::U128,
    log, near_bindgen,
    serde::{Deserialize, Serialize},
    serde_json,
    store::Vector,
    AccountId, Balance, BorshStorageKey, PanicOnDefault, Promise, PromiseOrValue, PromiseResult,
};
use std::fmt;

use error::*;
use token::{Token, TokenMetadata};
use util::{amount_to_canonical_amount, canonical_amount_to_amount, FT_TRANSFER_DEPOSIT_YOCTO_NEAR, U256};

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct ContractMetadata {
    owner: AccountId,
    tokens: Vec<TokenMetadata>,
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
    functional: bool,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(owner_id: AccountId, token_a_id: AccountId, token_b_id: AccountId) -> Self {
        assert_ne!(
            owner_id,
            env::current_account_id(),
            "{}",
            OWNER_CANNOT_BE_CONTRACT_ACCOUNT_ITSELF
        );
        assert_ne!(token_a_id, token_b_id, "{}", DUPLICATE_TOKENS);

        Self::get_token_metadata(&token_a_id);
        Self::get_token_metadata(&token_b_id);

        let mut tokens = Vector::new(StorageKey::Token);
        tokens.push(Token::new(token_a_id));
        tokens.push(Token::new(token_b_id));

        Self {
            owner_address: owner_id,
            tokens,
            functional: false,
        }
    }

    pub fn get_metadata(&self) -> ContractMetadata {
        assert!(self.functional, "{}", AMM_NOT_FUNCTIONAL_YET);
        let token_a_balance = self
            .tokens
            .get(0)
            .expect(INTERNAL_INDEX_ERROR)
            .get_canonical_balance() as f64;
        let ratios: Vec<f64> = self
            .tokens
            .iter()
            .map(|token| token.get_canonical_balance() as f64 / token_a_balance)
            .collect();
        ContractMetadata {
            owner: self.owner_address.clone(),
            tokens: self
                .tokens
                .iter()
                .enumerate()
                .map(|(index, token)| {
                    let mut metadata = token.get_metadata();
                    metadata.ratio = *ratios.get(index).expect(INTERNAL_INDEX_ERROR);
                    metadata
                })
                .collect(),
        }
    }

    fn deposit(&mut self, token_in: AccountId, amount: Balance) {
        self.tokens
            .iter_mut()
            .find(|token| token.check_address(&token_in))
            .expect(INVALID_TOKEN_TRANSFERRED)
            .add_balance(amount);

        if !self.functional && self.tokens.iter().all(|token| token.get_balance() > 0) {
            log!("Turning on the AMM engine!");
            self.functional = true;
        };

        log!("Liquidity {} of token {} added!", amount, token_in);
    }

    fn swap(&mut self, token_in_address: &AccountId, amount_in: Balance) -> (AccountId, Balance) {
        let token_in = self.tokens
            .iter()
            .find(|token| token.check_address(token_in_address))
            .expect(INVALID_TOKEN_TRANSFERRED);
        let token_out = self.tokens
            .iter()
            .find(|token| !token.check_address(token_in_address))
            .expect(INVALID_TOKEN_TRANSFERRED);

        let token_out_address = token_out.get_address().clone();

        let canonical_balance_in = U256::from(token_in.get_canonical_balance());
        assert!(canonical_balance_in > U256::from(0), "{}", INVALID_TOKEN_BALANCE);

        let canonical_balance_out = U256::from(token_out.get_canonical_balance());
        assert!(canonical_balance_out > U256::from(0), "{}", INVALID_TOKEN_BALANCE);

        log!(
            "In token {}'s balance before swap: {}",
            token_in_address,
            canonical_balance_in, 
        );

        log!(
            "Out token {}'s balance before swap: {}",
            token_out_address,
            canonical_balance_out, 
        );

        let canonical_amount_in = U256::from(
            amount_to_canonical_amount(amount_in, token_in.get_decimal())
        );
        let canonical_amount_out = (canonical_amount_in * canonical_balance_out / (canonical_balance_in + canonical_amount_in)).as_u128();

        let amount_out = canonical_amount_to_amount(
            canonical_amount_out,
            token_out.get_decimal()
        );
        assert!(amount_out > 0, "{}", SLIPPAGE);

        self.tokens
            .iter_mut()
            .for_each(|token| {
                if token.check_address(token_in_address) {
                    token.add_balance(amount_in);
                    log!(
                        "In token {}'s balance after swap: {}",
                        token_in_address,
                        token.get_canonical_balance(),
                    )
                } else if token.check_address(&token_out_address) {
                    token.subtract_balance(amount_out);
                    log!(
                        "Out token {}'s balance after swap: {}",
                        token_out_address,
                        token.get_canonical_balance(),
                    )
                } else {
                    unreachable!()
                }
            });

        (token_out_address, amount_out)
    }

    fn send_token_out(&self, sender_id: AccountId, token_id: AccountId, amount: Balance) -> Promise {
        ext_fungible_token::ext(token_id)
            .with_attached_deposit(FT_TRANSFER_DEPOSIT_YOCTO_NEAR)
            .ft_transfer(
                sender_id,
                U128(amount),
                None,
            )
    }

    fn get_token_metadata(token_id: &AccountId) -> Promise {
        ext_fungible_token::ext(token_id.clone())
            .ft_metadata()
            .then(Self::ext(env::current_account_id()).post_fungible_token_metadata(token_id))
    }

    #[private]
    pub fn post_fungible_token_metadata(&mut self, token_id: &AccountId) {
        assert_eq!(
            env::promise_results_count(),
            1,
            "{}",
            PROMISE_TOO_MANY_RESULTS
        );
        log!(
            "Received callback from {}'s ft_metadata cross contract call!",
            token_id
        );
        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Successful(value) => {
                if let Ok(metadata) = serde_json::from_slice::<FungibleTokenMetadata>(&value) {
                    self.tokens
                        .iter_mut()
                        .find(|token| token.check_address(token_id))
                        .expect(PROMISE_WRONG_VALUE_RECEIVED)
                        .set_metadata(metadata);
                } else {
                    env::panic_str(PROMISE_WRONG_VALUE_RECEIVED);
                }
            }
            PromiseResult::Failed => env::panic_str(PROMISE_CALL_FAILED),
        }
    }
}

#[ext_contract(ext_fungible_token)]
trait FungibleToken {
    fn ft_metadata(&self) -> FungibleTokenMetadata;

    fn ft_transfer(
        &mut self,
        receiver_id: AccountId,
        amount: U128,
        memo: Option<String>,
    );
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
        assert!(amount > U128(0), "{}", INVALID_AMOUNT_TRANSFERRED);

        let token_in = env::predecessor_account_id();
        log!("Received {} token {} from {} with msg {}!", amount.0, token_in, sender_id, msg);

        assert!(
            self.tokens
                .iter()
                .any(|token| token.check_address(&token_in)),
            "{}",
            INVALID_TOKEN_TRANSFERRED,
        );

        if msg == FungibleTokenReceiverMessage::LPDeposit.to_string() {
            assert_eq!(
                sender_id, self.owner_address,
                "{}",
                INVALID_LP_DEPOSIT_SENDER
            );
            self.deposit(token_in, amount.0);

            PromiseOrValue::Value(U128(0))
        } else if msg == FungibleTokenReceiverMessage::Swap.to_string() {
            assert!(self.functional, "{}", AMM_NOT_FUNCTIONAL_YET);
            assert_ne!(
                sender_id,
                env::current_account_id(),
                "{}",
                SWAPPER_CANNOT_BE_CONTRACT_ACCOUNT_ITSELF
            );

            let (token_out, amount_out) = self.swap(&token_in, amount.0);
            self.send_token_out(sender_id, token_out, amount_out);

            PromiseOrValue::Value(U128(0))
        } else {
            env::panic_str(INVALID_TOKEN_RECEIVER_MESSAGE);
        }
    }
}
