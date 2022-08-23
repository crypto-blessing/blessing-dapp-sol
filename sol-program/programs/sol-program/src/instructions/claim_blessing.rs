use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use crate::errors::CryptoBlessingError;
use crate::state::blessing::*;
use crate::state::sender_blessing::*;
use crate::state::claimer_blessing::*;
use crate::state::AdminParam;
use sha256::*;
use anchor_spl::token::mint_to;
use anchor_spl::token::{MintTo, Token};
use mpl_token_metadata::instruction::{create_metadata_accounts_v2};

fn random_num() -> u64 {
    let clock: Clock = Clock::get().unwrap();
    let mut random = (clock.unix_timestamp % 10) as u64;
    if random == 0 || random == 10 {
        random = 1;
    }
    return random;
}

pub fn claim_blessing(ctx: Context<ClaimBlessing>, 
    blessing_title: String,
    claim_key: String
) -> Result<()> {

    require_eq!(ctx.accounts.sender_blessing.revoked, false, CryptoBlessingError::BlessingRevoked);
        let claim_keys = ctx.accounts.sender_blessing.claim_keys.clone();
        require_gt!(claim_keys.len(), 0, CryptoBlessingError::NoKeysFound);
        let mut hex_finded = false;
        let mut claim_key_finded = "".to_string();
        for claim_key_info in claim_keys {
            if claim_key_info.key == digest(claim_key.clone()) {
                hex_finded = true;
                claim_key_finded = claim_key_info.key;
            }
        }
        require_eq!(hex_finded, true, CryptoBlessingError::ClaimKeyVerifyFailed);
        require_gt!(ctx.accounts.sender_blessing.claim_quantity as usize, ctx.accounts.sender_blessing.claimer_list.len(), CryptoBlessingError::NoBlessingLeft);
    
        // cal the claim amount
        let mut distributed_amount = 0;
        for status in ctx.accounts.sender_blessing.claimer_list.iter() {
            require_neq!(*ctx.accounts.claimer.key, status.claimer, CryptoBlessingError::RepeatClaimErr);
            distributed_amount += status.distributed_amount;
        }
        let mut distribution_amount = 0;
        match ctx.accounts.sender_blessing.claim_type {
            ClaimType::Average=>{
                distribution_amount = ctx.accounts.sender_blessing.token_amount / ctx.accounts.sender_blessing.claim_quantity;
            },
            ClaimType::Random => {
                let left_quantity = ctx.accounts.sender_blessing.claim_quantity - ctx.accounts.sender_blessing.claimer_list.len() as u64;
                let random_num = random_num();
                if left_quantity == 1 {
                    distribution_amount = ctx.accounts.sender_blessing.token_amount - distributed_amount;
                } else {
                    distribution_amount = (ctx.accounts.sender_blessing.token_amount - distributed_amount) / left_quantity * random_num / 10 * 2;
                }
            }, 
        }
    
        let mut cbt_token_reward = distribution_amount * ctx.accounts.admin_param.cbt_reward_ratio as u64;
        if cbt_token_reward > ctx.accounts.admin_param.cbt_reward_max {
            cbt_token_reward = ctx.accounts.admin_param.cbt_reward_max;
        }
        
        // update the sender_blessing
        ctx.accounts.sender_blessing.save_claim_key_used(claim_key_finded);
        let clock: Clock = Clock::get().unwrap();
        ctx.accounts.sender_blessing.insert_claimer_list(*ctx.accounts.claimer.key, clock.unix_timestamp, distributed_amount, 
            distribution_amount / 1000 * (1000 - ctx.accounts.admin_param.claim_tax_rate as u64),
            distribution_amount / 1000 * ctx.accounts.admin_param.claim_tax_rate as u64,
            cbt_token_reward,);
        
        let amount_to_claimer = distribution_amount / 1000 * (1000 - ctx.accounts.admin_param.claim_tax_rate as u64);
        **ctx.accounts.sender_blessing.to_account_info().try_borrow_mut_lamports()? -= amount_to_claimer;
        **ctx.accounts.claimer.to_account_info().try_borrow_mut_lamports()? += amount_to_claimer;
    
        let tax_to_program_owner = distribution_amount / 1000 * ctx.accounts.admin_param.claim_tax_rate as u64;
        **ctx.accounts.sender_blessing.to_account_info().try_borrow_mut_lamports()? -= tax_to_program_owner;
        **ctx.accounts.program_owner.to_account_info().try_borrow_mut_lamports()? += tax_to_program_owner;
    
        // transfer the cbt token to sender
        // let sender_blessing_pk = sender_blessing.key().clone();
        // let inner = vec![
        //     b"state".as_ref(),
        //     sender_blessing_pk.as_ref(),
        //     sender.key.as_ref(),
        // ];
        // let outer = vec![inner.as_slice()];
        // let transfer_instruction = Transfer{
        //     from: sender_blessing.to_account_info(),
        //     to: sender.to_account_info(),
        //     authority: sender_blessing.to_account_info(),
        // };
        // let cpi_ctx = CpiContext::new_with_signer(
        //     token_program.to_account_info(),
        //     transfer_instruction,
        //     outer.as_slice(),
        // );
        // anchor_spl::token::transfer(cpi_ctx, cbt_token_reward)?;
    
        // mint the NFT token to claimer
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.payer.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        // let result = mint_to(cpi_ctx, 1);
        // if let Err(_) = result {
        //     msg!("Token mint failed !!!");
        // } else {
        //     let accounts = vec![
        //         token_metadata_program.to_account_info(),
        //         metadata.to_account_info(),
        //         mint.to_account_info(),
        //         mint_authority.to_account_info(),
        //         payer.to_account_info(),
        //         token_program.to_account_info(),
        //         system_program.to_account_info(),
        //     ];
        //     let creators = vec![
        //         mpl_token_metadata::state::Creator {
        //             address: mint.key.clone(),
        //             verified: false,
        //             share: 100,
        //         },
        //         mpl_token_metadata::state::Creator {
        //             address: mint_authority.key(),
        //             verified: false,
        //             share: 0,
        //         },
        //     ];
        //     let result = invoke(
        //         &create_metadata_accounts_v2(
        //             token_metadata_program.key(),
        //             metadata.key(),
        //             mint.key(),
        //             mint_authority.key(),
        //             payer.key(),
        //             payer.key(),
        //             blessing_title,
        //             "CBNFT".to_string(),
        //             blessing.ipfs.clone(),
        //             Some(creators),
        //             1,
        //             true,
        //             false,
        //             None,
        //             None,
        //         ),
        //         &accounts
        //     );
        //     if let Err(_) = result {
        //         msg!("Token metadata creation failed !!!");
        //     }
        // }
        
        ctx.accounts.claimer_blessing.save(*ctx.accounts.claimer.key, ctx.accounts.sender_blessing.sender, ctx.accounts.sender_blessing.blessing_id, 
            ctx.accounts.sender_blessing.blessing_img.clone(), 
            distribution_amount / 1000 * (1000 - ctx.accounts.admin_param.claim_tax_rate as u64),
            distribution_amount / 1000 * ctx.accounts.admin_param.claim_tax_rate as u64)
}

// pub fn claim_blessing_with_new_claimer(ctx: Context<ClaimBlessingWithNewClaimer>, 
//     blessing_title: String,
//     claim_key: String
// ) -> Result<()> {
//     inner_claim_blessing(blessing_title, claim_key, 
//         &mut ctx.accounts.claimer_blessing, &mut ctx.accounts.claimer, &mut ctx.accounts.sender_blessing.to_owned(), 
//         ctx.accounts.blessing.to_owned(), ctx.accounts.admin_param.to_owned(), 
//         &mut ctx.accounts.program_owner, &mut ctx.accounts.sender, ctx.accounts.token_program.to_owned(), 
//         ctx.accounts.mint.to_owned(), ctx.accounts.token_account.to_owned(), ctx.accounts.payer.to_owned(), 
//         ctx.accounts.metadata.to_owned(), ctx.accounts.token_metadata_program.to_owned(), ctx.accounts.master_edition.to_owned(), 
//         ctx.accounts.system_program.to_owned())
// }

#[derive(Accounts)]
pub struct ClaimBlessing<'info> {
    #[account(init, payer = claimer, space = ClaimerBlessing::LEN + 8)]
    pub claimer_blessing: Account<'info, ClaimerBlessing>,
    #[account(mut)]
    pub claimer: Signer<'info>,
    #[account(mut)]
    pub sender_blessing: Account<'info, SenderBlessing>,
    pub blessing: Account<'info, Blessing>,
    pub admin_param: Account<'info, AdminParam>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub program_owner: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub sender: AccountInfo<'info>,

    #[account(mut)]
    pub mint_authority: Signer<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub mint: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub token_account: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_metadata_program: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub payer: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

// #[derive(Accounts)]
// pub struct ClaimBlessingWithNewClaimer<'info> {

//     /// CHECK: This is not dangerous because we don't read or write from this account
//     #[account(init, payer = sender_blessing, space = 8)]
//     pub claimer: AccountInfo<'info>,

//     #[account(init, payer = claimer, space = ClaimerBlessing::LEN + 8)]
//     pub claimer_blessing: Account<'info, ClaimerBlessing>,
//     #[account(mut)]
//     pub sender_blessing: Account<'info, SenderBlessing>,
//     pub blessing: Account<'info, Blessing>,
//     pub admin_param: Account<'info, AdminParam>,
//     /// CHECK: This is not dangerous because we don't read or write from this account
//     #[account(mut)]
//     pub program_owner: AccountInfo<'info>,
//     /// CHECK: This is not dangerous because we don't read or write from this account
//     #[account(mut)]
//     pub sender: AccountInfo<'info>,

//     /// CHECK: This is not dangerous because we don't read or write from this account
//     #[account(mut)]
//     pub mint: UncheckedAccount<'info>,
//     /// CHECK: This is not dangerous because we don't read or write from this account
//     #[account(mut)]
//     pub token_account: UncheckedAccount<'info>,
//     /// CHECK: This is not dangerous because we don't read or write from this account
//     #[account(mut)]
//     pub payer: AccountInfo<'info>,
//     /// CHECK: This is not dangerous because we don't read or write from this account
//     #[account(mut)]
//     pub metadata: UncheckedAccount<'info>,
//     /// CHECK: This is not dangerous because we don't read or write from this account
//     pub token_metadata_program: UncheckedAccount<'info>,
//     /// CHECK: This is not dangerous because we don't read or write from this account
//     #[account(mut)]
//     pub master_edition: UncheckedAccount<'info>,

//     pub token_program: Program<'info, Token>,
//     pub system_program: Program<'info, System>,
// }