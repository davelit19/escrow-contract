use anchor_lang::prelude::*;
use anchor_spl::{
    token::{Token, Mint, TokenAccount},
    associated_token::AssociatedToken,
};

use std;

use crate::state::{EscrowAccount, SEED, DISCRIMINATOR};

#[derive(Accounts)]
pub struct SenderEscrowAccount<'info> {
    #[account(
        mut,
        mint::token_program = token_program // Check if the mint is a token program
    )]
    pub sender_mint: Account<'info, Mint>, // Initiializing the sender mint account
    #[account(
        mut,
        associated_token::mint = sender_mint,
        associated_token::authority = payer,
    )]
    pub sender_token_account: Account<'info, TokenAccount>, //Sender Associated token account
    #[account(
        init_if_needed,
        payer = payer,
        space = DISCRIMINATOR + EscrowAccount::INIT_SPACE,
        seeds = [SEED, payer.key().as_ref()],
        bump
    )]
    pub escrow: Account<'info, EscrowAccount>, //The escrow data account
    #[account(
        init,
        payer = payer,
      //  space = TokenAccount::LEN,
       // seeds = [b"vault", payer.key().as_ref()],
       // bump,
        associated_token::mint = sender_mint,
        associated_token::authority = escrow,
       
    )]
    pub escrow_sender_vault: Account<'info, TokenAccount>, // The esrow pda sender associated vault account
    #[account(mut)]
    pub payer: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ReceiverEscrowAccount<'info> {
    #[account(
        mut, 
        mint::token_program = token_program
    )]
    pub receiver_mint: Account<'info, Mint>,  //Receiver mint account
    #[account(
        mut,
       associated_token::mint = receiver_mint,
       associated_token::authority = payer,
    )]
    pub receiver_token_account: Account<'info, TokenAccount>, //Receiver Associated token account
    #[account(
        init,
        payer = payer,
        associated_token::mint = receiver_mint,
        associated_token::authority = escrow,
    )]
    pub escrow_receiver_vault: Account<'info, TokenAccount>, // The esrow pda receiver associated vault account
    #[account(mut, seeds = [SEED, escrow.sender_pubkey.as_ref()], bump = escrow.pda_bump)]
    pub escrow: Account<'info, EscrowAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>
}


#[derive(Accounts)] 
pub struct ExecuteExchange<'info> {
    #[account(mut, 
       // address = escrow.receiver_pubkey,
        mint::token_program = token_program
    )]
    pub receiver_mint: Account<'info, Mint>,
    #[account(
        mut,
       // address = escrow.sender_pubkey,
        mint::token_program = token_program
    )]
    pub sender_mint: Account<'info, Mint>,
    #[account(
        init,
        payer = sender_signer,
        associated_token::mint = receiver_mint,
        associated_token::authority = sender_signer,
    )]
    pub sender_exchange_ata: Account<'info, TokenAccount>, //Sender Token account for the collected token from receiver
    #[account(
        init,
        payer = receiver_signer,
        associated_token::mint = sender_mint,
        associated_token::authority = receiver_signer,
    )]
    pub receiver_exchange_ata: Account<'info, TokenAccount>, //Receiver Token account for the collected token from sender
    #[account(mut, 
        seeds = [SEED, escrow.sender_pubkey.as_ref()], 
        bump = escrow.pda_bump,
        
     )]
    pub escrow: Account<'info, EscrowAccount>,  //The escrow data account
    #[account(
        mut,
        associated_token::mint = sender_mint,
        associated_token::authority = escrow,
    )]
    pub escrow_sender_vault: Account<'info, TokenAccount>, //The escrow pda receiver associated vault account
    #[account(
        mut,
        associated_token::mint = receiver_mint,
        associated_token::authority = escrow,
    )]
    pub escrow_receiver_vault: Account<'info, TokenAccount>, // The esrow pda sender associated vault account
    #[account(mut)]
    pub sender_signer: Signer<'info>,
    #[account(mut)]
    pub receiver_signer: Signer<'info>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CloseEscrowAccount <'info> {
    #[account(
        mut,
        seeds = [SEED, escrow.sender_pubkey.as_ref()], 
        bump = escrow.pda_bump,
        has_one = sender_pubkey,
        close = sender_pubkey,
    )]
    pub escrow: Account<'info, EscrowAccount>,
    #[account(mut)]
    pub sender_pubkey: Signer<'info>,
    pub system_program: Program<'info, System>
}

