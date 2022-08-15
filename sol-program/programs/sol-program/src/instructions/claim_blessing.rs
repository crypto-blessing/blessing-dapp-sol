use anchor_lang::prelude::*;
use anchor_spl::token;
use anchor_lang::solana_program::program::invoke;
use anchor_spl::{token::{MintTo, Token, Transfer}};
use mpl_token_metadata::instruction::{create_master_edition_v3, create_metadata_accounts_v2};
use crate::errors::CryptoBlessingError;
use crate::state::blessing::*;
use crate::state::sender_blessing::*;
use crate::state::claimer_blessing::*;
use crate::state::AdminParam;
use sha256::*;

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
    let sender_blessing = &mut ctx.accounts.sender_blessing;
    require_eq!(sender_blessing.revoked, false, CryptoBlessingError::BlessingRevoked);
    let claimer: &Signer = &ctx.accounts.claimer;
    let hex = digest(claim_key);
    let claim_keys =  &mut sender_blessing.claim_keys;
    require_gt!(claim_keys.len(), 0, CryptoBlessingError::NoKeysFound);
    let mut hex_finded = false;
    for claim_key in claim_keys {
        if hex == claim_key.key {
            hex_finded = true;
            claim_key.used = true;
        }
    }
    require_eq!(hex_finded, true, CryptoBlessingError::ClaimKeyVerifyFailed);
    require_gt!(sender_blessing.claim_quantity as usize, sender_blessing.claimer_list.len(), CryptoBlessingError::NoBlessingLeft);

    // cal the claim amount
    let mut distributed_amount = 0;
    for status in sender_blessing.claimer_list.iter() {
        require_neq!(*claimer.key, status.claimer, CryptoBlessingError::RepeatClaimErr);
        distributed_amount += status.distributed_amount;
    }
    let mut distribution_amount = 0;
    match sender_blessing.claim_type {
        ClaimType::Average=>{
            distribution_amount = sender_blessing.token_amount / sender_blessing.claim_quantity;
        },
        ClaimType::Random => {
            let left_quantity = sender_blessing.claim_quantity - sender_blessing.claimer_list.len() as u64;
            let random_num = random_num();
            if left_quantity == 1 {
                distribution_amount = sender_blessing.token_amount - distributed_amount;
            } else {
                distribution_amount = (sender_blessing.token_amount - distributed_amount) / left_quantity * random_num / 10 * 2;
            }
        }, 
    }

    let admin_param = &mut ctx.accounts.admin_param;

    let mut cbt_token_reward = distribution_amount * admin_param.cbt_reward_ratio as u64;
    if cbt_token_reward > admin_param.cbt_reward_max {
        cbt_token_reward = admin_param.cbt_reward_max;
    }

    let clock: Clock = Clock::get().unwrap();
    sender_blessing.claimer_list.push(ClaimerInfo {
        claimer: *claimer.key,
        distributed_amount: distribution_amount,
        claim_timestamp: clock.unix_timestamp,
        claim_amount: distribution_amount / 1000 * (1000 - admin_param.claim_tax_rate as u64),
        tax_amount: distribution_amount / 1000 * admin_param.claim_tax_rate as u64,
        cbt_token_reward_to_sender_amount: cbt_token_reward,
    });

    // transfer the blessing token to sender_blessing account
    let ix = anchor_lang::solana_program::system_instruction::transfer(
        &sender_blessing.key(),
        &claimer.key,
        distribution_amount / 1000 * (1000 - admin_param.claim_tax_rate as u64),
    );
    anchor_lang::solana_program::program::invoke(
        &ix,
        &[
            sender_blessing.to_account_info(),
            claimer.to_account_info(),
        ],
    ).expect("transfer to claimer failed");

    let ix = anchor_lang::solana_program::system_instruction::transfer(
        &sender_blessing.key(),
        &ctx.accounts.program_owner.key,
        distribution_amount / 1000 * admin_param.claim_tax_rate as u64,
    );
    anchor_lang::solana_program::program::invoke(
        &ix,
        &[
            sender_blessing.to_account_info(),
            ctx.accounts.program_owner.to_account_info(),
        ],
    ).expect("transfer to program owner failed");

    // transfer the cbt token to sender
    let sender_blessing_pk = sender_blessing.key().clone();
    let inner = vec![
        b"state".as_ref(),
        sender_blessing_pk.as_ref(),
        ctx.accounts.sender.key.as_ref(),
    ];
    let outer = vec![inner.as_slice()];
    let transfer_instruction = Transfer{
        from: sender_blessing.to_account_info(),
        to: ctx.accounts.sender.to_account_info(),
        authority: sender_blessing.to_account_info(),
    };
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        transfer_instruction,
        outer.as_slice(),
    );
    anchor_spl::token::transfer(cpi_ctx, cbt_token_reward)?;

    // mint the NFT token to claimer
    let cpi_accounts = MintTo {
        mint: ctx.accounts.mint.to_account_info(),
        to: ctx.accounts.token_account.to_account_info(),
        authority: ctx.accounts.payer.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::mint_to(cpi_ctx, 1)?;
    let account_info = vec![
            ctx.accounts.metadata.to_account_info(),
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.program_owner.to_account_info(),
            ctx.accounts.payer.to_account_info(),
            ctx.accounts.token_metadata_program.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ];

    let creator = vec![
        mpl_token_metadata::state::Creator {
            address: *claimer.key,
            verified: false,
            share: 1,
        },
        mpl_token_metadata::state::Creator {
            address: ctx.accounts.program_owner.key(),
            verified: false,
            share: 0,
        },
    ];
    let symbol = std::string::ToString::to_string("CBNFT");
    invoke(
        &create_metadata_accounts_v2(
            ctx.accounts.token_metadata_program.key(),
            ctx.accounts.metadata.key(),
            ctx.accounts.mint.key(),
            ctx.accounts.program_owner.key(),
            ctx.accounts.payer.key(),
            ctx.accounts.payer.key(),
            blessing_title,
            symbol,
            ctx.accounts.blessing.ipfs,
            Some(creator),
            1,
            true,
            false,
            None,
            None,
        ),
        account_info.as_slice(),
    )?;
    let master_edition_infos = vec![
        ctx.accounts.master_edition.to_account_info(),
        ctx.accounts.mint.to_account_info(),
        ctx.accounts.program_owner.to_account_info(),
        ctx.accounts.payer.to_account_info(),
        ctx.accounts.metadata.to_account_info(),
        ctx.accounts.token_metadata_program.to_account_info(),
        ctx.accounts.token_program.to_account_info(),
        ctx.accounts.system_program.to_account_info(),
    ];
    invoke(
        &create_master_edition_v3(
            ctx.accounts.token_metadata_program.key(),
            ctx.accounts.master_edition.key(),
            ctx.accounts.mint.key(),
            ctx.accounts.payer.key(),
            ctx.accounts.program_owner.key(),
            ctx.accounts.metadata.key(),
            ctx.accounts.payer.key(),
            Some(0),
        ),
        master_edition_infos.as_slice(),
    )?;

    ctx.accounts.claimer_blessing.save(*claimer.key, sender_blessing.sender, sender_blessing.blessing_id, 
        sender_blessing.blessing_img.clone(), 
        distribution_amount / 1000 * (1000 - admin_param.claim_tax_rate as u64),
        distribution_amount / 1000 * admin_param.claim_tax_rate as u64)
}


#[derive(Accounts)]
pub struct ClaimBlessing<'info> {
    #[account(init, payer = claimer, space = ClaimerBlessing::LEN + 8)]
    pub claimer_blessing: Account<'info, ClaimerBlessing>,
    #[account(mut)]
    pub claimer: Signer<'info>,
    #[account(mut)]
    pub sender_blessing: Account<'info, SenderBlessing>,
    pub blessing: Account<'info, Blessing>,
    /// CHECK:
    #[account(mut)]
    pub blessing_owner: AccountInfo<'info>,
    pub admin_param: Account<'info, AdminParam>,
    /// CHECK:
    #[account(mut)]
    pub program_owner: AccountInfo<'info>,
    #[account(mut)]
    pub sender: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub mint: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub token_account: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub payer: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_metadata_program: UncheckedAccount<'info>,
    #[account(mut)]
    pub master_edition: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ClaimBlessingWithNewClaimer<'info> {
    #[account(init, payer = sender_blessing, space = 8)]
    pub claimer: AccountInfo<'info>,

    #[account(init, payer = claimer, space = ClaimerBlessing::LEN + 8)]
    pub claimer_blessing: Account<'info, ClaimerBlessing>,
    #[account(mut)]
    pub sender_blessing: Account<'info, SenderBlessing>,
    pub blessing: Account<'info, Blessing>,
    /// CHECK:
    #[account(mut)]
    pub blessing_owner: AccountInfo<'info>,
    pub admin_param: Account<'info, AdminParam>,
    /// CHECK:
    #[account(mut)]
    pub program_owner: AccountInfo<'info>,
    #[account(mut)]
    pub sender: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub mint: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub token_account: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub payer: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_metadata_program: UncheckedAccount<'info>,
    #[account(mut)]
    pub master_edition: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}