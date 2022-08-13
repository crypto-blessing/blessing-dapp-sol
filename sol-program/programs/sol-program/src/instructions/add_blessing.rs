use crate::errors::CryptoBlessingError;
use crate::state::{blessing::*, AdminParam};
use anchor_lang::prelude::*;


pub fn add_blessing(ctx: Context<AddBlessing>, 
    image: String, 
    owner_id: Pubkey,
    price: u64,
    tax_rate: u16,
    ipfs: String,
) -> Result<()> {
    require_gt!(price, 0, CryptoBlessingError::BlessingPriceZero);
    let owner: &Signer = &ctx.accounts.owner;
    let admin_param = &ctx.accounts.admin_param;
    require_eq!(owner.key(), admin_param.program_owner, CryptoBlessingError::AdminCanDoThis);
    ctx.accounts.blessing.save(image, owner_id, price, tax_rate, ipfs)
}

#[derive(Accounts)]
pub struct AddBlessing<'info> {
    #[account(init, payer = owner, space = Blessing::LEN + 8)]
    pub blessing: Account<'info, Blessing>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub admin_param: Account<'info, AdminParam>,
    pub system_program: Program<'info, System>,
}