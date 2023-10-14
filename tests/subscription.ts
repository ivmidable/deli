import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Subscription, IDL } from "../target/types/subscription";
import { startAnchor, ProgramTestContext } from "solana-bankrun";
import { BankrunProvider } from "anchor-bankrun";
import { createMint, getOrCreateAssociatedTokenAccount, mintTo } from "@solana/spl-token";
import { BN } from "bn.js";

describe("subscription", function() {
  let authority = anchor.web3.Keypair.generate();
  let user = anchor.web3.Keypair.generate();
  //var authority = anchor.web3.Keypair.generate();
  let mint = anchor.web3.PublicKey.default;
  let program: Program<Subscription>;
  let programId = new anchor.web3.PublicKey(
    "EgYj5qqk1Kbq3eSwbUYwyDWQ6j3tnXkTej4MJJWkzqSQ"
  );
  const provider = anchor.AnchorProvider.env();

  before("setup", async function() {
    // Configure the client to use the local cluster.
    //const context = await startAnchor("./", [], []);
    //const provider = new BankrunProvider(context);


    //anchor.setProvider(provider);
    anchor.setProvider(provider);

    program = new anchor.Program<Subscription>(
      IDL,
      programId,
      anchor.getProvider()
    );
    //authority = context.payer;


  });

  it("Airdrop", async function() {
    await Promise.all(
      [authority, user].map(async (k) => {
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
      authority, // Transaction payer
      authority.publicKey, // Mint authority, this is the account who is allowed to mint a token
      null, // Freeze authority
      9 // Token decimals, ie: smallest token unit = 0.00000001
    );

    // Create new token account for the user
    let user_ata = await getOrCreateAssociatedTokenAccount(provider.connection, user, mint, user.publicKey);
    let res = await mintTo(provider.connection, authority, mint, user_ata.address, authority, 100000);
    console.log("minted to user account: Txid: ", res);
  });

  it("Add a Product!", async function() {
    // Add your test here.
    let pdas = getProductPdas(
      program.programId,
      authority.publicKey,
      "test",
      "test"
    );

    const tx = await program.methods
      .addProduct("test")
      .accounts({
        authority: authority.publicKey,
        product: pdas.product,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([authority])
      .rpc();

    console.log("Your transaction signature", tx);
  });

  it("Add a Tier to the newly crerated Product.", async function() {
    let pdas = getProductPdas(
      program.programId,
      authority.publicKey,
      "test",
      "test"
    );

    await program.methods
      .addTier(
        "test",
        new anchor.BN(5000),
        new anchor.BN(5000),
        new anchor.BN(5000)
      )
      .accounts({
        authority: authority.publicKey,
        product: pdas.product,
        tier: pdas.tier,
        mint,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      }).signers([authority])
      .rpc();
  });

  it("Delegate to program so the user can use Subscriptions", async function() {
    let auth = getAuthPda(program.programId, user.publicKey, mint);
    let user_ata = await getOrCreateAssociatedTokenAccount(provider.connection, user, mint, user.publicKey);

    await program.methods.delegate().accounts({
      user: user.publicKey,
      auth: auth,
      mint,
      userAta: user_ata.address,
      associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
      tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      systemProgram: anchor.web3.SystemProgram.programId,
    }).signers([user]).rpc();

  });

  xit("Purchase a Subscription", async function() {
    let pdas = getProductPdas(
      program.programId,
      authority.publicKey,
      "test",
      "test"
    );

    let subscriptionPDA = getSubscriptionPDA(
      program,
      user.publicKey,
      pdas.product,
      pdas.tier
    );

    let user_ata = await getOrCreateAssociatedTokenAccount(provider.connection, user, mint, user.publicKey);

    await program.methods.subscribe().accounts({
      user: user.publicKey,
      userAta: user_ata.address,
      mint,
      product: pdas.product,
      tier: pdas.tier,
      subscription: subscriptionPDA,
      associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
      tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      systemProgram: anchor.web3.SystemProgram.programId,
    }).signers([user]).rpc();
  });

  xit("Cancel a Subscription", async function() { });
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

function getProductPdas(
  programId: anchor.web3.PublicKey,
  productOwner: anchor.web3.PublicKey,
  productName: String,
  tierName: String
) {
  let product = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("product"), productOwner.toBuffer(), Buffer.from(productName)],
    programId
  )[0];

  let tier = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("tier"), product.toBuffer(), Buffer.from(tierName)],
    programId
  )[0];

  return { product, tier };
}

function getAuthPda(programId: anchor.web3.PublicKey, user: anchor.web3.PublicKey, mint: anchor.web3.PublicKey) {
  let auth = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("auth"), user.toBuffer(), mint.toBuffer()],
    programId
  )[0];
  return auth;
}

function getSubscriptionPDA(
  program: anchor.Program<Subscription>,
  user: anchor.web3.PublicKey,
  product: anchor.web3.PublicKey,
  tier: anchor.web3.PublicKey
) {
  return anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("subscription"),
      product.toBuffer(),
      tier.toBuffer(),
      user.toBuffer(),
    ],
    program.programId
  )[0];
}
