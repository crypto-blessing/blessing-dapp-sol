
// import kp from './admin_param_keypair.json'

const anchor = require("@project-serum/anchor");

const LAMPORTS_PER_SOL = 1000000000;
const admin_param_id_key = JSON.parse('{"_keypair":{"publicKey":{"0":162,"1":147,"2":110,"3":184,"4":143,"5":127,"6":222,"7":229,"8":179,"9":149,"10":207,"11":153,"12":7,"13":207,"14":27,"15":207,"16":67,"17":124,"18":242,"19":26,"20":74,"21":74,"22":12,"23":213,"24":181,"25":158,"26":106,"27":255,"28":90,"29":182,"30":217,"31":81},"secretKey":{"0":87,"1":160,"2":179,"3":53,"4":144,"5":209,"6":134,"7":80,"8":84,"9":192,"10":179,"11":176,"12":5,"13":115,"14":116,"15":63,"16":131,"17":107,"18":43,"19":182,"20":239,"21":204,"22":249,"23":13,"24":205,"25":11,"26":225,"27":76,"28":59,"29":125,"30":70,"31":125,"32":162,"33":147,"34":110,"35":184,"36":143,"37":127,"38":222,"39":229,"40":179,"41":149,"42":207,"43":153,"44":7,"45":207,"46":27,"47":207,"48":67,"49":124,"50":242,"51":26,"52":74,"53":74,"54":12,"55":213,"56":181,"57":158,"58":106,"59":255,"60":90,"61":182,"62":217,"63":81}}}')


describe('crypto-blessing', () => {

    const provider = anchor.Provider.env()
    anchor.setProvider(provider);
    const program = anchor.workspace.SolProgram;
    const sender = program.provider.wallet.publicKey
    const designer = new anchor.web3.PublicKey("DhCK19XeATX4yo1rm7Nqpv4fgBqYE815qeJAXhfF3iY9")
    const arr = Object.values(admin_param_id_key._keypair.secretKey)
    const secret = new Uint8Array(arr)
    const admin_param = anchor.web3.Keypair.fromSecretKey(secret)

    it("Add new blessings!", async () => {

        console.log('sender', program.provider.wallet)
        let beforeBalance = await provider.connection.getBalance(sender);
        console.log('beforeBalance', beforeBalance / LAMPORTS_PER_SOL)
        let blessing = await program.account.adminParam.all();
        console.log('blessing', blessing)

        let blessing1 = anchor.web3.Keypair.generate()

        const tx =  await program.rpc.addBlessing(
            'goddess_blessing.gif', 
            designer,
            new anchor.BN(100000000), 
            10 , 
            'https://bafybeigdqtu6clkz2bghdftt62tvsadkzyyaqoet5qly2kg6fe54ncggwu.ipfs.nftstorage.link/goddess_blessing.gif', 
        {
            accounts: {
                blessing: blessing1.publicKey,
                adminParam: admin_param.publicKey,
                owner: sender,
                systemProgram: anchor.web3.SystemProgram.programId,
            },
            signers: [blessing1],
        });
        console.log("Your transaction signature", tx);

        blessing1 = anchor.web3.Keypair.generate()
        await program.rpc.addBlessing(
            'I_adore_you.gif', 
            designer,
            new anchor.BN(100000000), 
            10 , 
            'https://bafybeigdqtu6clkz2bghdftt62tvsadkzyyaqoet5qly2kg6fe54ncggwu.ipfs.nftstorage.link/I_adore_you.gif', 
        {
            accounts: {
                blessing: blessing1.publicKey,
                adminParam: admin_param.publicKey,
                owner: sender,
                systemProgram: anchor.web3.SystemProgram.programId,
            },
            signers: [blessing1],
        });

        blessing1 = anchor.web3.Keypair.generate()
        await program.rpc.addBlessing(
            '12_love.gif', 
            designer,
            new anchor.BN(100000000), 
            10 , 
            'https://bafybeigdqtu6clkz2bghdftt62tvsadkzyyaqoet5qly2kg6fe54ncggwu.ipfs.nftstorage.link/12_love.gif', 
        {
            accounts: {
                blessing: blessing1.publicKey,
                adminParam: admin_param.publicKey,
                owner: sender,
                systemProgram: anchor.web3.SystemProgram.programId,
            },
            signers: [blessing1],
        });

        blessing1 = anchor.web3.Keypair.generate()
        await program.rpc.addBlessing(
            'mirabilis_lady.gif', 
            designer,
            new anchor.BN(100000000), 
            10 , 
            'https://bafybeigdqtu6clkz2bghdftt62tvsadkzyyaqoet5qly2kg6fe54ncggwu.ipfs.nftstorage.link/mirabilis_lady.gif', 
        {
            accounts: {
                blessing: blessing1.publicKey,
                adminParam: admin_param.publicKey,
                owner: sender,
                systemProgram: anchor.web3.SystemProgram.programId,
            },
            signers: [blessing1],
        });

        blessing1 = anchor.web3.Keypair.generate()
        await program.rpc.addBlessing(
            'YOU+ME.gif', 
            designer,
            new anchor.BN(100000000), 
            10 , 
            'https://bafybeigdqtu6clkz2bghdftt62tvsadkzyyaqoet5qly2kg6fe54ncggwu.ipfs.nftstorage.link/YOU+ME.gif', 
        {
            accounts: {
                blessing: blessing1.publicKey,
                adminParam: admin_param.publicKey,
                owner: sender,
                systemProgram: anchor.web3.SystemProgram.programId,
            },
            signers: [blessing1],
        });

        blessing = await program.account.blessing.all();
        console.log('blessing', blessing)

    })

})

