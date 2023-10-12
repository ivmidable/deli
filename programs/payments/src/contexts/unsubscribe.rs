use anchor_lang::prelude::*;

use crate::state::{Product, Subscription, Tier};

#[derive(Accounts)]
pub struct Unsubscribe<'info> {
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
    #[account(mut,
        close = user,
        seeds = [b"subscription", product.key().as_ref(), tier.key().as_ref(), user.key().as_ref()],
        bump = subscription.bump,
    )]
    pub subscription: Account<'info, Subscription>,
    pub system_program: Program<'info, System>,
}

impl<'info> Unsubscribe<'info> {
    pub fn unsubscribe(&self) -> Result<()> {
        // we can remove delegation here is we need to,
        // for now it's not needed, closeing the subscrition account
        // shoould make it impossible to collect from this user.
        Ok(())
    }
}
