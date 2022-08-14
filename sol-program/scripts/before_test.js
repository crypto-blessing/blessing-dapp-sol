
// import kp from './admin_param_keypair.json'

const anchor = require("@project-serum/anchor");

describe('crypto-blessing', () => {

    const provider = anchor.Provider.env()
    anchor.setProvider(provider);
    const program = anchor.workspace.SolProgram;
    const sender = program.provider.wallet.publicKey
    console.log('sender', sender)

    const keypair = anchor.web3.Keypair.generate()
    console.log(Buffer.from(keypair.secretKey).toString('base64'))

})

