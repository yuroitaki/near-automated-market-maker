use near_sdk::{Balance};
use uint::construct_uint;

use crate::error::*;

pub const MIN_DECIMAL: u8 = 1;
pub const MAX_DECIMAL: u8 = 24;
pub const FT_TRANSFER_DEPOSIT_YOCTO_NEAR: u128 = 1;

construct_uint! {
    /// 256-bit unsigned integer.
    pub struct U256(4);
}

pub fn amount_to_canonical_amount(amount: Balance, decimal: u8) -> Balance {
    let factor = 10u128
        .checked_pow((MAX_DECIMAL - decimal) as u32)
        .expect(INTERNAL_OVERFLOW_ERROR);

    amount
        .checked_mul(factor)
        .expect(INTERNAL_OVERFLOW_ERROR)
}

pub fn canonical_amount_to_amount(amount: Balance, decimal: u8) -> Balance {
    let factor = 10u128
        .checked_pow((MAX_DECIMAL - decimal) as u32)
        .expect(INTERNAL_OVERFLOW_ERROR);

    amount
        .checked_div(factor)
        .expect(INTERNAL_OVERFLOW_ERROR)
}
