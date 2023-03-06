use near_sdk::{Balance};
use uint::construct_uint;

use crate::error::*;

// only allow decimal precision of range below
pub const MIN_DECIMAL: u8 = 1;
pub const MAX_DECIMAL: u8 = 24;
pub const FT_TRANSFER_DEPOSIT_YOCTO_NEAR: u128 = 1;

construct_uint! {
    // 256-bit unsigned integer to prevent constant product calculation from overflow
    pub struct U256(4);
}

// convert amount to a canonical form so that amount with different decimals can be compared and calculated
pub fn amount_to_canonical_amount(amount: Balance, decimal: u8) -> Balance {
    let factor = 10u128
        .checked_pow((MAX_DECIMAL - decimal) as u32)
        .expect(INTERNAL_OVERFLOW_ERROR);

    amount
        .checked_mul(factor)
        .expect(INTERNAL_OVERFLOW_ERROR)
}

// convert canonical amount back to amount to be stored in contract state
pub fn canonical_amount_to_amount(amount: Balance, decimal: u8) -> Balance {
    let factor = 10u128
        .checked_pow((MAX_DECIMAL - decimal) as u32)
        .expect(INTERNAL_OVERFLOW_ERROR);

    amount
        .checked_div(factor)
        .expect(INTERNAL_OVERFLOW_ERROR)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_amount_to_canonical_amount() {
        let expected_balance = 100u128;
        let balance = amount_to_canonical_amount(1, 22);
        assert_eq!(expected_balance, balance);
    }

    #[test]
    fn test_canonical_amount_to_amount() {
        let expected_balance = 1u128;
        let balance = canonical_amount_to_amount(100u128, 22);
        assert_eq!(expected_balance, balance);
    }
}
