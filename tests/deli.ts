import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Deli, IDL } from "../target/types/deli";
import { startAnchor, ProgramTestContext } from "solana-bankrun";
import { BankrunProvider } from "anchor-bankrun";
import { createMint, getOrCreateAssociatedTokenAccount, mintTo } from "@solana/spl-token";
import { BN } from "bn.js";

describe("deli", function() {
  let admin = anchor.web3.Keypair.generate();
  let user = anchor.web3.Keypair.generate();
  //var authority = anchor.web3.Keypair.generate();
  let mint = anchor.web3.PublicKey.default;
  let program: Program<Deli>;
  let programId = new anchor.web3.PublicKey(
    "7cr8PdkQzH1WoCk6gyrhHYJqXpGx3LZH7ttH1cxcD8MS"
  );
  const provider = anchor.AnchorProvider.env();

  before("setup", async function() {
    // Configure the client to use the local cluster.
    //const context = await startAnchor("./", [], []);
    //const provider = new BankrunProvider(context);


    //anchor.setProvider(provider);
    anchor.setProvider(provider);

    program = new anchor.Program<Deli>(
      IDL,
      programId,
      anchor.getProvider()
    );
    //authority = context.payer;


  });

  it("Airdrop", async function() {
    await Promise.all(
      [admin, user].map(async (k) => {
        return await anchor
          .getProvider()
          .connection.requestAirdrop(
            k.publicKey,
            1000 * anchor.web3.LAMPORTS_PER_SOL
          )
          .then(confirmTx);
      })
    );


  });

  it("Mint Tokens and send them to User", async function() {
    mint = await createMint(
      provider.connection, // Devnet connection
      admin, // Transaction payer
      admin.publicKey, // Mint authority, this is the account who is allowed to mint a token
      null, // Freeze authority
      9 // Token decimals, ie: smallest token unit = 0.00000001
    );

    // Create new token account for the user
    let user_ata = await getOrCreateAssociatedTokenAccount(provider.connection, user, mint, user.publicKey);
    let res = await mintTo(provider.connection, admin, mint, user_ata.address, admin, 100000000);
    console.log("minted to user account: Txid: ", res);
  });

  it("Add a Registry/Product!", async function() {
    // Add your test here.
    //
    let registry = getRegistry(
      program.programId,
      admin.publicKey,
      mint,
      new BN(0)
    );
    console.log(registry.toBase58());
    try {
      const tx = await program.methods
        .createInterval(new BN(0), new BN(5000000), new BN(50000))
        .accounts({
          registry,
          admin: admin.publicKey,
          mint,
          tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([admin])
        .rpc();

      console.log("Your transaction signature", tx);
    } catch (e) {
      console.log(e);
      throw "BOOM";
    }
  });

  xit("Delegate to program so the user can use Subscriptions", async function() {
    let auth = getAuth(program.programId, user.publicKey, mint);
    let user_ata = await getOrCreateAssociatedTokenAccount(provider.connection, user, mint, user.publicKey);
    let registry = getRegistry(
      program.programId,
      admin.publicKey,
      mint,
      new BN(0)
    );
    try {
      let tx = await program.methods.delegate(null).accounts({
        user: user.publicKey,
        auth: auth,
        mint,
        userAta: user_ata.address,
        associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
        registry,
      }).signers([user]).rpc();
      console.log("Your transaction signature", tx);
    } catch (e) {
      console.log(e);
      throw "BOOM";
    }

  });

  xit("Purchase a Subscription", async function() {
    let registry = getRegistry(
      program.programId,
      admin.publicKey,
      mint,
      new BN(0)
    );

    let subscription = getSubscription(
      program.programId,
      registry,
      user.publicKey,
    );

    let auth = getAuth(program.programId, user.publicKey, mint);

    let user_ata = await getOrCreateAssociatedTokenAccount(provider.connection, user, mint, user.publicKey);

    let tx = await program.methods.subscribe().accounts({
      user: user.publicKey,
      userAta: user_ata.address,
      mint,
      registry,
      subscription,
      auth,
      associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
      tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      systemProgram: anchor.web3.SystemProgram.programId,
    }).signers([user]).rpc();
    console.log("Your transaction signature", tx);
    let subs = await program.account.subscription.all();
    console.log(subs);
  });

  it("Delegate and Subscribe", async function() {
    let auth = getAuth(program.programId, user.publicKey, mint);
    let user_ata = await getOrCreateAssociatedTokenAccount(provider.connection, user, mint, user.publicKey);
    let registry = getRegistry(
      program.programId,
      admin.publicKey,
      mint,
      new BN(0)
    );
    let subscription = getSubscription(
      program.programId,
      registry,
      user.publicKey,
    );

    try {
      let ix = await program.methods.delegate(null).accounts({
        user: user.publicKey,
        auth: auth,
        mint,
        userAta: user_ata.address,
        associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
        registry,
      }).signers([user]).instruction();

      let tx = await program.methods.subscribe().accounts({
        user: user.publicKey,
        userAta: user_ata.address,
        mint,
        registry,
        subscription,
        auth,
        associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
      }).signers([user]).preInstructions([ix]).rpc();
      console.log("Your transaction signature", tx);
    } catch (e) {
      console.log(e);
      throw "BOOM";
    }

  });




  xit("Collect a Subscriptipn", async function() {
    let registry = getRegistry(
      program.programId,
      admin.publicKey,
      mint,
      new BN(0)
    );

    let subscription = getSubscription(
      program.programId,
      registry,
      user.publicKey,
    );

    let auth = getAuth(program.programId, user.publicKey, mint);

    let user_ata = await getOrCreateAssociatedTokenAccount(provider.connection, user, mint, user.publicKey);

    let admin_ata = await getOrCreateAssociatedTokenAccount(provider.connection, admin, mint, admin.publicKey);

    let tx = await program.methods.collect().accounts({
      owner: admin.publicKey,
      ownerAta: admin_ata.address,
      userAta: user_ata.address,
      mint,
      registry,
      subscription,
      auth,
      associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
      tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      systemProgram: anchor.web3.SystemProgram.programId,
    }).signers([admin]).rpc();

    console.log("Your transaction signature", tx);
  });

  it("Cancel a Subscription", async function() {
    let registry = getRegistry(
      program.programId,
      admin.publicKey,
      mint,
      new BN(0)
    );

    let subscription = getSubscription(
      program.programId,
      registry,
      user.publicKey,
    );

    let tx = await program.methods.unsubscribe().accounts({
      user: user.publicKey,
      registry,
      subscription,
      systemProgram: anchor.web3.SystemProgram.programId,
    }).signers([user]).rpc();
    console.log("Your transaction signature", tx);
  });
});

const confirmTx = async (signature: string): Promise<string> => {
  const latestBlockhash = await anchor
    .getProvider()
    .connection.getLatestBlockhash();
  await anchor.getProvider().connection.confirmTransaction(
    {
      signature,
      ...latestBlockhash,
    },
    "confirmed"
  );
  return signature;
};

function getRegistry(
  programId: anchor.web3.PublicKey,
  admin: anchor.web3.PublicKey,
  mint: anchor.web3.PublicKey,
  nonce: anchor.BN
) {
  let registry = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("registry"), admin.toBuffer(), mint.toBuffer(), nonce.toArrayLike(Buffer, "le", 8)],
    programId
  )[0];

  return registry;
}

function getAuth(programId: anchor.web3.PublicKey, user: anchor.web3.PublicKey, mint: anchor.web3.PublicKey) {
  let auth = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("auth"), user.toBuffer(), mint.toBuffer()],
    programId
  )[0];
  return auth;
}

function getSubscription(
  programId: anchor.web3.PublicKey,
  registry: anchor.web3.PublicKey,
  user: anchor.web3.PublicKey,
) {
  return anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("subscription"),
      registry.toBuffer(),
      user.toBuffer(),
    ],
    programId
  )[0];
}
