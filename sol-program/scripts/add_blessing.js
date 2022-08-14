
// import kp from './admin_param_keypair.json'

const anchor = require("@project-serum/anchor");

const LAMPORTS_PER_SOL = 1000000000;
const admin_param_id_key = JSON.parse('{"_keypair":{"publicKey":{"0":54,"1":153,"2":117,"3":248,"4":246,"5":104,"6":16,"7":33,"8":59,"9":92,"10":77,"11":35,"12":165,"13":133,"14":29,"15":135,"16":193,"17":35,"18":221,"19":54,"20":253,"21":241,"22":65,"23":219,"24":5,"25":209,"26":234,"27":201,"28":50,"29":142,"30":24,"31":201},"secretKey":{"0":157,"1":9,"2":198,"3":185,"4":240,"5":213,"6":174,"7":42,"8":16,"9":156,"10":143,"11":192,"12":146,"13":126,"14":53,"15":45,"16":201,"17":18,"18":97,"19":33,"20":74,"21":169,"22":73,"23":226,"24":61,"25":111,"26":84,"27":52,"28":184,"29":219,"30":113,"31":134,"32":54,"33":153,"34":117,"35":248,"36":246,"37":104,"38":16,"39":33,"40":59,"41":92,"42":77,"43":35,"44":165,"45":133,"46":29,"47":135,"48":193,"49":35,"50":221,"51":54,"52":253,"53":241,"54":65,"55":219,"56":5,"57":209,"58":234,"59":201,"60":50,"61":142,"62":24,"63":201}}}')


describe('crypto-blessing', () => {

    const provider = anchor.Provider.env()
    anchor.setProvider(provider);
    const program = anchor.workspace.SolProgram;
    const sender = program.provider.wallet.publicKey

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
            sender.publicKey,
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
            sender.publicKey,
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
            sender.publicKey,
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
            sender.publicKey,
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
            sender.publicKey,
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

