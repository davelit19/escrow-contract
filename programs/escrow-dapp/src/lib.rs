pub mod handlers;
pub mod state;

use handlers::{accounts::*, error::*, functions::*};
pub use state::escrow::SEED;

use anchor_lang::prelude::*;

declare_id!("F8BdAaZvFJJ3FSMWwHNrfPGBAPRi8r5ttx1KwGisHkXJ");

#[program]

mod escrow_program {

    use super::*;

    pub fn sender_escrow_actions(
        ctx: Context<SenderEscrowAccount>,
        sender_expected_amount: u64,
        amount_to_send: u64,
        pda_bump: u8,
    ) -> Result<()> {
        let escrow = &mut ctx.accounts.escrow;
        escrow.sender_pubkey = ctx.accounts.payer.key();
        escrow.sender_mint = ctx.accounts.sender_mint.key();
        escrow.sender_expected_amount = sender_expected_amount;
        escrow.pda_bump = pda_bump;

        msg!("Initiating sender deposit to vault");
        deposit_to_vault(
            &ctx.accounts.sender_token_account,
            &ctx.accounts.sender_mint,
            &ctx.accounts.escrow_sender_vault,
            &ctx.accounts.payer,
            &ctx.accounts.token_program,
            amount_to_send,
        )?;

        msg!("Token succesfully transferred to vault");
        Ok(())
    }

    pub fn receiver_escrow_actions(
        ctx: Context<ReceiverEscrowAccount>,
        receiver_expected_amount: u64,
        amount_to_send: u64,
    
    ) -> Result<()> {
        let escrow = &mut ctx.accounts.escrow;
        escrow.receiver_pubkey = ctx.accounts.payer.key();
        escrow.receiver_mint = ctx.accounts.receiver_mint.key();
        escrow.receiver_expected_amount = receiver_expected_amount;

        msg!("Initiating receiver deposit to vault");
        deposit_to_vault(
            &ctx.accounts.receiver_token_account,
            &ctx.accounts.receiver_mint,
            &ctx.accounts.escrow_receiver_vault,
            &ctx.accounts.payer,
            &ctx.accounts.token_program,
            amount_to_send,
        )?;

        msg!("Token succesfully transferred to vault");
        Ok(())
    }

    pub fn execute_exchange(ctx: Context<ExecuteExchange>) -> Result<()> {
        let escrow = &ctx.accounts.escrow;
        let escrow_sender_vault = &ctx.accounts.escrow_sender_vault;
        let escrow_receiver_vault = &ctx.accounts.escrow_receiver_vault;
        let receiver_exchange_ata = &ctx.accounts.receiver_exchange_ata;
        let sender_exchange_ata = &ctx.accounts.sender_exchange_ata;
        let receiver_mint = &ctx.accounts.receiver_mint;
        let sender_mint = &ctx.accounts.sender_mint;
        let token_program = &ctx.accounts.token_program;
        let receiver_signer = &ctx.accounts.receiver_signer;
        let sender_signer = &ctx.accounts.sender_signer;

        if escrow_sender_vault.amount != escrow.receiver_expected_amount {
            return err!(EscrowError::InvalidReceiverExpectedAmount);
        }

        if escrow_receiver_vault.amount != escrow.sender_expected_amount {
            return err!(EscrowError::InvalidSenderExpectedAmount);
        }

        msg!("Transfering sender expected token");

      //  let id_slice = escrow.id.to_le_bytes();
        let bump_slice = [escrow.pda_bump];

        let pda_signer_seeds = &[&[
            SEED,
          //  id_slice.as_ref(),
            escrow.sender_pubkey.as_ref(),
            &bump_slice,
        ][..]];

        escrow_swap(
            escrow_receiver_vault,
            receiver_mint,
            sender_exchange_ata,
            escrow,
            token_program,
            pda_signer_seeds,
            escrow.sender_expected_amount,
        )?;
        msg!("Sender token successfully transferred");

        msg!("Transfering receiver expected token");
        escrow_swap(
            escrow_sender_vault,
            sender_mint,
            receiver_exchange_ata,
            escrow,
            token_program,
            pda_signer_seeds,
            escrow.receiver_expected_amount,
        )?;

        msg!("Sender token successfully transferred");

        msg!("Closing both vaults account");

        close_vaults_account(
            escrow_sender_vault,
            sender_signer,
            escrow,
            token_program,
            pda_signer_seeds,
        )?;
        close_vaults_account(
            escrow_receiver_vault,
            receiver_signer,
            escrow,
            token_program,
            pda_signer_seeds,
        )?;
        msg!("Successfully closed vaults account");
        Ok(())
    }

    pub fn close_data_account(_ctx: Context<CloseEscrowAccount>) -> Result<()> {
        Ok(())
    }
}
