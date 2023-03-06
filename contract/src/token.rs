use near_contract_standards::fungible_token::metadata::FungibleTokenMetadata;
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    log,
    serde::{Deserialize, Serialize},
    AccountId,
    Balance,
};

use crate::error::*;
use crate::util::*;

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct TokenMetadata {
    address: AccountId,
    name: String,
    ticker: String,
    decimal: u8,
    pub ratio: f64,
}

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
        log!(
            "Set token {} name: {}, ticker: {}, decimal: {}",
            self.address,
            metadata.name,
            metadata.symbol,
            metadata.decimals
        );

        self.name = Some(metadata.name);
        self.ticker = Some(metadata.symbol);
        self.decimal = Some(metadata.decimals);
    }

    pub fn get_metadata(&self) -> TokenMetadata {
        TokenMetadata {
            address: self.address.clone(),
            name: self
                .name
                .as_ref()
                .expect(TOKEN_METADATA_NOT_INITIALISED)
                .clone(),
            ticker: self
                .ticker
                .as_ref()
                .expect(TOKEN_METADATA_NOT_INITIALISED)
                .clone(),
            decimal: self.decimal.expect(TOKEN_METADATA_NOT_INITIALISED),
            ratio: 1f64,
        }
    }

    pub fn get_address(&self) -> &AccountId {
        &self.address
    }

    pub fn check_address(&self, address: &AccountId) -> bool {
        self.address.as_str() == address.as_str()
    }

    pub fn get_decimal(&self) -> u8 {
        self.decimal.expect(TOKEN_METADATA_NOT_INITIALISED)
    }

    pub fn get_balance(&self) -> Balance {
        self.balance
    }

    pub fn add_balance(&mut self, amount: Balance) {
        self.balance = self
            .balance
            .checked_add(amount)
            .expect(INTERNAL_OVERFLOW_ERROR);
    }

    pub fn subtract_balance(&mut self, amount: Balance) {
        self.balance = self
            .balance
            .checked_sub(amount)
            .expect(INTERNAL_OVERFLOW_ERROR);
        assert!(self.balance > 0, "{}", INVALID_TOKEN_BALANCE);
    }

    pub fn get_canonical_balance(&self) -> Balance {
        amount_to_canonical_amount(
            self.balance,
            self.decimal.expect(TOKEN_METADATA_NOT_INITIALISED)
        )
    }
}
