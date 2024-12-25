use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct Order {
    pub id: u64,
    pub admin: Pubkey,

    pub total: u64,
    pub limit_per_user: u64,

    pub price: u64,
    pub token: Pubkey,

    pub start_time: i64,
    pub end_time: i64,
}

impl Order {
    pub const LEN: usize = 8 + 8 + 32 + 8 + 8 + 8 + 32 + 8 + 8;
}

#[account]
#[derive(Default)]
pub struct Minted {
    pub minted: u64,
}

impl Minted {
    pub const LEN: usize = 8 + 8;
}
