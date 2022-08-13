use anchor_lang::prelude::*;
use instructions::*;
use state::*;

pub mod errors;
pub mod instructions;
pub mod state;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod sol_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::op_admin_param::initialize(ctx)
    }

    pub fn update_admin_param(ctx: Context<UpdateAdminParam>,
        cbt_reward_ratio: u16,
        cbt_reward_max: u64,
        claim_tax_rate: u16,
    ) -> Result<()> {
        instructions::op_admin_param::update_admin_param(ctx, cbt_reward_ratio, cbt_reward_max, claim_tax_rate)
    }

    pub fn add_blessing(ctx: Context<AddBlessing>, 
        image: String, 
        owner_id: Pubkey,
        price: u64,
        tax_rate: u16,
        ipfs: String,
    ) -> Result<()> {
        instructions::add_blessing::add_blessing(ctx, image, owner_id, price, tax_rate, ipfs)
    }

    pub fn send_blessing(ctx: Context<SendBlessing>, 
        blessing_id: Pubkey, 
        blessing_img: String,
        token_amount: u64,
        claim_quantity: u64,
        claim_type: ClaimType,
        keys: Vec<Pubkey>,
    ) -> Result<()> {
        // find the blessing in account
        // let blessing: Account<Blessing> = ctx.accounts.blessing;
        // require!(blessing, ErrorCode::BlessingNotFound);
        // require!(blessing.deleted == false, ErrorCode::BlessingDeleted);
        // require!(blessing.price > 0, ErrorCode::BlessingPriceZero);
        instructions::send_blessing::send_blessing(ctx, blessing_id, blessing_img, token_amount, claim_quantity, claim_type, keys)
    }
}


