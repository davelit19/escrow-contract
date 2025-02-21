use anchor_lang::prelude::*;
use anchor_spl::token::{transfer_checked, close_account, CloseAccount, Mint, Token, TokenAccount, TransferChecked};

use crate::state::EscrowAccount;

pub fn deposit_to_vault<'info>(
    from: &Account<'info, TokenAccount>,
    mint: &Account<'info, Mint>,
    to: &Account<'info, TokenAccount>,
    authority: &Signer<'info>,
    token_program: &Program<'info, Token>,
    amount: u64,
) -> Result<()> {
    let cpi_accounts = TransferChecked {
        from: from.to_account_info(),
        mint: mint.to_account_info(),
        to: to.to_account_info(),
        authority: authority.to_account_info(),
    };

    transfer_checked(
        CpiContext::new(token_program.to_account_info(), cpi_accounts),
        amount,
        mint.decimals,
    )
}

pub fn escrow_swap<'info>(
    from: &Account<'info, TokenAccount>,
    mint: &Account<'info, Mint>,
    to: &Account<'info, TokenAccount>,
    authority: &Account<'info, EscrowAccount>,
    token_program: &Program<'info, Token>,
    signer_seeds: &[&[&[u8]]],
    amount: u64,
) -> Result<()> {
    let cpi_accounts = TransferChecked {
        from: from.to_account_info(),
        mint: mint.to_account_info(),
        to: to.to_account_info(),
        authority: authority.to_account_info(),
    };

    transfer_checked(
        CpiContext::new_with_signer(token_program.to_account_info(), cpi_accounts, signer_seeds),
        amount,
        mint.decimals,
    )
}

pub fn close_vaults_account<'info>(
    account: &Account<'info, TokenAccount>,
    destination: &Signer<'info>,
    authority: &Account<'info, EscrowAccount>,
    program: &Program<'info, Token>,
    signer_seeds: &[&[&[u8]]]
) -> Result<()> {

     let cpi_accounts = CloseAccount {
        account: account.to_account_info(),
        destination: destination.to_account_info(),
        authority: authority.to_account_info()
     };

     close_account(
        CpiContext::new_with_signer(program.to_account_info(), cpi_accounts, signer_seeds)
     )
    //Ok(())
}
