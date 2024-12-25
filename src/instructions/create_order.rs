use anchor_lang::prelude::*;

use crate::{
    constants::{MINTED_SEED, ORDER_SEED},
    state::{Minted, Order},
};

pub fn create_order(
    ctx: Context<CreateOrder>,
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
    order.id = id;
    order.admin = admin;
    order.total = total;
    order.limit_per_user = limit_per_user;
    order.price = price;
    order.token = token;
    order.start_time = start_time;
    order.end_time = end_time;

    let minted = &mut ctx.accounts.minted;
    minted.minted = 0;

    emit!(OrderCreated {
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
#[instruction(id: u64, fee: u16)]
pub struct CreateOrder<'info> {
    #[account(
        init,
        payer = payer,
        space = Order::LEN,
        seeds = [
            ORDER_SEED.as_bytes(),
            id.to_le_bytes().as_ref(),
        ],
        bump,
    )]
    pub order: Account<'info, Order>,

    #[account(
        init,
        payer = payer,
        space = Minted::LEN,
        seeds = [
            MINTED_SEED.as_bytes(),
            id.to_le_bytes().as_ref(),
            ],
        bump,
    )]
    pub minted: Account<'info, Minted>,

    /// The account paying for all rents
    #[account(mut)]
    pub payer: Signer<'info>,

    /// Solana ecosystem accounts
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[event]
pub struct OrderCreated {
    pub id: u64,
    pub admin: Pubkey,

    pub total: u64,
    pub limit_per_user: u64,
    pub price: u64,

    pub token: Pubkey,

    pub start_time: i64,
    pub end_time: i64,
}
