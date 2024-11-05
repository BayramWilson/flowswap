use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct AmmPoolState {
    pub token_a_balance: u64,
    pub token_b_balance: u64,
    pub total_liquidity: u64,
    pub fee_rate: u64,
    pub bump: u8,
}

/* impl PoolState {
    pub const SIZE: usize = 8 + 8 + 8 + 2;
}
 */