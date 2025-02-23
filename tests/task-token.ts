import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import {
  LAMPORTS_PER_SOL,
  PublicKey,
  SendTransactionError,
  SystemProgram,
} from "@solana/web3.js";
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

  // protocol admin
  const admin = anchor.web3.Keypair.generate();

  // Config
  const [configPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("config"), admin.publicKey.toBuffer()],
    program.programId
  );
  // Vault
  const [vaultPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("config"), configPda.toBuffer()],
    program.programId
  );
  // Mint
  const [mintPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("rewards"), configPda.toBuffer()],
    program.programId
  );
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

      const txhash = await connection.confirmTransaction({
        blockhash: latestBlockHash.blockhash,
        lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
        signature: txSig,
      });

      console.log(
        `Success! Check out your TX here: https://explorer.solana.com/tx/${txhash}?cluster=Localnet`
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
          admin: admin.publicKey,
          config: configPda,
          vault: vaultPda,
          taskTokenMint: mintPda,
          tokenProgram,
          systemProgram,
        })
        .signers([admin])
        .rpc();
      console.log("Your transaction signature", tx);
    } catch (error) {
      const logs = await new SendTransactionError(error).getLogs(
        provider.connection
      );
      console.log(`log: ${logs}`);
    }
    console.log(`commitment: ${connection.commitment}`);
    console.log(`cluster: ${connection.rpcEndpoint}`);
  });
});
