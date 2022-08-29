use crate::errors::CryptoBlessingError;
use crate::state::sender_blessing::*;
use crate::state::claimer_blessing::*;
use sha256::*;
use {
    anchor_lang::{
        prelude::*,
        solana_program::program::invoke,
        system_program,
    },
    anchor_spl::{
        associated_token,
        token,
    },
    mpl_token_metadata::{
        ID as TOKEN_METADATA_ID,
        instruction as token_instruction,
    },
};

fn random_num() -> u64 {
    let clock: Clock = Clock::get().unwrap();
    let mut random = (clock.unix_timestamp % 10) as u64;
    if random == 0 || random == 10 {
        random = 1;
    }
    return random;
}

pub fn claim_blessing(ctx: Context<ClaimBlessing>, 
    metadata_title: String, 
    metadata_uri: String,
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
        require_neq!(*ctx.accounts.mint_authority.key, status.claimer, CryptoBlessingError::RepeatClaimErr);
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

    // let mut cbt_token_reward = distribution_amount * ctx.accounts.admin_param.cbt_reward_ratio as u64;
    // if cbt_token_reward > ctx.accounts.admin_param.cbt_reward_max {
    //     cbt_token_reward = ctx.accounts.admin_param.cbt_reward_max;
    // }
    
    // update the sender_blessing
    ctx.accounts.sender_blessing.save_claim_key_used(claim_key_finded);
    let clock: Clock = Clock::get().unwrap();
    ctx.accounts.sender_blessing.insert_claimer_list(*ctx.accounts.mint_authority.key, clock.unix_timestamp, distributed_amount, 
        distribution_amount,
        0,
        0,);
    

    // mint the NFT token to claimer
    msg!("Creating mint account...");
    msg!("Mint: {}", &ctx.accounts.mint.key());
    system_program::create_account(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            system_program::CreateAccount {
                from: ctx.accounts.mint_authority.to_account_info(),
                to: ctx.accounts.mint.to_account_info(),
            },
        ),
        10000000,
        82,
        &ctx.accounts.token_program.key(),
    )?;

    msg!("Initializing mint account...");
    msg!("Mint: {}", &ctx.accounts.mint.key());
    token::initialize_mint(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::InitializeMint {
                mint: ctx.accounts.mint.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
        ),
        0,
        &ctx.accounts.mint_authority.key(),
        Some(&ctx.accounts.mint_authority.key()),
    )?;

    msg!("Creating token account...");
    msg!("Token Address: {}", &ctx.accounts.token_account.key());    
    associated_token::create(
        CpiContext::new(
            ctx.accounts.associated_token_program.to_account_info(),
            associated_token::Create {
                payer: ctx.accounts.mint_authority.to_account_info(),
                associated_token: ctx.accounts.token_account.to_account_info(),
                authority: ctx.accounts.mint_authority.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
        ),
    )?;

    msg!("Minting token to token account...");
    msg!("Mint: {}", &ctx.accounts.mint.to_account_info().key());   
    msg!("Token Address: {}", &ctx.accounts.token_account.key());     
    token::mint_to(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::MintTo {
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.token_account.to_account_info(),
                authority: ctx.accounts.mint_authority.to_account_info(),
            },
        ),
        1,
    )?;

    msg!("Creating metadata account...");
    msg!("Metadata account address: {}", &ctx.accounts.metadata.to_account_info().key());
    invoke(
        &token_instruction::create_metadata_accounts_v2(
            TOKEN_METADATA_ID, 
            ctx.accounts.metadata.key(), 
            ctx.accounts.mint.key(), 
            ctx.accounts.mint_authority.key(), 
            ctx.accounts.mint_authority.key(), 
            ctx.accounts.mint_authority.key(), 
            metadata_title, 
            "CBNFT".to_string(), 
            metadata_uri, 
            None,
            1,
            true, 
            false, 
            None, 
            None,
        ),
        &[
            ctx.accounts.metadata.to_account_info(),
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.token_account.to_account_info(),
            ctx.accounts.mint_authority.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ],
    )?;

    msg!("Creating master edition metadata account...");
    msg!("Master edition metadata account address: {}", &ctx.accounts.master_edition.to_account_info().key());
    invoke(
        &token_instruction::create_master_edition_v3(
            TOKEN_METADATA_ID, 
            ctx.accounts.master_edition.key(), 
            ctx.accounts.mint.key(), 
            ctx.accounts.mint_authority.key(), 
            ctx.accounts.mint_authority.key(), 
            ctx.accounts.metadata.key(), 
            ctx.accounts.mint_authority.key(), 
            Some(0),
        ),
        &[
            ctx.accounts.master_edition.to_account_info(),
            ctx.accounts.metadata.to_account_info(),
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.token_account.to_account_info(),
            ctx.accounts.mint_authority.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ],
    )?;

    msg!("Token mint process completed successfully.");

    **ctx.accounts.sender_blessing.to_account_info().try_borrow_mut_lamports()? -= distribution_amount;
    **ctx.accounts.mint_authority.to_account_info().try_borrow_mut_lamports()? += distribution_amount;
    
    ctx.accounts.claimer_blessing.save(*ctx.accounts.mint_authority.key, ctx.accounts.sender_blessing.sender, ctx.accounts.sender_blessing.blessing_id, 
        ctx.accounts.sender_blessing.blessing_img.clone(), 
        distribution_amount,
        0)

}


#[derive(Accounts)]
pub struct ClaimBlessing<'info> {
    #[account(init, payer = mint_authority, space = ClaimerBlessing::LEN + 8)]
    pub claimer_blessing: Account<'info, ClaimerBlessing>,
    // #[account(mut)]
    // pub claimer: Signer<'info>,
    #[account(mut)]
    pub sender_blessing: Account<'info, SenderBlessing>,
    // pub blessing: Account<'info, Blessing>,
    // pub admin_param: Account<'info, AdminParam>,
    // /// CHECK: This is not dangerous because we don't read or write from this account
    // #[account(mut)]
    // pub program_owner: AccountInfo<'info>,
    // /// CHECK: This is not dangerous because we don't read or write from this account
    // #[account(mut)]
    // pub sender: AccountInfo<'info>,

    pub system_program: Program<'info, System>,

    /// CHECK: We're about to create this with Metaplex
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,

    /// CHECK: We're about to create this with Metaplex
    #[account(mut)]
    pub master_edition: UncheckedAccount<'info>,

    #[account(mut)]
    pub mint: Signer<'info>,
    /// CHECK: We're about to create this with Anchor
    #[account(mut)]
    pub token_account: UncheckedAccount<'info>,
    #[account(mut)]
    pub mint_authority: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, token::Token>,
    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
    /// CHECK: Metaplex will check this
    pub token_metadata_program: UncheckedAccount<'info>,
    
}