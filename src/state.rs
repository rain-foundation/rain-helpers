use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub struct Pool {
    pub bump: u8,
    pub owner: Pubkey,
    pub currency: Pubkey,

    pub padding: [u8; 64],
    pub padding1: [u8; 3],

    pub total_amount: u64,
    pub borrowed_amount: u64,
    pub available_amount: u64,
    pub usable_amount: u64,
}

impl anchor_lang::Discriminator for Pool {
    const DISCRIMINATOR: [u8; 8] = [241, 154, 109, 4, 17, 177, 109, 188];
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum LoanKind {
    Loan,
    Mortgage,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum LoanStatus {
    Ongoing,
    Repaid,
    Liquidated,
    Sold,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub struct Loan {
    pub kind: LoanKind,     // 1 byte, offset: 0
    pub status: LoanStatus, // 1 byte, offset: 1

    pub borrower: Pubkey, // 32 bytes, offset: 2
    pub lender: Pubkey,   // 32 bytes, offset: 34
    pub pool: Pubkey,     // 32 bytes, offset: 66
    pub mint: Pubkey,     // 32 bytes, offset: 98
    pub currency: Pubkey, // 32 bytes, offset: 130

    pub is_custom: bool, // 1 byte, offset: 162
    pub is_frozen: bool, // 1 byte, offset: 163

    pub price: u64,       // 8 bytes, offset: 164
    pub interest: u64,    // 8 bytes, offset: 172
    pub amount: u64,      // 8 bytes, offset: 180
    pub duration: u64,    // 8 bytes, offset: 188
    pub collection: u32,  // 4 bytes, offset: 196
    pub liquidation: u16, // 2 bytes, offset: 200

    pub padding: [u8; 20],
    pub padding1: [u8; 20],
    pub padding3: [u8; 2],

    pub created_at: u64,    // 8 bytes, offset: 244
    pub expired_at: u64,    // 8 bytes, offset: 252
    pub repaid_at: u64,     // 8 bytes, offset: 260
    pub sold_at: u64,       // 8 bytes, offset: 268
    pub liquidated_at: u64, // 8 bytes, offset: 27
}

impl anchor_lang::Discriminator for Loan {
    const DISCRIMINATOR: [u8; 8] = [20, 195, 70, 117, 165, 227, 182, 1];
}