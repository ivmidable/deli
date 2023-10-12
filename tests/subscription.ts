import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Subscription } from "../target/types/subscription";
import { startAnchor, ProgramTestContext } from "solana-bankrun";
import { BankrunProvider } from "anchor-bankrun";

describe("subscription", () => {
  var authority = anchor.web3.Keypair.generate();
  var context: ProgramTestContext = undefined;
  var product = anchor.web3.PublicKey.default;
  var mint = anchor.web3.PublicKey.default;
  var program: anchor.Program<Subscription> = undefined;

  before("setup", async () => {
    // Configure the client to use the local cluster.
    context = await startAnchor("./", [], []);
    const provider = new BankrunProvider(context);
    anchor.setProvider(provider);
    program = anchor.workspace.Subscription as Program<Subscription>;
    authority = context.payer;

    product = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("product"),
        authority.publicKey.toBuffer(),
        Buffer.from("test"),
      ],
      program.programId
    )[0];
  });

  xit("Airdrop", async () => {
    await Promise.all(
      [authority].map(async (k) => {
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

  it("send coins from payer to authority", async () => {});

  it("Add a Product!", async () => {
    // Add your test here.
    const tx = await program.methods
      .addProduct("test")
      .accounts({
        authority: authority.publicKey,
        product,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([authority])
      .rpc();

    console.log("Your transaction signature", tx);
  });

  it("Add a Tier to the newly crerated Product.", async () => {
    const tier = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("tier"), product.toBuffer(), Buffer.from("test")],
      program.programId
    )[0];

    const auth = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("auth"), product.toBuffer(), tier.toBuffer()],
      program.programId
    )[0];

    await program.methods
      .addTier(
        "test",
        new anchor.BN(5000),
        new anchor.BN(5000),
        new anchor.BN(5000)
      )
      .accounts({
        authority: authority.publicKey,
        product,
        tier,
        mint,
        auth,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      }).rpc();


  });

  it("Purchase a Subscription", async () => {
    /*const product = anchor.web3.PublicKey.findProgramAddressSync([
      Buffer.from("product"),
      authority.publicKey.toBuffer(),
      Buffer.from("test")
    ],
      program.programId)[0];

    const tier = anchor.web3.PublicKey.findProgramAddressSync([
      Buffer.from("tier"),
      product.toBuffer(),
      Buffer.from("test")],
      program.programId)[0];

    const subscription = anchor.web3.PublicKey.findProgramAddressSync([
      Buffer.from("subscription"),
      tier.toBuffer(),
      authority.publicKey.toBuffer()],
      program.programId)[0];*/
  });

  it("Cancel a Subscription", async () => {});
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
