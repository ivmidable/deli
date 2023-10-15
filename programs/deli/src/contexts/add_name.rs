use anchor_lang::prelude::*;
use std::collections::BTreeMap;

use crate::{errors::DeliError, state::Product};

#[derive(Accounts)]
#[instruction(name: String)]
pub struct AddName<'info> {
    #[account(mut)]
    admin: Signer<'info>,
    #[account(
        realloc = Registry::LEN + name.len(),
        has_one = admin,
        has_one = mint,
    )]
    registry: Account<'info, Registry>,
    mint: InterfaceAccount<'info, Mint>,
    token_program: Interface<'info, TokenInterface>,
    system_program: Program<'info, System>,
}

impl<'info> AddName<'info> {
    pub fn add_name(&mut self, name: String) -> Result<()> {
        self.registry.name = name;
        Ok(())
    }
}
