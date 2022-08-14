import { LAMPORTS_PER_SOL } from '@solana/web3.js'

export const LamportsToSOLFormat = (lamports) => {
    return parseFloat(lamports / LAMPORTS_PER_SOL).toFixed(2)
}

export const simpleShowPublicKey = (publicKey) => {
    if (publicKey && publicKey.length > 10) {
        return publicKey.substring(0, 4) + '...' + publicKey.substring(publicKey.length - 4)
    }

    return publicKey
}