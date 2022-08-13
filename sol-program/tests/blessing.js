
const anchor = require("@project-serum/anchor");

const LAMPORTS_PER_SOL = 1000000000;

describe('crypto-blessing', () => {

    

    // Configure the client to use the local cluster.
    const provider = anchor.Provider.env()
    anchor.setProvider(provider);
    const program = anchor.workspace.SolProgram;

    it('can send a new blessing', async () => {
        const sender = program.provider.wallet.publicKey
        const blessing_owner = anchor.web3.Keypair.generate()
        const blessing = anchor.web3.Keypair.generate();

        await program.rpc.addBlessing(
            'image', 
            blessing_owner.publicKey,
            new anchor.BN(0.05 * LAMPORTS_PER_SOL), 
            30 , 
            'ipfs', 
        {
            accounts: {
                blessing: blessing.publicKey,
                owner: sender,
                systemProgram: anchor.web3.SystemProgram.programId,
            },
            signers: [blessing],
        });

        const sender_blessing1 = anchor.web3.Keypair.generate();
        const blessing1 = anchor.web3.Keypair.generate();

        console.log('sender_blessing1', sender_blessing1.publicKey)
        console.log('blessing1', blessing1.publicKey)
        console.log('blessing_owner', blessing_owner.publicKey)

        let beforeBalance = await provider.connection.getBalance(sender);
        console.log('beforeBalance', beforeBalance / LAMPORTS_PER_SOL)
       
        await program.rpc.sendBlessing(
            blessing1.publicKey, 
            'image', 
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
                senderBlessing: sender_blessing1.publicKey,
                sender: sender,
                blessing: blessing.publicKey,
                blessingOwner: blessing_owner.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId,
            },
            signers: [sender_blessing1],
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