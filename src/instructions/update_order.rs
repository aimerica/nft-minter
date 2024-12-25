use anchor_lang::prelude::*;

use crate::{constants::ORDER_SEED, errors::MintError, state::Order};

pub fn update_order(
    ctx: Context<UpdateOrder>,
    id: u64,
    total: u64,
    limit_per_user: u64,
    price: u64,
    admin: Pubkey,
    token: Pubkey,
    start_time: i64,
    end_time: i64,
) -> Result<()> {
    let order = &mut ctx.accounts.order;
    if order.id != id {
        return err!(MintError::InvalidId);
    }
    if order.admin != ctx.accounts.payer.key() {
        return err!(MintError::InvalidAdmin);
    }
    order.admin = admin;
    order.total = total;
    order.limit_per_user = limit_per_user;
    order.price = price;
    order.token = token;
    order.start_time = start_time;
    order.end_time = end_time;

    emit!(OrderUpdated {
        id,
        admin,
        total,
        limit_per_user,
        price,
        token,
        start_time,
        end_time,
    });
    Ok(())
}

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct UpdateOrder<'info> {
    #[account(
        mut,
        seeds = [
            ORDER_SEED.as_bytes(),
            id.to_le_bytes().as_ref(),
        ],
        bump,
    )]
    pub order: Account<'info, Order>,

    /// The account paying for all rents
    #[account(mut)]
    pub payer: Signer<'info>,

    /// Solana ecosystem accounts
    pub system_program: Program<'info, System>,
}

#[event]
pub struct OrderUpdated {
    pub id: u64,
    pub admin: Pubkey,

    pub total: u64,
    pub limit_per_user: u64,
    pub price: u64,

    pub token: Pubkey,
    
    pub start_time: i64,
    pub end_time: i64,
}
