use crate::state::{Product, Tier};
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenInterface};
use std::collections::BTreeMap;

#[derive(Accounts)]
#[instruction(name:String)]
pub struct AddTier<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        has_one = authority,
        seeds = [b"product", product.authority.key().as_ref(), product.name.as_bytes().as_ref()],
        bump = product.bump
    )]
    pub product: Account<'info, Product>,

    #[account(
        init,
        payer = authority,
        seeds = [b"tier", product.key().as_ref(), name.as_bytes().as_ref()],
        bump,
        space = Tier::LEN
    )]
    pub tier: Account<'info, Tier>,
    pub mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> AddTier<'info> {
    pub fn add_tier(
        &mut self,
        bumps: &BTreeMap<String, u8>,
        name: String,
        term: i64,
        trial: i64,
        amount: u64,
    ) -> Result<()> {
        self.tier.name = name;
        self.tier.term = term;
        self.tier.trial = trial;
        self.tier.amount = amount;
        self.tier.mint = self.mint.key();
        self.tier.bump = *bumps.get("tier").unwrap();
        Ok(())
    }
}
