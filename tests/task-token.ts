import * as anchor from "@coral-xyz/anchor";
import { Program, BN } from "@coral-xyz/anchor";
import { LAMPORTS_PER_SOL, PublicKey, SystemProgram } from "@solana/web3.js";
import {
  Account,
  createMint,
  getOrCreateAssociatedTokenAccount,
  mintTo,
  ASSOCIATED_TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import { TaskToken } from "../target/types/task_token";
import { expect } from "chai";

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

  // taskTwoOwner keypair
  const taskTwoOwner = anchor.web3.Keypair.generate();

  // taskOwner Payment ATA
  let taskOwnerAta: Account;

  // taskOwner Payment ATA
  let taskTwoOwnerAta: Account;

  // developer keypair
  const developer = anchor.web3.Keypair.generate();

  // developer keypair
  const developerTwo = anchor.web3.Keypair.generate();

  // developer Payment ATA
  let developerPaymentAta: Account;

  // developer Task Token ATA
  let developerTaskTokenAta: Account;

  // developer Payment ATA
  let developerTwoPaymentAta: Account;

  // developer Task Token ATA
  let developerTwoTaskTokenAta: Account;

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
  let taskOneVault: Account;
  let taskOneSubmissionPda: PublicKey;

  // Token program
  const tokenProgram = anchor.utils.token.TOKEN_PROGRAM_ID;
  // System program
  const systemProgram = SystemProgram.programId;

  before("Prepare test environment", async () => {
    try {
      // Airdrop sol to admin
      const txSig = await provider.connection.requestAirdrop(
        admin.publicKey,
        100 * LAMPORTS_PER_SOL
      );
      const latestBlockHash = await connection.getLatestBlockhash();
      const tx = await connection.confirmTransaction({
        blockhash: latestBlockHash.blockhash,
        lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
        signature: txSig,
      });

      // Airdrop sol to taskOwner
      const txSig2 = await provider.connection.requestAirdrop(
        taskOwner.publicKey,
        100 * LAMPORTS_PER_SOL
      );
      const latestBlockHash2 = await connection.getLatestBlockhash();
      const tx2 = await connection.confirmTransaction({
        blockhash: latestBlockHash2.blockhash,
        lastValidBlockHeight: latestBlockHash2.lastValidBlockHeight,
        signature: txSig2,
      });

      // Airdrop sol to developer
      const txSig3 = await provider.connection.requestAirdrop(
        developer.publicKey,
        100 * LAMPORTS_PER_SOL
      );
      const latestBlockHash3 = await connection.getLatestBlockhash();
      const tx3 = await connection.confirmTransaction({
        blockhash: latestBlockHash3.blockhash,
        lastValidBlockHeight: latestBlockHash3.lastValidBlockHeight,
        signature: txSig3,
      });

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

      // Airdrop some payment mint tokens to the taskOwnerAta
      const tx2Sig = await mintTo(
        provider.connection,
        admin,
        paymentMint,
        taskOwnerAta.address,
        admin,
        100_000_000
      );
    } catch (e) {
      console.error(`Oops, something went wrong: ${e}`);
    }
  });

  // Happy path - initilaze smart contract
  it("Admin can initialize smart contract!", async () => {
    // Admin can create a task token contract
    try {
      const initializeInstruction = await program.methods
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
        .instruction();

      const blockhash = await connection.getLatestBlockhash();

      const tx = new anchor.web3.Transaction({
        feePayer: admin.publicKey,
        blockhash: blockhash.blockhash,
        lastValidBlockHeight: blockhash.lastValidBlockHeight,
      }).add(initializeInstruction);

      const txSig = await anchor.web3.sendAndConfirmTransaction(
        connection,
        tx,
        [admin]
      );

      const configAccount = await program.account.config.fetch(configPda);
      expect(configAccount.admin.toString()).equal(admin.publicKey.toString());
    } catch (error) {
      console.log(`an error occured: ${error}`);
    }
  });

  // Unhappy path - Cannot reinitialize already initialized config
  it("Fails when trying to initialize an already initialized config", async () => {
    try {
      // Create the initialization instruction using accountsPartial
      const initializeInstruction = await program.methods
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
        .instruction();

      // Get the latest blockhash to build the transaction
      const { blockhash, lastValidBlockHeight } =
        await connection.getLatestBlockhash();

      // Build the transaction with the instruction
      const tx = new anchor.web3.Transaction({
        feePayer: admin.publicKey,
        blockhash,
        lastValidBlockHeight,
      }).add(initializeInstruction);

      // Attempt to send and confirm the transaction
      const txSig = await anchor.web3.sendAndConfirmTransaction(
        connection,
        tx,
        [admin]
      );
      expect.fail(
        "The instruction should have failed because the config account already exists."
      );
    } catch (error) {
      // Check that the error indicates the account already exists or a related issue.
      expect(error.message).to.include("Transaction simulation failed");
    }
  });

  // Happy path - create task
  it("Task owner can create a task!", async () => {
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
      const createTaskInstruction = await program.methods
        .createTask(title, description, pay, deadline, difficulty)
        .accountsPartial({
          config: configPda,
          task: taskOnePda,
          owner: taskOwner.publicKey,
          systemProgram: systemProgram,
          configVault: vaultPda,
        })
        .signers([taskOwner])
        .instruction();

      const blockhash = await connection.getLatestBlockhash();
      const tx = new anchor.web3.Transaction({
        feePayer: taskOwner.publicKey,
        blockhash: blockhash.blockhash,
        lastValidBlockHeight: blockhash.lastValidBlockHeight,
      }).add(createTaskInstruction);

      const txSig = await anchor.web3.sendAndConfirmTransaction(
        connection,
        tx,
        [taskOwner]
      );

      const taskOneAccount = await program.account.task.fetch(taskOnePda);
      expect(taskOneAccount.description).equal(description);
    } catch (error) {
      console.log(`an error occured: ${error}`);
    }
  });

  // Happy path - create task
  it("Task owner can create a task vault!", async () => {
    try {
      const createTaskVaultInstruction = await program.methods
        .createTaskVault()
        .accountsPartial({
          config: configPda,
          task: taskOnePda,
          paymentMint,
          signerPaymentMintAta: taskOwnerAta.address,
          signer: taskOwner.publicKey,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          tokenProgram,
          systemProgram: systemProgram,
        })
        .signers([taskOwner])
        .instruction();

      const blockhash = await connection.getLatestBlockhash();
      const tx = new anchor.web3.Transaction({
        feePayer: taskOwner.publicKey,
        blockhash: blockhash.blockhash,
        lastValidBlockHeight: blockhash.lastValidBlockHeight,
      }).add(createTaskVaultInstruction);

      const txSig = await anchor.web3.sendAndConfirmTransaction(
        connection,
        tx,
        [taskOwner]
      );

      taskOneVault = await getOrCreateAssociatedTokenAccount(
        connection,
        taskOwner,
        paymentMint,
        taskOnePda,
        true
      );

      // Assert that the account exists
      expect(Number(taskOneVault.amount)).greaterThan(10_000_000);
    } catch (error) {
      console.log(`an error occured: ${error}`);
    }
  });

  // Happy path - submit task
  it("Developer can submit a task!", async () => {
    // task submission PDA for task one
    [taskOneSubmissionPda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("submission"),
        developer.publicKey.toBuffer(),
        taskOnePda.toBuffer(),
      ],
      program.programId
    );

    try {
      let commit_url = "http://changes-made-to-readme.git";
      const submissionInstruction = await program.methods
        .submitTask(commit_url)
        .accountsPartial({
          submission: taskOneSubmissionPda,
          task: taskOnePda,
          systemProgram: systemProgram,
          signer: developer.publicKey,
        })
        .signers([developer])
        .instruction();

      const blockhash = await connection.getLatestBlockhash();
      const tx = new anchor.web3.Transaction({
        feePayer: developer.publicKey,
        blockhash: blockhash.blockhash,
        lastValidBlockHeight: blockhash.lastValidBlockHeight,
      }).add(submissionInstruction);

      const txSig = await anchor.web3.sendAndConfirmTransaction(
        connection,
        tx,
        [developer]
      );

      const submissionAccount = await program.account.submission.fetch(
        taskOneSubmissionPda
      );

      expect(submissionAccount.submissionLink).equal(commit_url);
    } catch (error) {
      console.log(`an error occured: ${error}`);
    }
  });

  // Happy path - Task Owner can pay developer
  it("Task owner can pay developer", async () => {
    try {
      const payDeveloperInstruction = await program.methods
        .payDeveloper()
        .accountsPartial({
          signer: taskOwner.publicKey,
          config: configPda,
          submission: taskOneSubmissionPda,
          task: taskOnePda,
          taskVault: taskOneVault.address,
          taskTokenMint,
          paymentMint,
          developer: developer.publicKey,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          tokenProgram,
          systemProgram,
        })
        .signers([taskOwner])
        .instruction();

      const blockhash = await connection.getLatestBlockhash();
      const tx = new anchor.web3.Transaction({
        feePayer: taskOwner.publicKey,
        blockhash: blockhash.blockhash,
        lastValidBlockHeight: blockhash.lastValidBlockHeight,
      }).add(payDeveloperInstruction);

      const txSig = await anchor.web3.sendAndConfirmTransaction(
        connection,
        tx,
        [taskOwner]
      );

      developerPaymentAta = await getOrCreateAssociatedTokenAccount(
        provider.connection,
        developer,
        paymentMint,
        developer.publicKey
      );

      developerTaskTokenAta = await getOrCreateAssociatedTokenAccount(
        provider.connection,
        developer,
        taskTokenMint,
        developer.publicKey
      );

      expect(Number(developerPaymentAta.amount)).greaterThan(10_000_000);
      expect(Number(developerTaskTokenAta.amount)).equal(1_000_000);
    } catch (error) {
      console.log(`an error occured: ${error}`);
    }
  });

  // Happy path - Task owner can close task account & task vault
  it("Task owner can close task account and task vault", async () => {
    try {
      const closeTaskAccountVaultInstruction = await program.methods
        .closeTaskAccountVault()
        .accountsPartial({
          signer: taskOwner.publicKey,
          task: taskOnePda,
          taskVault: taskOneVault.address,
          tokenProgram,
        })
        .signers([taskOwner])
        .instruction();

      const blockhash = await connection.getLatestBlockhash();
      const tx = new anchor.web3.Transaction({
        feePayer: taskOwner.publicKey,
        blockhash: blockhash.blockhash,
        lastValidBlockHeight: blockhash.lastValidBlockHeight,
      }).add(closeTaskAccountVaultInstruction);

      const txSig = await anchor.web3.sendAndConfirmTransaction(
        connection,
        tx,
        [taskOwner]
      );

      // Check if the task account is closed
      try {
        await program.account.task.fetch(taskOnePda);
        expect.fail("Task account should have been closed");
      } catch (error: any) {
        expect(error.message).to.include("Account does not exist");
      }
    } catch (error) {
      console.log("An error occured", error);
    }
  });

  // Happy path - Developer can close submission account
  it("Developer can close submission account", async () => {
    try {
      const closeSubmissionInstruction = await program.methods
        .closeSubmission()
        .accountsPartial({
          signer: developer.publicKey,
          config: configPda,
          submission: taskOneSubmissionPda,
          developerTaskTokenAta: developerTaskTokenAta.address,
          taskTokenMint,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          tokenProgram,
          systemProgram,
        })
        .signers([developer])
        .instruction();

      const blockhash = await connection.getLatestBlockhash();
      const tx = new anchor.web3.Transaction({
        feePayer: developer.publicKey,
        blockhash: blockhash.blockhash,
        lastValidBlockHeight: blockhash.lastValidBlockHeight,
      }).add(closeSubmissionInstruction);

      const txSig = await anchor.web3.sendAndConfirmTransaction(
        connection,
        tx,
        [developer]
      );
    } catch (error) {
      console.log("an error occured", error);
    }
  });
});
