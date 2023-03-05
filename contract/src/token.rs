use near_contract_standards::fungible_token::metadata::FungibleTokenMetadata;
use near_sdk::{
    AccountId, Balance, ext_contract,
    borsh::{self, BorshDeserialize, BorshSerialize},
    // json_types::U128,
    log,
    serde::{Deserialize, Serialize},
};

use crate::error::*;

pub const MIN_DECIMAL: u8 = 1;
pub const MAX_DECIMAL: u8 = 24;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Token {
    address: AccountId,
    name: Option<String>,
    ticker: Option<String>,
    decimal: Option<u8>,
    balance: Balance,
}

impl Token {
    pub fn new(address: AccountId) -> Self {
        Self {
            address,
            name: None,
            ticker: None,
            decimal: None,
            balance: 0,
        }
    }

    pub fn set_metadata(&mut self, metadata: FungibleTokenMetadata) {
        assert!(
            metadata.decimals >= MIN_DECIMAL && metadata.decimals <= MAX_DECIMAL,
            "{}",
            INVALID_TOKEN_DECIMAL
        );
        log!("Set token {} name: {}, ticker: {}, decimal: {}", self.address, metadata.name, metadata.symbol, metadata.decimals);

        self.name = Some(metadata.name);
        self.ticker = Some(metadata.symbol);
        self.decimal = Some(metadata.decimals);
    }

    pub fn get_metadata(&self) -> TokenMetadata {
        assert!(self.name.is_some(), "{}", TOKEN_METADATA_NOT_INITIALISED);
        assert!(self.ticker.is_some(), "{}", TOKEN_METADATA_NOT_INITIALISED);
        assert!(self.decimal.is_some(), "{}", TOKEN_METADATA_NOT_INITIALISED);

        TokenMetadata {
            name: self.name.as_ref().unwrap().clone(),
            ticker: self.ticker.as_ref().unwrap().clone(),
            decimal: self.decimal.unwrap(),
        }
    }

    pub fn get_address(&self) -> &AccountId {
        &self.address
    }

    pub fn get_balance(&self) -> Balance {
        self.balance
    }

    pub fn set_balance(&mut self, amount: Balance) {
        self.balance = self.balance.checked_add(amount).expect(INTERNAL_OVERFLOW_ERROR);
    }

    pub fn check_address(&self, address: &AccountId) -> bool {
        self.address.as_str() == address.as_str()
    }

    pub fn get_canonical_balance(&self) -> Balance {
        assert!(self.decimal.is_some(), "{}", TOKEN_METADATA_NOT_INITIALISED);

        let factor = 10u128
            .checked_pow((MAX_DECIMAL - self.decimal.unwrap()) as u32)
            .expect(INTERNAL_OVERFLOW_ERROR);

        self.balance.checked_mul(factor).expect(INTERNAL_OVERFLOW_ERROR)
    }
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct TokenMetadata {
    name: String,
    ticker: String,
    decimal: u8
}

#[ext_contract(ext_fungible_token)]
trait FungibleToken {
    fn ft_metadata(&self) -> FungibleTokenMetadata;

    // fn ft_transfer(&mut self, receiver_id: AccountId, amount: U128, memo: Option<String>);

    // fn ft_balance_of(&self, account_id: AccountId) -> U128;
}
