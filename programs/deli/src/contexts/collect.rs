use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};

use crate::{
    errors::DeliError,
    state::{Registry, Subscription},
};

#[derive(Accounts)]
pub struct Collect<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    /// CHECK: auth is used to sign the transfer of funds from the user to the owner.
    #[account(
        seeds = [b"auth", subscription.user.key().as_ref(), mint.key().as_ref()],
        bump = subscription.auth_bump
    )]
    pub auth: UncheckedAccount<'info>,
    #[account(mut)]
    pub subscription: Account<'info, Subscription>,
    #[account(
        init_if_needed,
        payer = owner,
        associated_token::mint = mint,
        associated_token::authority = owner,
    )]
    pub owner_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = subscription.user
    )]
    pub user_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        has_one = mint
    )]
    pub registry: Account<'info, Registry>,
    pub mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> Collect<'info> {
    pub fn collect(&mut self) -> Result<()> {
        self.check_if_subscription_is_due()?;
        self.transfer_to_owner()?;
        self.update_subscription()
    }

    pub fn check_if_subscription_is_due(&self) -> Result<()> {
        if self.subscription.next_payment > Clock::get()?.unix_timestamp {
            return Err(DeliError::SubscriptionNotDue.into());
        }
        Ok(())
    }

    pub fn update_subscription(&mut self) -> Result<()> {
        self.subscription.next_payment = self.subscription.next_payment + self.registry.interval;
        self.subscription.nonce = self.subscription.nonce + 1;
        Ok(())
    }

    pub fn transfer_to_owner(&self) -> Result<()> {
        let user_key = self.subscription.user.key();
        let mint_key = self.mint.key();
        let seeds = &[
            b"auth",
            user_key.as_ref(),
            mint_key.as_ref(),
            &[self.subscription.auth_bump],
        ];
        let signer_seeds = &[&seeds[..]];

        let accounts = TransferChecked {
            from: self.user_ata.to_account_info(),
            to: self.owner_ata.to_account_info(),
            mint: self.mint.to_account_info(),
            authority: self.auth.to_account_info(),
        };

        let cpi_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            accounts,
            signer_seeds,
        );
        transfer_checked(cpi_ctx, self.registry.amount, self.mint.decimals)
    }
}
