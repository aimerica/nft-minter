#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;
use instructions::*;
mod constants;
mod errors;
pub mod instructions;
mod state;

declare_id!("7PQQGVHYqB3hFN5B3i53h8w5ZVTNFKt5XYXVA4q57bku");

#[program]
pub mod nft_minter {
    use super::*;

    pub fn buy_via_burn_token(ctx: Context<BuyViaBurnToken>, id: u64, amount: u64) -> Result<()> {
        buy_via_burn_token::buy_via_burn_token(ctx, id, amount)
    }

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
        create_order::create_order(
            ctx,
            id,
            total,
            limit_per_user,
            price,
            admin,
            token,
            start_time,
            end_time,
        )
    }
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
        update_order::update_order(
            ctx,
            id,
            total,
            limit_per_user,
            price,
            admin,
            token,
            start_time,
            end_time,
        )
    }
}
