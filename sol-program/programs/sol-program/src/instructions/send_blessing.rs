use crate::errors::CryptoBlessingError;
use crate::state::sender_blessing::*;
use crate::state::blessing::*;
use anchor_lang::prelude::*;



pub fn send_blessing(ctx: Context<SendBlessing>, 
    token_amount: u64,
    claim_quantity: u64,
    claim_type: ClaimType,
    keys: Vec<Pubkey>,
) -> Result<()> {
    // find the blessing in account
    let blessing= &mut ctx.accounts.blessing;
    require_gt!(blessing.price, 0 , CryptoBlessingError::BlessingPriceZero);
    require_neq!(blessing.deleted, true, CryptoBlessingError::BlessingDeleted);
    require_eq!(blessing.owner_id, ctx.accounts.blessing_owner.key(), CryptoBlessingError::BlessingOwnerNotMatch);
    require_eq!(claim_quantity as usize, keys.len(), CryptoBlessingError::BlessingClaimQuantityNotMatch);
    require_gt!(token_amount, 0, CryptoBlessingError::BlessingPriceZero);
    require_gt!(claim_quantity, 0, CryptoBlessingError::ClaimeQuantityGT0LTE13);
    require!(claim_quantity <= 13, CryptoBlessingError::ClaimeQuantityGT0LTE13);
    
    let sender: &Signer = &ctx.accounts.sender;

    // transfer the blessing token to sender_blessing account
    let ix = anchor_lang::solana_program::system_instruction::transfer(
        &sender.key(),
        &ctx.accounts.sender_blessing.key(),
        token_amount,
    );
    anchor_lang::solana_program::program::invoke(
        &ix,
        &[
            sender.to_account_info(),
            ctx.accounts.sender_blessing.to_account_info(),
        ],
    ).expect("transfer to sender blessing account failed");

    // transfer the blessing price to the owner of each blessing
    let ix = anchor_lang::solana_program::system_instruction::transfer(
        &sender.key(),
        &ctx.accounts.blessing_owner.key(),
        blessing.price * claim_quantity,
    );
    anchor_lang::solana_program::program::invoke(
        &ix,
        &[
            sender.to_account_info(),
            ctx.accounts.blessing_owner.to_account_info(),
        ],
    ).expect("transfer to the owner of blessing failed");

    // set claime keys
    let mut claim_keys = vec![];
    for key in keys {
        claim_keys.push(ClaimKey {
            key: key,
            used: false,
        });
    }

    ctx.accounts.sender_blessing.save(*sender.key, blessing.key(), blessing.image.clone(), token_amount, claim_quantity, claim_type, claim_keys)
}


#[derive(Accounts)]
pub struct SendBlessing<'info> {
    #[account(init, payer = sender, space = SenderBlessing::LEN + 8)]
    pub sender_blessing: Account<'info, SenderBlessing>,
    #[account(mut)]
    pub sender: Signer<'info>,
    pub blessing: Account<'info, Blessing>,
    /// CHECK:
    #[account(mut)]
    pub blessing_owner: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}