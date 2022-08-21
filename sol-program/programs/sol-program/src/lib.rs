use anchor_lang::prelude::*;
use instructions::*;
use state::*;

pub mod errors;
pub mod instructions;
pub mod state;

declare_id!("2tHZEpRNAW7a4NLqiyYC6G9xwySHbDs4CS6SnAiHux4F");

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

    pub fn update_blessing(ctx: Context<UpdateBlessing>, 
        image: String, 
        owner_id: Pubkey,
        price: u64,
        tax_rate: u16,
        ipfs: String,
    ) -> Result<()> {
        instructions::update_blessing::update_blessing(ctx, image, owner_id, price, tax_rate, ipfs)
    }

    pub fn send_blessing(ctx: Context<SendBlessing>, 
        token_amount: u64,
        claim_quantity: u64,
        claim_type: ClaimType,
        keys: Vec<Pubkey>,
    ) -> Result<()> {
        // find the blessing in account
        instructions::send_blessing::send_blessing(ctx, token_amount, claim_quantity, claim_type, keys)
    }

    pub fn revoke_blessing(
        ctx: Context<RevokeBlessing>,
    ) -> Result<()> {
        instructions::revoke_blessing::revoke_blessing(ctx)
    }

    pub fn claim_blessing(ctx: Context<ClaimBlessing>, 
        blessing_title: String,
        claim_key: String
    ) -> Result<()> {
        instructions::claim_blessing::claim_blessing(ctx, blessing_title, claim_key)
    }

    pub fn claim_blessing_with_new_claimer(ctx: Context<ClaimBlessingWithNewClaimer>, 
        blessing_title: String,
        claim_key: String
    ) -> Result<()> {
        instructions::claim_blessing::claim_blessing_with_new_claimer(ctx, blessing_title, claim_key)
    }

}


