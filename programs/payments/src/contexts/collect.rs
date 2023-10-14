use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};

use crate::errors::SubscriptionError;
use crate::state::{Subscription, Tier};

#[derive(Accounts)]
pub struct Collect<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    /// CHECK: auth is used to sign the transfer of funds from the user to the owner.
    #[account(
        seeds = [b"auth", subscription.user.key().as_ref(), mint.key().as_ref()],
        bump
    )]
    pub auth: UncheckedAccount<'info>,
    pub subscription: Account<'info, Subscription>,
    #[account(
        init_if_needed,
        payer = owner,
        associated_token::mint = mint,
        associated_token::authority = owner,
    )]
    pub owner_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        associated_token::mint = mint,
        associated_token::authority = subscription.user
    )]
    pub user_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        has_one = mint
    )]
    pub tier: Account<'info, Tier>,
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
        if self.subscription.nextPayment > Clock::get()?.unix_timestamp {
            return Err(SubscriptionError::SubscriptionNotDue.into());
        }
        Ok(())
    }

    pub fn update_subscription(&mut self) -> Result<()> {
        self.subscription.nextPayment = self.subscription.nextPayment + self.tier.term;
        Ok(())
    }

    pub fn transfer_to_owner(&self) -> Result<()> {
        let product_key = self.subscription.product.key();
        let tier_key = self.subscription.tier.key();
        let seeds = &[
            b"auth",
            product_key.as_ref(),
            tier_key.as_ref(),
            &[self.tier.auth_bump],
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
        transfer_checked(cpi_ctx, self.tier.amount, self.mint.decimals)
    }
}
