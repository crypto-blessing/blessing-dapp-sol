import sha256 from 'js-sha256';

// const anchor = import "@project-serum/anchor";

import * as spl from '@solana/spl-token';
import { clusterApiUrl, Connection, Keypair, LAMPORTS_PER_SOL } from '@solana/web3.js';


import anchor from "@project-serum/anchor";



describe('crypto-blessing', () => {

    // Configure the client to use the local cluster.
    const provider = anchor.Provider.env()

    anchor.setProvider(provider);
    const program = anchor.workspace.SolProgram;
    const senderKeypair = program.provider.wallet.payer
    const sender = program.provider.wallet.publicKey
    const fakeSender = anchor.web3.Keypair.generate()
    const admin_param = anchor.web3.Keypair.generate()



    const createCBT = async () => {
        const tokenMint = new anchor.web3.Keypair();
        const lamportsForMint = await provider.connection.getMinimumBalanceForRentExemption(spl.MintLayout.span);
        let tx = new anchor.web3.Transaction();

        // Allocate mint
        tx.add(
            anchor.web3.SystemProgram.createAccount({
                programId: spl.TOKEN_PROGRAM_ID,
                space: spl.MintLayout.span,
                fromPubkey: provider.wallet.publicKey,
                newAccountPubkey: tokenMint.publicKey,
                lamports: lamportsForMint,
            })
        )

        // Allocate wallet account
        tx.add(
            spl.Token.createInitMintInstruction(
                spl.TOKEN_PROGRAM_ID,
                tokenMint.publicKey,
                9,
                provider.wallet.publicKey,
                provider.wallet.publicKey,
            )
        );
        const signature = await provider.send(tx, [tokenMint]);

        console.log(`[${tokenMint.publicKey}] Created new mint account at ${signature}`);

        return tokenMint.publicKey;
    }


    it("Is initialized!", async () => {
        // Add your test here.
        

        const tx = await program.rpc.initialize({
            accounts: {
                adminParam: admin_param.publicKey,
                owner: sender,
                systemProgram: anchor.web3.SystemProgram.programId,
            },
            signers: [admin_param],
        });
        console.log("Your transaction signature", tx);
    });

    // const blessing_owner = anchor.web3.Keypair.generate()
    const designer = new anchor.web3.PublicKey("DhCK19XeATX4yo1rm7Nqpv4fgBqYE815qeJAXhfF3iY9")
    const blessing = anchor.web3.Keypair.generate();
    it('can add blessing', async () => {

        await program.rpc.addBlessing(
            'image', 
            designer,
            new anchor.BN(0.05 * LAMPORTS_PER_SOL), 
            30 , 
            'ipfs', 
        {
            accounts: {
                blessing: blessing.publicKey,
                adminParam: admin_param.publicKey,
                owner: sender,
                systemProgram: anchor.web3.SystemProgram.programId,
            },
            signers: [blessing],
        });
    })

    it('can update blessing', async () => {

        await program.rpc.updateBlessing(
            'image', 
            designer,
            new anchor.BN(0.05 * LAMPORTS_PER_SOL), 
            40 , 
            'ipfs', 
        {
            accounts: {
                blessing: blessing.publicKey,
                adminParam: admin_param.publicKey,
                owner: sender,
            },
        });
    })

    const sender_blessing1 = anchor.web3.Keypair.generate();


    let claimKey1 = anchor.web3.Keypair.generate().publicKey.toBase58()
    let claimKey2 = anchor.web3.Keypair.generate().publicKey.toBase58()
    let claimKey3 = anchor.web3.Keypair.generate().publicKey.toBase58()
    let claimKey4 = anchor.web3.Keypair.generate().publicKey.toBase58()
    let claimKey5 = anchor.web3.Keypair.generate().publicKey.toBase58()
    let claimKey6 = anchor.web3.Keypair.generate().publicKey.toBase58()
    let claimKey7 = anchor.web3.Keypair.generate().publicKey.toBase58()
    let claimKey8 = anchor.web3.Keypair.generate().publicKey.toBase58()
    let claimKey9 = anchor.web3.Keypair.generate().publicKey.toBase58()
    let claimKey10 = anchor.web3.Keypair.generate().publicKey.toBase58()

    it('can send blessing 1', async () => {
        console.log('designer', designer.toBase58())

        let beforeBalance = await provider.connection.getBalance(sender);
        console.log('beforeBalance', beforeBalance / LAMPORTS_PER_SOL)
       
        await program.rpc.sendBlessing(
            new anchor.BN(1 * LAMPORTS_PER_SOL) , 
            new anchor.BN(2), 
            {random:{}}, 
            [sha256.sha256(claimKey1), sha256.sha256(claimKey2)],
        {
            accounts: {
                senderBlessing: sender_blessing1.publicKey,
                sender: sender,
                blessing: blessing.publicKey,
                blessingOwner: designer,
                systemProgram: anchor.web3.SystemProgram.programId,
            },
            signers: [sender_blessing1],
        });

        let afterBalance = await provider.connection.getBalance(sender);
        console.log('afterBalance of sender', afterBalance / LAMPORTS_PER_SOL)
        let after_blessing_owner = await provider.connection.getBalance(designer);
        console.log('after_blessing_owner', after_blessing_owner / LAMPORTS_PER_SOL)
        let afterBalanceOfSenderBlessing = await provider.connection.getBalance(sender_blessing1.publicKey);
        console.log('afterBalanceOfSenderBlessing', afterBalanceOfSenderBlessing / LAMPORTS_PER_SOL)
    })

    // it('can not revoke blessing 1', async () => {
    //     await program.rpc.revokeBlessing(
    //     {
    //         accounts: {
    //             senderBlessing: sender_blessing1.publicKey,
    //             sender: fakeSender.publicKey,
    //             systemProgram: anchor.web3.SystemProgram.programId,
    //         },
    //         signers: [fakeSender],
    //     });
    // })

    it('can revoke blessing 1', async () => {
        await program.rpc.revokeBlessing(
        {
            accounts: {
                senderBlessing: sender_blessing1.publicKey,
                sender: sender,
                systemProgram: anchor.web3.SystemProgram.programId,
            },
            signers: [],
        });
        let afterBalanceOfSenderBlessing = await provider.connection.getBalance(sender_blessing1.publicKey);
        console.log('afterBalanceOfSenderBlessing revoked', afterBalanceOfSenderBlessing / LAMPORTS_PER_SOL)
        let balance = await provider.connection.getBalance(sender);
        console.log('after balance revoked', balance / LAMPORTS_PER_SOL)
        let after_blessing_owner = await provider.connection.getBalance(designer);
        console.log('after_blessing_owner revoked', after_blessing_owner / LAMPORTS_PER_SOL)
    })

    const sender_blessing2 = anchor.web3.Keypair.generate();

    it('can send blessing 2', async () => {

        console.log('sender_blessing2', sender_blessing2.publicKey)

        let beforeBalance = await provider.connection.getBalance(sender);
        console.log('beforeBalance', beforeBalance / LAMPORTS_PER_SOL)
       
        await program.rpc.sendBlessing(
            new anchor.BN(1 * LAMPORTS_PER_SOL) , 
            new anchor.BN(10), 
            {random:{}}, 
            [sha256.sha256(claimKey1), sha256.sha256(claimKey2),sha256.sha256(claimKey3),sha256.sha256(claimKey4),sha256.sha256(claimKey5),
            sha256.sha256(claimKey6),sha256.sha256(claimKey7),sha256.sha256(claimKey8),sha256.sha256(claimKey9),sha256.sha256(claimKey10)],
        {
            accounts: {
                senderBlessing: sender_blessing2.publicKey,
                sender: sender,
                blessing: blessing.publicKey,
                blessingOwner: designer,
                systemProgram: anchor.web3.SystemProgram.programId,
            },
            signers: [sender_blessing2],
        });

        // Fetch the account details of the created tweet.
        const blessings = await program.account.senderBlessing.all();
        console.log('blessings:', blessings)
        let afterBalance = await provider.connection.getBalance(sender);
        console.log('afterBalance', afterBalance / LAMPORTS_PER_SOL)
        let senderBlessingBalance = await provider.connection.getBalance(sender_blessing2.publicKey);
        console.log('senderBlessingBalance', senderBlessingBalance / LAMPORTS_PER_SOL)
        let blessingOwnerBalance = await provider.connection.getBalance(designer);
        console.log('blessingOwnerBalance', blessingOwnerBalance / LAMPORTS_PER_SOL)
        let after_blessing_owner = await provider.connection.getBalance(designer);
        console.log('after_blessing_owner', after_blessing_owner / LAMPORTS_PER_SOL)
    });

    const fund_claimer = async (claimer) => {
        // Fund user with some SOL
        let txFund = new anchor.web3.Transaction();
        txFund.add(anchor.web3.SystemProgram.transfer({
            fromPubkey: sender,
            toPubkey: claimer.publicKey,
            lamports: 5 * anchor.web3.LAMPORTS_PER_SOL,
        }));
        const sigTxFund = await provider.send(txFund);
        console.log(`[${claimer.publicKey.toBase58()}] Funded new account with 5 SOL: ${sigTxFund}`);
    }

    it('can claim blessing 2', async () => {

        const claimer1 = anchor.web3.Keypair.generate()
        const claimer2 = anchor.web3.Keypair.generate()
        const claimer3 = anchor.web3.Keypair.generate()
        const claimer4 = anchor.web3.Keypair.generate()
        const claimer5 = anchor.web3.Keypair.generate()
        const claimer6 = anchor.web3.Keypair.generate()
        const claimer7 = anchor.web3.Keypair.generate()
        const claimer8 = anchor.web3.Keypair.generate()
        const claimer9 = anchor.web3.Keypair.generate()
        const claimer10 = anchor.web3.Keypair.generate()

        const claimer_blessing1 = anchor.web3.Keypair.generate();
        await fund_claimer(claimer1);

        let claimer1Balance = await provider.connection.getBalance(claimer1.publicKey);
        console.log('claimer1', claimer1Balance / LAMPORTS_PER_SOL)

        await createCBT()

        await program.rpc.claimBlessing(
            'claim title', claimKey1,
        {
            accounts: {
                claimerBlessing: claimer_blessing1.publicKey,
                claimer: claimer1.publicKey,
                senderBlessing: sender_blessing2.publicKey,
                blessing: blessing.publicKey,
                adminParam: admin_param.publicKey,
                programOwner: sender,
                sender: sender,
                systemProgram: anchor.web3.SystemProgram.programId,
            },
            signers: [claimer1, claimer_blessing1],
        });

        claimer1Balance = await provider.connection.getBalance(claimer1.publicKey);
        console.log('claimer1', claimer1Balance / LAMPORTS_PER_SOL)


        const claimer_blessing2 = anchor.web3.Keypair.generate();
        await fund_claimer(claimer2);

        let claimer2Balance = await provider.connection.getBalance(claimer2.publicKey);
        console.log('claimer2', claimer2Balance / LAMPORTS_PER_SOL)

        await createCBT()

        await program.rpc.claimBlessing(
            'claim title', claimKey2,
        {
            accounts: {
                claimerBlessing: claimer_blessing2.publicKey,
                claimer: claimer2.publicKey,
                senderBlessing: sender_blessing2.publicKey,
                blessing: blessing.publicKey,
                adminParam: admin_param.publicKey,
                programOwner: sender,
                sender: sender,
                systemProgram: anchor.web3.SystemProgram.programId,
            },
            signers: [claimer2, claimer_blessing2],
        });

        claimer2Balance = await provider.connection.getBalance(claimer2.publicKey);
        console.log('claimer2', claimer2Balance / LAMPORTS_PER_SOL)

        let senderBlessingBalance = await provider.connection.getBalance(sender_blessing2.publicKey);
        console.log('senderBlessingBalance', senderBlessingBalance / LAMPORTS_PER_SOL)

    })


});