use anchor_lang::prelude::*;

pub const INTERVAL: u8 = 0b00000001;
pub const SUB_REGSTRY: u8 = 0b00000010;
pub const REGISTRY_FROZEN: u8 = 0b00000100;
const _PLACEHOLDER_1: u8 = 0b00001000;
const _PLACEHOLDER_2: u8 = 0b00010000;
const _PLACEHOLDER_3: u8 = 0b00100000;

#[account]
pub struct Registry {
    pub admin: Pubkey,
    pub mint: Pubkey,
    pub amount: u64,
    pub nonce: u64,
    pub interval: i64,
    pub bump: u8,
    pub flags: u8,
    pub name: String,
}

impl Registry {
    pub const LEN: usize = 8 + 32 + 32 + (3 * 8) + 1 + 1 + 4;
}

#[account]
pub struct Subscription {
    pub user: Pubkey,
    pub registry: Pubkey,
    pub started_at: i64,
    pub nonce: u64,
    pub next_payment: i64,
    pub bump: u8,
    pub auth_bump: u8,
}

impl Subscription {
    pub const LEN: usize = 8 + 32 + 32 + 8 + 8 + 8 + 1 + 1;
}

#[event]
pub struct SubscribeEvent {
    pub registry: Pubkey,
    pub user: Pubkey,
    pub next_payment: i64,
}

#[event]
pub struct UnsubscribeEvent {
    pub registry: Pubkey,
    pub user: Pubkey,
}

#[event]
pub struct CreateEvent {
    pub registry: Pubkey,
    pub amount: u64,
}

#[event]
pub struct CreateIntervalEvent {
    pub registry: Pubkey,
    pub amount: u64,
    pub interval: i64,
}
