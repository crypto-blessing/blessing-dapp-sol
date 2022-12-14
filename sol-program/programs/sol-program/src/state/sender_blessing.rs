use anchor_lang::prelude::*;


#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Copy)]
pub enum ClaimType {
    Average,
    Random,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub struct ClaimKey {
    pub key: String, // 4+65*4
    pub used: bool, // 1
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Copy)]
pub struct ClaimerInfo {
    pub claimer: Pubkey, // 32
    pub claim_timestamp: i64,  //8
    pub distributed_amount: u64,    // 8
    pub claim_amount: u64,  // 8
    pub tax_amount: u64,    // 8
    pub cbt_token_reward_to_sender_amount: u64, // 8
}

#[account]
pub struct SenderBlessing {
    pub sender: Pubkey, // 32
    pub blessing_id: Pubkey, //32
    pub blessing_img: String, // 4 + 256 * 4
    pub send_timestamp: i64,  //8
    pub token_amount: u64,  //8 
    pub claim_quantity: u64,     //8
    pub claim_type: ClaimType,  //1
    pub revoked: bool,  //1
    pub claim_keys: Vec<ClaimKey>, // 4 + (4+ 65 * 4 + 1) * 13
    pub claimer_list: Vec<ClaimerInfo>, // 4 + 72 * 13
}

impl SenderBlessing {

    pub const LEN: usize = 32 + 32 + 4 + 256 * 4 + 8 + 8 + 8 + 1 + 1 + 4 + (4+ 65 * 4 + 1) * 13 + 4 + 72 * 13;

    pub fn save(&mut self, 
        sender: Pubkey,
        blessing_id: Pubkey,
        blessing_img: String,
        token_amount: u64,
        claim_quantity: u64,
        claim_type: ClaimType,
        claim_keys: Vec<ClaimKey>,
    ) -> Result<()> {
        let clock: Clock = Clock::get().unwrap();
        self.sender = sender;
        self.blessing_id = blessing_id;
        self.blessing_img = blessing_img;
        self.send_timestamp = clock.unix_timestamp;
        self.token_amount = token_amount;
        self.claim_quantity = claim_quantity;
        self.claim_type = claim_type;
        self.revoked = false;
        self.claim_keys = claim_keys;
        Ok(())
    }

    pub fn revoke_blessing(&mut self) -> Result<()> {
        self.revoked = true;
        Ok(())
    }

    pub fn insert_claimer_list(&mut self, claimer: Pubkey, claim_timestamp: i64, distributed_amount: u64, claim_amount: u64, tax_amount: u64, cbt_token_reward_to_sender_amount: u64) {
        msg!("insert claimer list");
        self.claimer_list.push(ClaimerInfo {
            claimer,
            claim_timestamp,
            distributed_amount,
            claim_amount,
            tax_amount,
            cbt_token_reward_to_sender_amount,
        });
    }

    pub fn save_claim_key_used(&mut self, key: String) {
        for claim_key in self.claim_keys.iter_mut() {
            if claim_key.key == key {
                claim_key.used = true;
                break;
            }
        }
    }


}