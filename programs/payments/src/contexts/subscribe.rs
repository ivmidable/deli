use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::ApproveChecked,
    token_interface::{approve, Approve, Mint, TokenAccount, TokenInterface},
};

use crate::errors::SubscriptionError;
use crate::state::{Product, Subscription, Tier};

#[derive(Accounts)]
pub struct Subscribe<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        seeds = [b"product", product.authority.key().as_ref(), product.name.as_bytes().as_ref()],
        bump = product.bump
    )]
    pub product: Account<'info, Product>,
    #[account(
        has_one = mint,
        seeds = [b"tier", product.key().as_ref(), tier.name.as_bytes().as_ref()],
        bump = tier.bump
    )]
    pub tier: Account<'info, Tier>,
    #[account(
        init,
        payer = user,
        seeds = [b"subscription", product.key().as_ref(), tier.key().as_ref(), user.key().as_ref()],
        bump,
        space = Subscription::LEN
    )]
    pub subscription: Account<'info, Subscription>,
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
    pub fn subscribe(&mut self) -> Result<()> {
        //make sure they have enough funds for subscription even though we are only delegating.
        if self.user_ata.amount < self.tier.amount {
            return Err(SubscriptionError::InsufficientFunds.into());
        }

        self.subscription.user = self.user.key();
        self.subscription.product = self.product.key();
        self.subscription.tier = self.tier.key();
        self.subscription.startedAt = Clock::get()?.unix_timestamp;
        self.subscription.lastPayment = 0;
        self.subscription.nextPayment =
            Clock::get()?.unix_timestamp + self.tier.term + self.tier.trial;
        Ok(())
    }
}
