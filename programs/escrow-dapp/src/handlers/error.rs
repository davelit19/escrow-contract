use anchor_lang::prelude::*;

#[error_code]
pub enum EscrowError {
    #[msg("The amount expected by the receiver is unequal to amount sent by sender")]
    InvalidReceiverExpectedAmount,
    #[msg("The amount expected by the sender is unequal to amount sent by receiver")]
    InvalidSenderExpectedAmount,
 
}