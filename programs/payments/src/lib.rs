use anchor_lang::prelude::*;

mod contexts;
use contexts::*;
mod errors;
mod state;

declare_id!("EgYj5qqk1Kbq3eSwbUYwyDWQ6j3tnXkTej4MJJWkzqSQ");

#[program]
pub mod subscription {
    use super::*;

    pub fn add_product(ctx: Context<AddProduct>, name: String) -> Result<()> {
        ctx.accounts.add_product(&ctx.bumps, name)
    }

    pub fn add_tier(
        ctx: Context<AddTier>,
        name: String,
        term: i64,
        trial: i64,
        amount: u64,
    ) -> Result<()> {
        ctx.accounts.add_tier(&ctx.bumps, name, term, trial, amount)
    }

    pub fn add_child_product(ctx: Context<AddChildProduct>, name: String) -> Result<()> {
        ctx.accounts.add_child_product(&ctx.bumps, name)
    }

    pub fn delegate(ctx: Context<Delegate>) -> Result<()> {
        ctx.accounts.delegate()
    }

    pub fn subscribe(ctx: Context<Subscribe>) -> Result<()> {
        ctx.accounts.subscribe()
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

/*#[derive(Accounts)]
pub struct CancelSubscription {}

#[derive(Accounts)]
pub struct PauseSubscription {}

#[derive(Accounts)]
pub struct CollectPayment {}

#[derive(Accounts)]
pub struct RemoveSubscription<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        has_one = owner,
        seeds = [b"product", product.name.as_bytes().as_ref()],
        bump = product.bump
    )]
    pub product: Account<'info, Product>,
}

#[derive(Accounts)]
pub struct UpdateTier<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        has_one = owner,
        seeds = [b"product", product.name.as_bytes().as_ref()],
        bump = product.bump
    )]
    pub product: Account<'info, Product>,
}*/
