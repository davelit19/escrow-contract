import * as anchor from "@coral-xyz/anchor";
import { LAMPORTS_PER_SOL, Keypair, Connection} from "@solana/web3.js";


export const senderAccount: Keypair = anchor.web3.Keypair.generate();
export const receiverAccount: Keypair = anchor.web3.Keypair.generate();

const connection = new Connection("http://127.0.0.1:8899", "finalized" );

(async () => {
    const sig1 = await connection.requestAirdrop(senderAccount.publicKey, 4*LAMPORTS_PER_SOL);
    const sig2 = await connection.requestAirdrop(receiverAccount.publicKey, 4*LAMPORTS_PER_SOL);

    await connection.confirmTransaction(sig1);
    await connection.confirmTransaction(sig2);
})()

