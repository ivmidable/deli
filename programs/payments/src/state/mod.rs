use anchor_lang::prelude::*;

pub const CHILD_PRODUCT: u8 = 0b00000001;
pub const PRODUCT_PAUSED: u8 = 0b00000010;
pub const PRODUCT_APPROVED: u8 = 0b00000100;
const _PLACEHOLDER_1: u8 = 0b00001000;
const _PLACEHOLDER_2: u8 = 0b00010000;
const _PLACEHOLDER_3: u8 = 0b00100000;

#[account]
pub struct Product {
    pub authority: Pubkey,
    pub bump: u8,
    pub subscriptions: u32,
    pub name: String,
    pub flags: u8,
}

impl Product {
    pub const LEN: usize = 8 + 32 + 1 + 4 + (4 + 20) + 1;
}

#[account]
pub struct Tier {
    pub term: i64,
    pub trial: i64,
    pub mint: Pubkey,
    pub amount: u64,
    pub bump: u8,
    pub auth_bump: u8,
    pub name: String,
}

impl Tier {
    pub const LEN: usize = 8 + (8 * 2) + 32 + 8 + 1 + 1 + (4 + 20);
}

#[account]
pub struct Subscription {
    pub user: Pubkey,
    pub product: Pubkey,
    pub tier: Pubkey,
    pub startedAt: i64,
    pub nextPayment: i64,
    pub lastPayment: i64,
    pub bump: u8,
}

impl Subscription {
    pub const LEN: usize = 8 + 32 + 32 + 32 + 8 + 8 + 8 + 1;
}
