// ** Next Imports
import Head from 'next/head'
import { Router } from 'next/router'

// ** Loader Import
import NProgress from 'nprogress'

// ** Emotion Imports
import { CacheProvider } from '@emotion/react'

// ** Config Imports
import themeConfig from 'src/configs/themeConfig'

// ** Component Imports
import UserLayout from 'src/layouts/UserLayout'
import ThemeComponent from 'src/@core/theme/ThemeComponent'

// ** Contexts
import { SettingsConsumer, SettingsProvider } from 'src/@core/context/settingsContext'

// ** Utils Imports
import { createEmotionCache } from 'src/@core/utils/create-emotion-cache'

import { FC, useMemo } from 'react';
import { ConnectionProvider, WalletProvider } from '@solana/wallet-adapter-react';
import { WalletAdapterNetwork } from '@solana/wallet-adapter-base';
import {
    CoinbaseWalletAdapter,
    GlowWalletAdapter,
    PhantomWalletAdapter,
    SlopeWalletAdapter,
    SolflareWalletAdapter,
    SolletExtensionWalletAdapter,
    SolletWalletAdapter,
    TorusWalletAdapter,
    TokenaryWalletAdapter,
} from '@solana/wallet-adapter-wallets';
import {
    WalletModalProvider,
    WalletDisconnectButton,
    WalletMultiButton
} from '@solana/wallet-adapter-react-ui';
import { clusterApiUrl } from '@solana/web3.js';

// Default styles that can be overridden by your app
import '@solana/wallet-adapter-react-ui/styles.css'


// ** React Perfect Scrollbar Style
import 'react-perfect-scrollbar/dist/css/styles.css'

// ** Global css styles
import '../../styles/globals.css'

const clientSideEmotionCache = createEmotionCache()

// ** Pace Loader
if (themeConfig.routingLoader) {
  Router.events.on('routeChangeStart', () => {
    NProgress.start()
  })
  Router.events.on('routeChangeError', () => {
    NProgress.done()
  })
  Router.events.on('routeChangeComplete', () => {
    NProgress.done()
  })
}

// ** Configure JSS & ClassName
const App = props => {

  const { Component, emotionCache = clientSideEmotionCache, pageProps } = props

  // Variables
  const getLayout = Component.getLayout ?? (page => <UserLayout>{page}</UserLayout>)

  // The network can be set to 'devnet', 'testnet', or 'mainnet-beta'.
  const network = WalletAdapterNetwork.Devnet;

  // You can also provide a custom RPC endpoint.
  const endpoint = useMemo(() => clusterApiUrl(network), [network]);

  const wallets = useMemo(
      () => [
          /**
           * Select the wallets you wish to support, by instantiating wallet adapters here.
           *
           * Common adapters can be found in the npm package `@solana/wallet-adapter-wallets`.
           * That package supports tree shaking and lazy loading -- only the wallets you import
           * will be compiled into your application, and only the dependencies of wallets that
           * your users connect to will be loaded.
           */
           new CoinbaseWalletAdapter(),
           new GlowWalletAdapter(),
           new PhantomWalletAdapter(),
           new SlopeWalletAdapter(),
           new SolflareWalletAdapter(),
           new SolletExtensionWalletAdapter(),
           new SolletWalletAdapter(),
           new TorusWalletAdapter(),
           new TokenaryWalletAdapter(),
      ],
      []
  );

  return (
    
    <CacheProvider value={emotionCache}>
      
        <Head>
          <title>{`${themeConfig.templateName} - Blessing is the most universal human expression of emotion, and we are NFTizing it.`}</title>
          <meta
            name='description'
            content={`${themeConfig.templateName} – Blessing is the most universal human expression of emotion, and we are NFTizing it.`}
          />
          <meta name='keywords' content='crypto, blessing, coins, web3, lucky, bag, red' />
          <meta name='viewport' content='initial-scale=1, width=device-width' />
        </Head>
        <ConnectionProvider endpoint={endpoint}>
          <WalletProvider wallets={wallets} autoConnect>
            <WalletModalProvider>
              {/* <WalletMultiButton />
              <WalletDisconnectButton /> */}
              <SettingsProvider>
                <SettingsConsumer>
                  {({ settings }) => {
                    return <ThemeComponent settings={settings}>{getLayout(
                            <Component {...pageProps} />
                        )}</ThemeComponent>
                  }}
                </SettingsConsumer>
              </SettingsProvider>
            </WalletModalProvider>
          </WalletProvider>
        </ConnectionProvider>
        
    </CacheProvider>
  )
}

export default App
