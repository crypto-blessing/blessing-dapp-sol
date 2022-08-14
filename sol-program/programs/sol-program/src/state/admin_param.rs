use anchor_lang::prelude::*;

#[account]
pub struct AdminParam {
    pub program_owner: Pubkey,   // 32
    pub cbt_reward_ratio: u16,   // 2
    pub cbt_reward_max: u64,         // 8
    pub claim_tax_rate: u16,    // 2
    pub inited: bool,   // 1
}

impl AdminParam {
    pub const LEN: usize = 32 + 2 + 8 + 2 + 1;

    pub fn save(&mut self, 
        program_owner: Pubkey,
        cbt_reward_ratio: u16,
        cbt_reward_max: u64,
        claim_tax_rate: u16,
    ) -> Result<()> {
        self.program_owner = program_owner;
        self.cbt_reward_ratio = cbt_reward_ratio;
        self.cbt_reward_max = cbt_reward_max;
        self.claim_tax_rate = claim_tax_rate;
        self.inited = true;
        Ok(())
    }
}