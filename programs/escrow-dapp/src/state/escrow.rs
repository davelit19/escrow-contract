use anchor_lang::prelude::*;

pub const DISCRIMINATOR: usize = 8;
pub const SEED: &[u8] = b"escrow";

#[account]
#[derive(InitSpace)]
pub struct EscrowAccount {
    pub sender_pubkey: Pubkey,
    pub receiver_pubkey: Pubkey,
    pub sender_mint: Pubkey, // Sender's token account
    pub receiver_mint: Pubkey, // Receiver's token account
    pub sender_expected_amount: u64,
    pub receiver_expected_amount: u64,
    pub pda_bump: u8
   
}


