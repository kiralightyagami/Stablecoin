import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Stablecoin } from "../target/types/Stablecoin";
import { PythSolanaReceiver } from "@pythnetwork/pyth-solana-receiver";

describe("stablecoin", () => {
  const provider = anchor.AnchorProvider.env();
  const connection = provider.connection;
  const wallet = provider.wallet as anchor.Wallet;
  
  anchor.setProvider(provider);

  const program = anchor.workspace.stablecoin as Program<Stablecoin>;

  const pythSolanaReceiver = new PythSolanaReceiver({
    connection,
    wallet,
  });

  const SOL_PRICE_FEED_ID = "0xfe650f0367d4a7ef9815a593ea15d36593f0643aaaf0149bb04be67ab851decd";

  const solUsdPriceFeedAccount = pythSolanaReceiver.getPriceFeedAccountAddress(
    0,
    SOL_PRICE_FEED_ID
  );

  const [collateralAccount] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("collateral"), wallet.publicKey.toBuffer()],
    program.programId
  );

  it("Is initialized!", async () => {
    const tx = await program.methods
    .initConfig()
    .accounts({})
    .rpc({skipPreflight: true, commitment: "confirmed"});
 
    console.log("Transaction signature:", tx);
  });

  it("Deposits collateral and mints tokens", async () => {
    const collateralAmount = 1000000000;
    const amountToMint = 1000000000;

    const tx = await program.methods
    .depositCollateralAndMintTokens(new anchor.BN(collateralAmount), new anchor.BN(amountToMint))
    .accounts({
      priceUpdate: solUsdPriceFeedAccount,
    })
    .rpc({skipPreflight: true, commitment: "confirmed"});

    console.log("Transaction signature:", tx);
    
  })

  it("Redeems collateral and burns tokens", async () => {
    const collateralAmount = 500000000;
    const amountToBurn = 500000;

    const tx = await program.methods
    .redeemCollateralAndBurnTokens(new anchor.BN(collateralAmount), new anchor.BN(amountToBurn))
    .accounts({
      priceUpdate: solUsdPriceFeedAccount,
    })
    .rpc({skipPreflight: true, commitment: "confirmed"});

    console.log("Transaction signature:", tx);
    
  })

  it("Updates config", async () => {
    const tx = await program.methods
    .updateConfig(new anchor.BN(100))
    .accounts({})
    .rpc({skipPreflight: true, commitment: "confirmed"});

    console.log("Transaction signature:", tx);
  })

  it("Liquidates collateral", async () => {

    const tx = await program.methods
    .liquidate(new anchor.BN(1))
    .accounts({})
    .rpc({skipPreflight: true, commitment: "confirmed"});

    console.log("Transaction signature:", tx);
  })

  it("Update Config", async () => {
    const tx = await program.methods
    .updateConfig(new anchor.BN(1))
    .accounts({})
    .rpc({skipPreflight: true, commitment: "confirmed"});

    console.log("Transaction signature:", tx);
  })
});
