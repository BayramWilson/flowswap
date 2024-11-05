use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct ValidatorPoolState {
    pub staked_amount: u64,
    pub reward_rate: u16,
    pub total_stakers: u32,
    pub bump: u8,
}

/* impl ValidatorPoolState {
    pub const SIZE: usize = 8 + 2 + 4;
} */

