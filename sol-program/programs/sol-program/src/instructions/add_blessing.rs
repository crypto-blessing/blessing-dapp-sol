use crate::errors::CryptoBlessingError;
use crate::state::blessing::*;
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
    
    ctx.accounts.blessing.save(image, owner_id, price, tax_rate, ipfs)
}

#[derive(Accounts)]
pub struct AddBlessing<'info> {
    #[account(init, payer = owner, space = Blessing::LEN + 8)]
    pub blessing: Account<'info, Blessing>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}