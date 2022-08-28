
// import kp from './admin_param_keypair.json'

import anchor from "@project-serum/anchor";
import {
    TOKEN_PROGRAM_ID,
    createAssociatedTokenAccountInstruction,
    getAssociatedTokenAddress,
    createInitializeMintInstruction,
    MINT_SIZE,
  } from "@solana/spl-token";

describe('crypto-blessing', () => {

    const provider = anchor.AnchorProvider.env()
    anchor.setProvider(provider);
    const program = anchor.workspace.SolProgram;
    const sender = program.provider.wallet.publicKey

    it("Mint Nft!", async () => {

        const TOKEN_METADATA_PROGRAM_ID = new anchor.web3.PublicKey(
            "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
        );

        const mintKeypair = anchor.web3.Keypair.generate();

        const tokenAddress = await anchor.utils.token.associatedAddress({
            mint: mintKeypair.publicKey,
            owner: sender
        });
        console.log(`New token: ${mintKeypair.publicKey}, tokenAddress: ${tokenAddress}`);

        // Derive the metadata and master edition addresses

        const metadataAddress = (await anchor.web3.PublicKey.findProgramAddress(
        [
            Buffer.from("metadata"),
            TOKEN_METADATA_PROGRAM_ID.toBuffer(),
            mintKeypair.publicKey.toBuffer(),
        ],
        TOKEN_METADATA_PROGRAM_ID
        ))[0];
        console.log(`Metadata initialized, metadataAddress: ${metadataAddress}`);

        const masterEditionAddress = (await anchor.web3.PublicKey.findProgramAddress(
        [
            Buffer.from("metadata"),
            TOKEN_METADATA_PROGRAM_ID.toBuffer(),
            mintKeypair.publicKey.toBuffer(),
            Buffer.from("edition"),
        ],
        TOKEN_METADATA_PROGRAM_ID
        ))[0];
        console.log(`Master edition metadata initialized, masterEditionAddress: ${masterEditionAddress}`);

        const testNftTitle = "CryptoBlessing";
        const testNftSymbol = "CBNFT";
        const testNftUri = "https://raw.githubusercontent.com/crypto-blessing/blessing-dapp-sol/main/sol-program/assets/cbnft_meta.json";

        // await program.rpc.mintNft(
        //     testNftTitle, metadata_symbol, testNftUri, {
        //         accounts: {
        //             masterEdition: masterEditionAddress,
        //             metadata: metadataAddress,
        //             mint: mintKeypair.publicKey,
        //             tokenAccount: tokenAddress,
        //             mintAuthority: sender,
        //             tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
        //             systemProgram: anchor.web3.SystemProgram.programId,
        //             rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        //             tokenProgram: TOKEN_PROGRAM_ID,
        //         },
        //         signers: [mintKeypair],
        //     }
        // )

        await program.methods.mintNft(
            testNftTitle, testNftSymbol, testNftUri
          )
          .accounts({
            masterEdition: masterEditionAddress,
            metadata: metadataAddress,
            mint: mintKeypair.publicKey,
            tokenAccount: tokenAddress,
            mintAuthority: sender,
            tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
          })
          .signers([mintKeypair])
          .rpc();
    })

})

