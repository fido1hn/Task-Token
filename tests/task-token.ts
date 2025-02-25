import * as anchor from "@coral-xyz/anchor";
import { Program, BN } from "@coral-xyz/anchor";
import {
  LAMPORTS_PER_SOL,
  PublicKey,
  SendTransactionError,
  SystemProgram,
} from "@solana/web3.js";
import {
  Account,
  createMint,
  getOrCreateAssociatedTokenAccount,
  mintTo,
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

  // taskOwner keypair
  const taskOwner = anchor.web3.Keypair.generate();

  // taskOwner Payment ATA
  let taskOwnerAta: Account;

  // developer keypair
  const developer = anchor.web3.Keypair.generate();

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
  let taskOnePda: PublicKey;
  let taskOneVault: PublicKey;

  // Token program
  const tokenProgram = anchor.utils.token.TOKEN_PROGRAM_ID;
  // System program
  const systemProgram = SystemProgram.programId;

  before("Prepare test environment", async () => {
    try {
      // Airdrop sol to admin
      const txSig = await provider.connection.requestAirdrop(
        admin.publicKey,
        10 * LAMPORTS_PER_SOL
      );

      const latestBlockHash = await connection.getLatestBlockhash();

      const tx = await connection.confirmTransaction({
        blockhash: latestBlockHash.blockhash,
        lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
        signature: txSig,
      });

      console.log(
        `Success! Check out your TX here: https://explorer.solana.com/tx/${txSig}?cluster=Localnet`
      );

      // Airdrop sol to taskOwner
      const txSig2 = await provider.connection.requestAirdrop(
        taskOwner.publicKey,
        10 * LAMPORTS_PER_SOL
      );

      const latestBlockHash2 = await connection.getLatestBlockhash();

      const tx2 = await connection.confirmTransaction({
        blockhash: latestBlockHash2.blockhash,
        lastValidBlockHeight: latestBlockHash2.lastValidBlockHeight,
        signature: txSig2,
      });

      console.log(
        `Success! Check out your TX here: https://explorer.solana.com/tx/${txSig2}?cluster=Localnet`
      );

      // Airdrop sol to developer
      const txSig3 = await provider.connection.requestAirdrop(
        developer.publicKey,
        10 * LAMPORTS_PER_SOL
      );

      const latestBlockHash3 = await connection.getLatestBlockhash();

      const tx3 = await connection.confirmTransaction({
        blockhash: latestBlockHash3.blockhash,
        lastValidBlockHeight: latestBlockHash3.lastValidBlockHeight,
        signature: txSig3,
      });

      console.log(
        `Success! Check out your TX here: https://explorer.solana.com/tx/${txSig2}?cluster=Localnet`
      );

      // Create protocol payment mint
      paymentMint = await createMint(
        provider.connection,
        admin,
        admin.publicKey,
        null,
        6
      );

      // Check the taskOwner has received payment mint tokens
      taskOwnerAta = await getOrCreateAssociatedTokenAccount(
        provider.connection,
        admin,
        paymentMint,
        taskOwner.publicKey
      );

      // Airdrop some payment mint tokens to the taskOwner
      const tx2Sig = await mintTo(
        provider.connection,
        admin,
        paymentMint,
        taskOwnerAta.address,
        admin,
        30_000_000
      );

      console.log(
        `Success! Check out your TX here: https://explorer.solana.com/tx/${tx2Sig}?cluster=Localnet`
      );

      let taskOwnerbalance = await connection.getTokenAccountBalance(
        taskOwnerAta.address
      );

      let developerBalance = await connection.getBalance(developer.publicKey);

      console.log(`TaskOwner balance is: ${taskOwnerbalance.value.uiAmount}`);
      console.log(
        `Developer Sol balance is: ${developerBalance / LAMPORTS_PER_SOL}`
      );
    } catch (e) {
      console.error(`Oops, something went wrong: ${e}`);
    }
  });

  // Happy path - initialize contract
  it("Is Contract initialized!", async () => {
    // Add your test here.
    // Admin can create a task token contract
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

  // Happy path - create task
  it("Can create a task!", async () => {
    // Add your test here.
    // task owner create a task
    [taskOnePda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("task"),
        Buffer.from("Task-1: Edit README"),
        taskOwner.publicKey.toBuffer(),
      ],
      program.programId
    );
    try {
      let title = "Task-1: Edit README";
      let description = "Correct the spelling mistake in the README";
      let pay = new BN(20_000_000);
      let deadline = new BN(1424832629);
      let difficulty = 0;
      const tx = await program.methods
        .createTask(title, description, pay, deadline, difficulty)
        .accountsPartial({
          config: configPda,
          task: taskOnePda,
          owner: taskOwner.publicKey,
          systemProgram: systemProgram,
          configVault: vaultPda,
        })
        .signers([taskOwner])
        .rpc();
      console.log("Your transaction signature", tx);
    } catch (error) {
      console.log(`an error occured: ${error}`);
    }
  });

  // Happy path - create task
  it("Can create a task vault!", async () => {
    // Add your test here.
    // task owner create a task
    [taskOneVault] = PublicKey.findProgramAddressSync(
      [Buffer.from("task_vault"), taskOnePda.toBuffer()],
      program.programId
    );
    try {
      const tx = await program.methods
        .createTaskVault()
        .accountsPartial({
          config: configPda,
          task: taskOnePda,
          paymentMint,
          signerPaymentMintAta: taskOwnerAta.address,
          signer: taskOwner.publicKey,
          taskVault: taskOneVault,
          tokenProgram,
          systemProgram: systemProgram,
        })
        .signers([taskOwner])
        .rpc();
      console.log("Your transaction signature", tx);
    } catch (error) {
      console.log(`an error occured: ${error}`);
    }
  });

  // Happy path - submit task
  it("Can submit a task!", async () => {
    // Add your test here.
    // task submission PDA for task one
    let [taskOneSubmissionPda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("submission"),
        developer.publicKey.toBuffer(),
        taskOnePda.toBuffer(),
      ],
      program.programId
    );
    try {
      let commit_url = "http://changes-made-to-readme.git";

      const tx = await program.methods
        .submitTask(commit_url)
        .accountsPartial({
          submission: taskOneSubmissionPda,
          task: taskOnePda,
          systemProgram: systemProgram,
          signer: developer.publicKey,
        })
        .signers([developer])
        .rpc();
      console.log("Your transaction signature", tx);
    } catch (error) {
      console.log(`an error occured: ${error}`);
    }
  });
});
