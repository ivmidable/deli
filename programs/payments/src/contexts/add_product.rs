use anchor_lang::prelude::*;
use std::collections::BTreeMap;

use crate::state::Product;

#[derive(Accounts)]
#[instruction(name: String)]
pub struct AddProduct<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init,
        payer = authority,
        seeds = [b"product", authority.key().as_ref(), name.as_bytes().as_ref()],
        bump,
        space = Product::LEN
    )]
    product: Account<'info, Product>,
    system_program: Program<'info, System>,
}

impl<'info> AddProduct<'info> {
    pub fn add_product(&mut self, bumps: &BTreeMap<String, u8>, name: String) -> Result<()> {
        self.product.name = name;
        self.product.authority = *self.authority.key;
        self.product.flags = 0;
        self.product.bump = *bumps.get("product").unwrap();
        Ok(())
    }
}
