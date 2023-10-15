use anchor_lang::prelude::*;

use crate::state::{Registry, Subscription, UnsubscribeEvent};

#[derive(Accounts)]
pub struct Unsubscribe<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    pub registry: Account<'info, Registry>,
    #[account(mut,
        close = user,
        seeds = [b"subscription", registry.key().as_ref(), user.key().as_ref()],
        bump = subscription.bump,
    )]
    pub subscription: Account<'info, Subscription>,
    pub system_program: Program<'info, System>,
}

impl<'info> Unsubscribe<'info> {
    pub fn unsubscribe(&self) -> Result<()> {
        emit!(UnsubscribeEvent {
            registry: self.registry.key(),
            user: self.user.key()
        });
        //we don't use this for anything at this point,
        //everything is done via anchor constraints.
        Ok(())
    }
}
