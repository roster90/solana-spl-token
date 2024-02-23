import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SplTokenStudy } from "../target/types/spl_token_study";
import {
  getAssociatedTokenAddress,
  createMint,
  TOKEN_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID,
  getAccount,
  getOrCreateAssociatedTokenAccount,
  getAssociatedTokenAddressSync,
} from "@solana/spl-token"
import { assert } from "chai";

describe("spl-token-study", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();

  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SplTokenStudy as Program<SplTokenStudy>;
  // const mintToken = anchor.web3.Keypair.generate();

          // Constants from our program
          const MINT_SEED = "mint";
    const [mint] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(MINT_SEED)],
      program.programId
    );

  it("Is initialized token", async () => {
    const METADATA_SEED = "metadata";
    const TOKEN_METADATA_PROGRAM_ID = new anchor.web3.PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");

    let token_dummy  = {name:"DUMMY TOKEN" , decimals: 9, symbol: "DUMMY", uri :"https://5vfxc4tr6xoy23qefqbj4qx2adzkzapneebanhcalf7myvn5gzja.arweave.net/7UtxcnH13Y1uBCwCnkL6APKsge0hAgacQFl-zFW9NlI", };
    
    const [metadataAddress] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from(METADATA_SEED),
        TOKEN_METADATA_PROGRAM_ID.toBuffer(),
        mint.toBuffer(),
      ],
      TOKEN_METADATA_PROGRAM_ID
    );

    const context = {
      metadata: metadataAddress,
      mint,
      signer: provider.wallet.publicKey,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      systemProgram: anchor.web3.SystemProgram.programId,
      tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
    };
    try {
         // Add your test here.
      const txHash = await program.methods.initToken(token_dummy).accounts(context).rpc();
      console.log("Your transaction signature", txHash);
      await provider.connection.confirmTransaction(txHash, 'finalized');
      const newInfo = await provider.connection.getAccountInfo(mint);
      assert(newInfo, "  Mint should be initialized."); 
    } catch (error) {
        console.log(error);
        
    }


  });


  // it("mint to account ", async () => {


  //   const mintAmount =  new anchor.BN(100000);
  //   const tokenAccount =  getAssociatedTokenAddressSync(mint, provider.wallet.publicKey, true);
  //   const context = {
  //     mint,
  //     tokenAccount,
  //     signer: provider.wallet.publicKey,
  //     rent: anchor.web3.SYSVAR_RENT_PUBKEY,
  //     systemProgram: anchor.web3.SystemProgram.programId,
  //     tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
  //   };
  //   // Add your test here.
  //   const txHash = await program.methods.mintTokens(mintAmount).accounts(context).rpc();
  //   console.log("Your transaction signature", txHash);


  // });
});
