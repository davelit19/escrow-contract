import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { EscrowProgram } from "../target/types/escrow_program";
import BN from "bn.js";
import { assert, expect } from "chai";
import { createMint, getOrCreateAssociatedTokenAccount, mintTo, getAssociatedTokenAddress, Account, TOKEN_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { PublicKey } from "@solana/web3.js";
import { senderAccount, receiverAccount } from "./utils";

// Configure the client to use the local cluster.
anchor.setProvider(anchor.AnchorProvider.env());

const program = anchor.workspace.EscrowProgram as Program<EscrowProgram>;


//Deriving all the necessary accounts for both sender and receiver.
const test_accounts = async (
  account: anchor.web3.Keypair,
  escrow_pda,
): Promise<[anchor.web3.PublicKey, Account, PublicKey]> => {
  //Creating the mint account
  const mint_account = await createMint(
    program.provider.connection,
    account,
    account.publicKey,
    null,
    0,
  )
  //Getting the ata account
  const ata_account = await getOrCreateAssociatedTokenAccount(
    program.provider.connection,
    account,
    mint_account,
    account.publicKey,
  );


  const signature = await mintTo(
    program.provider.connection,
    account,
    mint_account,
    ata_account.address,
    account,
    100000,

  );
  await program.provider.connection.confirmTransaction(signature);

  // Deriving the sender vault pda
  const vault = await getAssociatedTokenAddress(
    mint_account,
    escrow_pda,
    true
  );

  return [mint_account, ata_account, vault];
}


describe("escrow-program", async function () {

  const [escrow_pda, escrow_bump] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("escrow"), senderAccount.publicKey.toBuffer()],
    program.programId
  )

  const sender_expected_amount = new BN(100);
  const sender_sent_amount = new BN(40);
  console.log(sender_expected_amount.toString())

  let sender_mint: anchor.web3.PublicKey;
  let sender_ata: Account;
  let escrow_sender_vault: anchor.web3.PublicKey;
  let receiver_mint: anchor.web3.PublicKey;
  let receiver_ata: Account;
  let escrow_receiver_vault: anchor.web3.PublicKey;

  it("Sender and Receiver are initializing the escrow account", async () => {

    [sender_mint, sender_ata, escrow_sender_vault] = await test_accounts(senderAccount, escrow_pda);
    [receiver_mint, receiver_ata, escrow_receiver_vault] = await test_accounts(receiverAccount, escrow_pda);


    try {
      const sender_tx = await program.methods.
        senderEscrowActions(sender_expected_amount, sender_sent_amount, escrow_bump)
        .accountsStrict({
          senderMint: sender_mint,
          senderTokenAccount: sender_ata.address,
          escrowSenderVault: escrow_sender_vault,
          payer: senderAccount.publicKey,
          escrow: escrow_pda,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          rent: anchor.web3.SYSVAR_RENT_PUBKEY,
          systemProgram: anchor.web3.SystemProgram.programId
        })
        .signers([senderAccount])
        .rpc();

      await program.provider.connection.confirmTransaction(sender_tx);
    } catch (error) {
      console.log("Solana error :", error);
    }

    try {
      const receiver_tx = await program.methods
        .receiverEscrowActions(sender_sent_amount, sender_expected_amount)
        .accountsStrict({
          receiverMint: receiver_mint,
          receiverTokenAccount: receiver_ata.address,
          escrowReceiverVault: escrow_receiver_vault,
          payer: receiverAccount.publicKey,
          escrow: escrow_pda,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          systemProgram: anchor.web3.SystemProgram.programId
        }).signers([receiverAccount])
        .rpc();

      await program.provider.connection.confirmTransaction(receiver_tx);
    } catch (error) {
      console.log("Solana error :", error);
    }

    const escrow_data_account = await program.account.escrowAccount.fetch(escrow_pda);
    const sender_vault_Token = await program.provider.connection.getTokenAccountBalance(escrow_sender_vault);
    const receiver_vault_Token = await program.provider.connection.getTokenAccountBalance(escrow_receiver_vault);

    console.log("sender_vault_Token:  -->", sender_vault_Token);
    console.log("escrow_data_account: -->", escrow_data_account);
    assert.equal(Number(escrow_data_account.receiverExpectedAmount), Number(sender_vault_Token.value.amount));
    assert.equal(Number(escrow_data_account.senderExpectedAmount), Number(receiver_vault_Token.value.amount));
  });

  

  it("Executing the escrow exchange", async () => {

    const receiver_exchange_ata = await getAssociatedTokenAddress(
      sender_mint,
      receiverAccount.publicKey
    );

    const sender_exchange_ata = await getAssociatedTokenAddress(
      receiver_mint,
      senderAccount.publicKey
    )



    const transaction = new anchor.web3.Transaction();
    const exchange_ix = await program.methods
      .executeExchange()
      .accountsStrict({
        senderMint: sender_mint,
        receiverMint: receiver_mint,
        senderSigner: senderAccount.publicKey,
        receiverSigner: receiverAccount.publicKey,
        senderExchangeAta: sender_exchange_ata,
        escrow: escrow_pda,
        receiverExchangeAta: receiver_exchange_ata,
        escrowSenderVault: escrow_sender_vault,
        escrowReceiverVault: escrow_receiver_vault,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId

      }).instruction()

    transaction.add(exchange_ix);

    const txHash = await anchor.web3.sendAndConfirmTransaction(program.provider.connection, transaction, [senderAccount, receiverAccount]);
    console.log(`https://explorer.solana.com/${txHash}?cluster=devnet`);


    const sender_x = await program.provider.connection.getTokenAccountBalance(sender_exchange_ata);
    const receiver_x = await program.provider.connection.getTokenAccountBalance(receiver_exchange_ata);

    const escrow_data_account = await program.account.escrowAccount.fetch(escrow_pda);
    assert.equal(Number(escrow_data_account.receiverExpectedAmount), Number(receiver_x.value.amount));
    assert.equal(Number(escrow_data_account.senderExpectedAmount), Number(sender_x.value.amount));

  })  

  it("Closing the data acccount", async () => {

    let escrowAccount = await program.account.escrowAccount.fetchNullable(escrow_pda);
    expect(escrowAccount).to.not.be.null;

    const pre_sender_balance = await program.provider.connection.getBalance(senderAccount.publicKey);

    try {
     const close_tx = await program.methods
       .closeDataAccount()
       .accountsStrict({
         escrow: escrow_pda,
         senderPubkey: senderAccount.publicKey,
         systemProgram: anchor.web3.SystemProgram.programId
       })
       .signers([senderAccount])
       .rpc()

       await program.provider.connection.confirmTransaction(close_tx);
      } catch (err) {
        console.log(err)
      }
      
      //The account no longer exist
      escrowAccount = await program.account.escrowAccount.fetchNullable(escrow_pda);
      expect(escrowAccount).to.be.null;

      const post_sender_balance = await program.provider.connection.getBalance(senderAccount.publicKey);
      expect(post_sender_balance).to.be.greaterThan(pre_sender_balance);
  })
});









