use crate::errors::CryptoBlessingError;
use crate::state::admin_param::*;
use anchor_lang::prelude::*;

pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    let owner: &Signer = &ctx.accounts.owner;
    let admin_param = &ctx.accounts.admin_param;
    require!(admin_param.inited == false, CryptoBlessingError::AdminParamAlreadyInited);
    ctx.accounts.admin_param.save(*owner.key, 5, 0, 10)
}

pub fn update_admin_param(ctx: Context<UpdateAdminParam>,
    cbt_reward_ratio: u16,
    cbt_reward_max: u64,
    claim_tax_rate: u16,
    ) -> Result<()> {
    let owner: &Signer = &ctx.accounts.program_owner;
    let admin_param = &ctx.accounts.admin_param;
    require!(admin_param.inited == true, CryptoBlessingError::AdminParamNotInited);
    ctx.accounts.admin_param.save(*owner.key, cbt_reward_ratio, cbt_reward_max, claim_tax_rate)
}


#[derive(Accounts)]
pub struct Initialize<'info> {

    #[account(init, payer = owner, space = AdminParam::LEN + 8)]
    pub admin_param: Account<'info, AdminParam>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,

}

#[derive(Accounts)]
pub struct UpdateAdminParam<'info> {

    #[account(mut, has_one = program_owner)]
    pub admin_param: Account<'info, AdminParam>,
    pub program_owner: Signer<'info>,
}