use anchor_lang::prelude::*;

mod contexts;
use contexts::*;
mod errors;
mod state;

declare_id!("7cr8PdkQzH1WoCk6gyrhHYJqXpGx3LZH7ttH1cxcD8MS");

#[program]
pub mod deli {
    use super::*;

    pub fn create_single(ctx: Context<CreateSingle>, nonce: u64, amount: u64) -> Result<()> {
        ctx.accounts.create_single(&ctx.bumps, amount, nonce)
    }

    pub fn create_interval(
        ctx: Context<CreateInterval>,
        nonce: u64,
        amount: u64,
        interval: i64,
    ) -> Result<()> {
        ctx.accounts
            .create_interval(&ctx.bumps, nonce, amount, interval)
    }

    pub fn delegate(ctx: Context<Delegate>, amount: Option<u64>) -> Result<()> {
        if let Some(amount) = amount {
            ctx.accounts.delegate(amount)
        } else {
            ctx.accounts.delegate(u64::MAX)
        }
    }

    pub fn subscribe(ctx: Context<Subscribe>) -> Result<()> {
        ctx.accounts.subscribe(&ctx.bumps)
    }

    pub fn unsubscribe(ctx: Context<Unsubscribe>) -> Result<()> {
        ctx.accounts.unsubscribe()
    }

    pub fn collect(ctx: Context<Collect>) -> Result<()> {
        ctx.accounts.collect()
    }
}

/*#[derive(Accounts)]
pub struct ApproveSubProduct<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        seeds = [b"product", product.authority.key().as_ref(), product.name.as_bytes().as_ref()],
        bump = product.bump
    )]
    pub product: Account<'info, Product>,
}

#[derive(Accounts)]
pub struct FreezeProduct<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        seeds = [b"product", product.authority.key().as_ref(), product.name.as_bytes().as_ref()],
        bump = product.bump
    )]
    pub product: Account<'info, Product>,
    }*/

/*#[derive(Accounts)]
pub struct RemoveProduct<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        has_one = authority,
        seeds = [b"product", authority.key().as_ref(), product.name.as_bytes().as_ref()],
        bump = product.bump
    )]
    pub product: Account<'info, Product>,
}*/
