use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
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
        init_if_needed,
        payer = user,
        associated_token::mint = mint,
        associated_token::authority = auth,
    )]
    pub subscription_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        seeds = [b"auth",product.key().as_ref(), tier.key().as_ref()],
        bump
    )]
    /// CHECK: this is the auth for the subscription ata.
    pub auth: UncheckedAccount<'info>,
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

        self.subscription.user = *self.user.key;
        self.subscription.product = self.product.key();
        self.subscription.tier = self.tier.key();
        self.subscription.startedAt = Clock::get()?.unix_timestamp;
        self.subscription.lastPayment = 0;
        self.subscription.nextPayment =
            Clock::get()?.unix_timestamp + self.tier.term + self.tier.trial;
        self.approve(u64::MAX)
    }

    pub fn approve(&mut self, amount: u64) -> Result<()> {
        let accounts = Approve {
            to: self.user_ata.to_account_info(),
            delegate: self.auth.to_account_info(),
            authority: self.user.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), accounts);
        approve(cpi_ctx, amount)
    }
}
