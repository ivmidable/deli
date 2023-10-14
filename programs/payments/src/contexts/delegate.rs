use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{approve, Approve, Mint, TokenAccount, TokenInterface},
};

#[derive(Accounts)]
pub struct Delegate<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        seeds = [b"auth", user.key().as_ref(), mint.key().as_ref()],
        bump
    )]
    /// CHECK: this is the auth that the user delegates to
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

impl<'info> Delegate<'info> {
    pub fn delegate(&mut self) -> Result<()> {
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
