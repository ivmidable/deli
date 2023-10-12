use anchor_lang::prelude::*;
use std::collections::BTreeMap;

use crate::state::{Product, CHILD_PRODUCT};

#[derive(Accounts)]
#[instruction(name: String)]
pub struct AddChildProduct<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        seeds = [b"product", product.authority.key().as_ref(), product.name.as_bytes().as_ref()],
        bump = product.bump
    )]
    pub product: Account<'info, Product>,
    #[account(
        init,
        payer = authority,
        seeds = [b"product", authority.key().as_ref(), name.as_bytes().as_ref()],
        bump,
        space = Product::LEN
    )]
    pub child_product: Account<'info, Product>,
    pub system_program: Program<'info, System>,
}

impl<'info> AddChildProduct<'info> {
    pub fn add_child_product(&mut self, bumps: &BTreeMap<String, u8>, name: String) -> Result<()> {
        self.child_product.bump = *bumps.get("child_product").unwrap();
        self.child_product.authority = self.authority.key();
        self.child_product.flags = 0 | CHILD_PRODUCT;
        self.child_product.subscriptions = 0;
        self.child_product.name = name;
        Ok(())
    }
}
