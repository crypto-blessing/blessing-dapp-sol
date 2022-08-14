use crate::errors::CryptoBlessingError;
use crate::state::sender_blessing::*;
use crate::state::claimer_blessing::*;
use anchor_lang::prelude::*;
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

    ctx.accounts.claimer_blessing.save(*claimer.key, sender_blessing.sender, sender_blessing.blessing_id, sender_blessing.blessing_img.clone(), 1, 2)
}


#[derive(Accounts)]
pub struct ClaimBlessing<'info> {
    #[account(init, payer = claimer, space = ClaimerBlessing::LEN + 8)]
    pub claimer_blessing: Account<'info, ClaimerBlessing>,
    #[account(mut)]
    pub claimer: Signer<'info>,
    #[account(mut)]
    pub sender_blessing: Account<'info, SenderBlessing>,
    /// CHECK:
    #[account(mut)]
    pub blessing_owner: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}