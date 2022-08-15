use crate::errors::CryptoBlessingError;
use crate::state::sender_blessing::*;
use anchor_lang::prelude::*;

pub fn revoke_blessing(ctx: Context<RevokeBlessing>) -> Result<()> {
    require_eq!(ctx.accounts.sender_blessing.revoked, false, CryptoBlessingError::BlessingRevoked);
    require_eq!(ctx.accounts.sender_blessing.claimer_list.len(), 0, CryptoBlessingError::BlessingClaimingErr);
    // transfer the blessing token back to sender account
    let ix = anchor_lang::solana_program::system_instruction::transfer(
        &ctx.accounts.sender_blessing.key(),
        &ctx.accounts.sender.key(),
        ctx.accounts.sender_blessing.token_amount,
    );
    anchor_lang::solana_program::program::invoke(
        &ix,
        &[
            ctx.accounts.sender_blessing.to_account_info(),
            ctx.accounts.sender.to_account_info(),
        ],
    ).expect("transfer to sender blessing account failed");
    ctx.accounts.sender_blessing.revoke_blessing()
}


#[derive(Accounts)]
pub struct RevokeBlessing<'info> {
    #[account(mut)]
    pub sender_blessing: Account<'info, SenderBlessing>,
    #[account(mut)]
    pub sender: Signer<'info>,
}