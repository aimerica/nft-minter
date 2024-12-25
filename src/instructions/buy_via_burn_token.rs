use crate::{
    constants::{MINTED_SEED, ORDER_SEED, USER_MINTED_SEED, ZERO_PUBKEY},
    errors::MintError,
    state::{Minted, Order},
};

use std::str::FromStr;
use {
    anchor_lang::prelude::*,
    anchor_spl::{
        associated_token::AssociatedToken,
        token::{burn, Mint, Token, TokenAccount, Burn},
    },
};

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct BuyViaBurnToken<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub token_account: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = token_account,
        associated_token::authority = payer,
    )]
    pub sender_ata: Box<Account<'info, TokenAccount>>,

    #[account(
        seeds = [
            ORDER_SEED.as_bytes(),
            id.to_le_bytes().as_ref(),
        ],
        bump,
    )]
    pub order: Account<'info, Order>,

    #[account(
        mut,
        seeds = [
            MINTED_SEED.as_bytes(),
            id.to_le_bytes().as_ref(),
            ],
        bump,
    )]
    pub minted: Account<'info, Minted>,

    #[account(
        init_if_needed,
        payer = payer,
        space = Minted::LEN,
        seeds = [
            USER_MINTED_SEED.as_bytes(),
            id.to_le_bytes().as_ref(),
            payer.key.as_ref(),
            ],
        bump,
    )]
    pub user_minted: Account<'info, Minted>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub clock: Sysvar<'info, Clock>,
}

pub fn buy_via_burn_token(ctx: Context<BuyViaBurnToken>, id: u64, amount: u64) -> Result<()> {
    msg!("Minting token: {:?}, {:?} ", id, amount);

    let from_account = &ctx.accounts.payer;
    let order = &ctx.accounts.order;
    let clock = &ctx.accounts.clock;
    // Get the current timestamp
    let timestamp: i64 = clock.unix_timestamp;
    if timestamp < order.start_time || timestamp > order.end_time {
        return err!(MintError::InvalidTime);
    }
    let zero_key = Pubkey::from_str(ZERO_PUBKEY).unwrap();
    if order.token == zero_key {
        return err!(MintError::InvalidToken);
    }
    let minted = &mut ctx.accounts.minted;
    let user_minted = &mut ctx.accounts.user_minted;
    if minted.minted + amount > order.total {
        return err!(MintError::InvalidAmount);
    }
    minted.minted += amount;

    if user_minted.minted + amount > order.limit_per_user {
        return err!(MintError::InvalidUserLimit);
    }
    user_minted.minted += amount;

    let total_price = amount * order.price;

    burn(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Burn {
                mint: ctx.accounts.token_account.to_account_info(),
                from: ctx.accounts.sender_ata.to_account_info(),
                authority: ctx.accounts.payer.to_account_info(),
            },
        ),
        total_price, // Transfer amount, adjust for decimals
    )?;

    emit!(BuyViaTokenEvent {
        id,
        amount,
        price: order.price,
        payer: *from_account.key,
    });

    Ok(())
}

#[event]
pub struct BuyViaTokenEvent {
    pub id: u64,
    pub amount: u64,
    pub price: u64,
    pub payer: Pubkey,
}
