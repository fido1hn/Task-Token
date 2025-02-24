import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import {
  LAMPORTS_PER_SOL,
  PublicKey,
  SendTransactionError,
  SystemProgram,
} from "@solana/web3.js";
import {
  createMint,
  getOrCreateAssociatedTokenAccount,
} from "@solana/spl-token";
import { TaskToken } from "../target/types/task_token";

describe("task-token", () => {
  // Configure the client to use the local cluster.
  const provider = new anchor.AnchorProvider(
    new anchor.web3.Connection("http://127.0.0.1:8899", "confirmed"),
    anchor.AnchorProvider.env().wallet
  );
  anchor.setProvider(provider);
  const program = anchor.workspace.TaskToken as Program<TaskToken>;
  const connection = provider.connection;

  // Protocol admin
  const admin = anchor.web3.Keypair.generate();

  // Config PDA
  const [configPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("config"), admin.publicKey.toBuffer()],
    program.programId
  );
  // Config Vault
  const [vaultPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("config"), configPda.toBuffer()],
    program.programId
  );
  // Task Token Mint
  const [taskTokenMint] = PublicKey.findProgramAddressSync(
    [Buffer.from("task_token"), configPda.toBuffer()],
    program.programId
  );
  // Payment Mint:
  let paymentMint: PublicKey;

  // Token program
  const tokenProgram = anchor.utils.token.TOKEN_PROGRAM_ID;
  // System program
  const systemProgram = SystemProgram.programId;

  before("Prepare test environment", async () => {
    try {
      const txSig = await provider.connection.requestAirdrop(
        admin.publicKey,
        10 * LAMPORTS_PER_SOL
      );

      const latestBlockHash = await connection.getLatestBlockhash();

      const _ = await connection.confirmTransaction({
        blockhash: latestBlockHash.blockhash,
        lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
        signature: txSig,
      });

      console.log(
        `Success! Check out your TX here: https://explorer.solana.com/tx/${txSig}?cluster=Localnet`
      );

      paymentMint = await createMint(
        provider.connection,
        admin,
        admin.publicKey,
        null,
        6
      );
    } catch (e) {
      console.error(`Oops, something went wrong: ${e}`);
    }
  });

  it("Is initialized!", async () => {
    // Add your test here.
    try {
      const tx = await program.methods
        .initialize(150)
        .accountsPartial({
          config: configPda,
          admin: admin.publicKey,
          paymentMint,
          vault: vaultPda,
          taskTokenMint,
          tokenProgram,
          systemProgram,
        })
        .signers([admin])
        .rpc();
      console.log("Your transaction signature", tx);
    } catch (error) {
      console.log(`an error occured: ${error}`);
    }
  });
});
