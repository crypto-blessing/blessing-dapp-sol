// ** MUI Imports
import Grid from '@mui/material/Grid'
import Typography from '@mui/material/Typography'

import Paper from '@mui/material/Paper'
import Table from '@mui/material/Table'
import { styled } from '@mui/material/styles'
import TableHead from '@mui/material/TableHead'
import TableBody from '@mui/material/TableBody'
import TableContainer from '@mui/material/TableContainer'
import TableRow from '@mui/material/TableRow'
import TableCell, { tableCellClasses } from '@mui/material/TableCell'
import Card from '@mui/material/Card'
import CardContent from '@mui/material/CardContent'
import CardHeader from '@mui/material/CardHeader'
import ImageList from '@mui/material/ImageList';
import ImageListItem from '@mui/material/ImageListItem';
import Chip from '@mui/material/Chip';
import Link from '@mui/material/Link';
import Box from '@mui/material/Box'
import Avatar from '@mui/material/Avatar';
import {SOL_ICON, CBT_ICON} from 'src/@core/components/wallet/crypto-icons'

import{ cbtBalance, nftBalance } from 'src/@core/configs/utils'
import { useWallet, useConnection } from '@solana/wallet-adapter-react';
import {LamportsToSOLFormat} from 'src/@core/components/wallet/utils'
import { useEffect, useState } from "react"


const StyledTableCell = styled(TableCell)(({ theme }) => ({
    [`&.${tableCellClasses.head}`]: {
      color: theme.palette.common.black,
      backgroundColor: '#ede3ff'
    },
    [`&.${tableCellClasses.body}`]: {
      fontSize: 14
    }
  }))
  
const StyledTableRow = styled(TableRow)(({ theme }) => ({
'&:nth-of-type(odd)': {
    backgroundColor: theme.palette.action.hover
},

// hide last border
'&:last-of-type td, &:last-of-type th': {
    border: 0
}
}))


const Wallet = () => {
    const { connection } = useConnection();
    const { publicKey } = useWallet();

    const [solAmount, setSolAmount] = useState(0)
    const [CBTAmount, setCBTAmount] = useState('')
    const [CBNFTItems, setCBNFTItems] = useState([])


    async function fetchERC20Amount() {
        try {
            if (publicKey) {
                setSolAmount(await connection.getBalance(publicKey))
            }
        } catch (err) {
            console.log("Error: ", err)
        }
    }
    
    useEffect(() => {
        fetchERC20Amount()
    // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [publicKey])

    return (
        <Grid container spacing={6}>
            <Grid item xs={12} sx={{ paddingBottom: 4 }}>
                <Typography variant='h5'>My Assets</Typography>
            </Grid>
            <Grid item xs={12} sm={6}>
                <Card>
                    <CardHeader title='Tokens' titleTypographyProps={{ variant: 'h6' }} />
                    <TableContainer component={Paper}>
                        <Table sx={{ minWidth: 500 }} aria-label='customized table'>
                            <TableHead>
                                <TableRow>
                                    <StyledTableCell>Assets</StyledTableCell>
                                    <StyledTableCell align='right'>Balance</StyledTableCell>
                                </TableRow>
                            </TableHead>
                            { publicKey ?
                                <TableBody>
                                <StyledTableRow key='SOL'>
                                    <StyledTableCell component='th' scope='row'>
                                        <Chip variant="outlined" icon={<SOL_ICON />} label="SOL" />
                                    </StyledTableCell>
                                    <StyledTableCell align='right'>{LamportsToSOLFormat(solAmount)}</StyledTableCell>
                                </StyledTableRow>
                                <StyledTableRow key='CBT'>
                                    <StyledTableCell component='th' scope='row'>
                                        <Chip variant="outlined" icon={<CBT_ICON />} label="CBT" />
                                    </StyledTableCell>
                                    <StyledTableCell align='right'>{CBTAmount}</StyledTableCell>
                                </StyledTableRow>
                            </TableBody>
                            :
                            <Box sx={{ p: 5, display: 'flex', flexDirection: 'column', alignItems: 'center', textAlign: 'center' }}>
                                <Typography  variant="overline" display="block" gutterBottom>
                                    Pls login to see your assets
                                </Typography>
                            </Box>
                            }
                        </Table>
                    </TableContainer>
                </Card>
                {/* <Card>
                    <CardContent>
                        <Typography variant='caption'>See my NEP-141 Tokens on <Link target='_blank' href={nearConfig?.walletUrl}>NEAR Wallet Page</Link></Typography>
                    </CardContent>
                </Card> */}
                
            </Grid>
            <Grid item xs={12} sm={6}>
                <Card>
                    <CardHeader title='Blessing NFTs' titleTypographyProps={{ variant: 'h6' }} />
                    <CardContent>
                        { CBNFTItems.length > 0 ?
                        <ImageList sx={{ width: 500, height: 450 }} cols={3} rowHeight={164}>
                        {CBNFTItems.map((item, index) => (
                            <ImageListItem key={item + '-' + index}>
                            <img
                                src={`${item}?w=164&h=164&fit=crop&auto=format`}
                                srcSet={`${item}?w=164&h=164&fit=crop&auto=format&dpr=2 2x`}
                                alt={item}
                                loading="lazy"
                            />
                            </ImageListItem>
                        ))}
                        </ImageList>
                        :
                        <Box sx={{ p: 5, display: 'flex', flexDirection: 'column', alignItems: 'center', textAlign: 'center' }}>
                            <Typography  variant="overline" display="block" gutterBottom>
                                {publicKey ? "You don't have claimed any CryptoBlessing NFT yet!" : 'Pls login to see your assets'}
                            </Typography>
                        </Box>
                        }

                    </CardContent>
                    
                </Card>

                {/* <Card>
                    <CardContent>
                        <Typography variant='caption'>See my NEP-178 Tokens on <Link target='_blank' href={nearConfig?.walletUrl}>NEAR Collectibles Page</Link></Typography>
                    </CardContent>
                </Card> */}
            </Grid>
        </Grid>
    )
}

export default Wallet
