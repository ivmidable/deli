use anchor_lang::prelude::*;
use std::collections::BTreeMap;

use crate::state::Registry;
use anchor_spl::token_interface::{Mint, TokenInterface};

#[derive(Accounts)]
#[instruction(nonce: u64)]
pub struct CreateSingle<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        init,
        payer = admin,
        seeds = [b"registry", admin.key().as_ref(), mint.key().as_ref(), nonce.to_le_bytes().as_ref()],
        bump,
        space = Registry::LEN
    )]
    registry: Account<'info, Registry>,
    mint: InterfaceAccount<'info, Mint>,
    token_program: Interface<'info, TokenInterface>,
    system_program: Program<'info, System>,
}

impl<'info> CreateSingle<'info> {
    pub fn create_single(
        &mut self,
        bumps: &BTreeMap<String, u8>,
        amount: u64,
        nonce: u64,
    ) -> Result<()> {
        self.registry.name = "".to_string();
        self.registry.admin = self.admin.key();
        self.registry.mint = self.mint.key();
        self.registry.amount = amount;
        self.registry.nonce = nonce;
        self.registry.interval = 0;
        self.registry.bump = *bumps.get("regstry").unwrap();
        Ok(())
    }
}
