
const anchor = require("@project-serum/anchor");

const LAMPORTS_PER_SOL = 1000000000;

describe('crypto-blessing', () => {

    

    // Configure the client to use the local cluster.
    const provider = anchor.Provider.env()
    anchor.setProvider(provider);
    const program = anchor.workspace.SolProgram;
    const sender = program.provider.wallet.publicKey
    const admin_param = anchor.web3.Keypair.generate()

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

    const blessing_owner = anchor.web3.Keypair.generate()
    const blessing = anchor.web3.Keypair.generate();
    it('can add blessing', async () => {

        await program.rpc.addBlessing(
            'image', 
            blessing_owner.publicKey,
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

    const sender_blessing1 = anchor.web3.Keypair.generate();

    it('can send blessing 1', async () => {
        console.log('blessing_owner', blessing_owner.publicKey)

        let beforeBalance = await provider.connection.getBalance(sender);
        console.log('beforeBalance', beforeBalance / LAMPORTS_PER_SOL)
       
        await program.rpc.sendBlessing(
            new anchor.BN(1 * LAMPORTS_PER_SOL) , 
            new anchor.BN(2), 
            {random:{}}, 
            [anchor.web3.Keypair.generate().publicKey, anchor.web3.Keypair.generate().publicKey],
        {
            accounts: {
                senderBlessing: sender_blessing1.publicKey,
                sender: sender,
                blessing: blessing.publicKey,
                blessingOwner: blessing_owner.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId,
            },
            signers: [sender_blessing1],
        });
    })

    it('can revoke blessing 1', async () => {
        await program.rpc.revokeBlessing(
        {
            accounts: {
                senderBlessing: sender_blessing1.publicKey,
                sender: sender,
            },
        });
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
            [anchor.web3.Keypair.generate().publicKey, anchor.web3.Keypair.generate().publicKey,
                anchor.web3.Keypair.generate().publicKey, anchor.web3.Keypair.generate().publicKey,
                anchor.web3.Keypair.generate().publicKey, anchor.web3.Keypair.generate().publicKey,
                anchor.web3.Keypair.generate().publicKey, anchor.web3.Keypair.generate().publicKey,
                anchor.web3.Keypair.generate().publicKey, anchor.web3.Keypair.generate().publicKey],
        {
            accounts: {
                senderBlessing: sender_blessing2.publicKey,
                sender: sender,
                blessing: blessing.publicKey,
                blessingOwner: blessing_owner.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId,
            },
            signers: [sender_blessing2],
        });

        // Fetch the account details of the created tweet.
        const blessings = await program.account.senderBlessing.all();
        console.log('blessings:', blessings)
        let afterBalance = await provider.connection.getBalance(sender);
        console.log('afterBalance', afterBalance / LAMPORTS_PER_SOL)
        let senderBlessingBalance = await provider.connection.getBalance(sender_blessing1.publicKey);
        console.log('senderBlessingBalance', senderBlessingBalance / LAMPORTS_PER_SOL)
        let blessingOwnerBalance = await provider.connection.getBalance(blessing_owner.publicKey);
        console.log('blessingOwnerBalance', blessingOwnerBalance / LAMPORTS_PER_SOL)
    });


});