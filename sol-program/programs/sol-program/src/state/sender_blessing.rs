use anchor_lang::prelude::*;


#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum ClaimType {
    Avagege,
    Random,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub struct ClaimKey {
    pub key: Pubkey, // 32
    pub used: bool, // 1
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
    pub claim_keys: Vec<ClaimKey>, // 4 + 32 * 13
}

impl SenderBlessing {

    pub const LEN: usize = 32 + 32 + 4 + 256 * 4 + 8 + 8 + 8 + 1 + 1;

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


}