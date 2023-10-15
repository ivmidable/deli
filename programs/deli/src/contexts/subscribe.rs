use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};
use std::collections::BTreeMap;

use crate::{
    errors::DeliError,
    state::{Registry, Subscription},
};

#[derive(Accounts)]
pub struct Subscribe<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        has_one = mint,
    )]
    pub registry: Account<'info, Registry>,
    #[account(
        init,
        payer = user,
        seeds = [b"subscription", registry.key().as_ref(), user.key().as_ref()],
        bump,
        space = Subscription::LEN
    )]
    pub subscription: Account<'info, Subscription>,
    /// CHECK: This auth is used to delegate to.
    #[account(
        seeds = [b"auth", user.key().as_ref(), mint.key().as_ref()],
        bump
    )]
    pub auth: UncheckedAccount<'info>,
    #[account(
        associated_token::mint = mint,
        associated_token::authority = user
    )]
    pub user_ata: InterfaceAccount<'info, TokenAccount>,
    pub mint: InterfaceAccount<'info, Mint>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> Subscribe<'info> {
    pub fn subscribe(&mut self, bumps: &BTreeMap<String, u8>) -> Result<()> {
        //make sure they have enough funds for subscription even though we are only delegating.
        if self.user_ata.amount < self.registry.amount {
            return Err(DeliError::InsufficientFunds.into());
        }

        self.subscription.user = self.user.key();
        self.subscription.registry = self.registry.key();
        self.subscription.started_at = Clock::get()?.unix_timestamp;
        self.subscription.nonce = 0;
        self.subscription.bump = *bumps.get("subscription").unwrap();
        self.subscription.auth_bump = *bumps.get("auth").unwrap();
        self.subscription.next_payment = Clock::get()?.unix_timestamp + self.registry.interval;
        Ok(())
    }
}
