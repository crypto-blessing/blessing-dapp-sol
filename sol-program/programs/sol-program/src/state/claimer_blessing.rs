use anchor_lang::prelude::*;

#[account]
pub struct ClaimerBlessing {
    pub claimer: Pubkey,    // 32
    pub sender: Pubkey, // 32
    pub blessing_id: Pubkey, //32
    pub blessing_img: String, // 4 + 256 * 4
    pub claim_timestamp: i64,  //8
    pub claim_amount: u64,  //8 
    pub tax_amount: u64,  //8 
}

impl ClaimerBlessing {

    pub const LEN: usize = 32 + 32 + 32 + 4 + 256 * 4 + 8 + 8 + 8;

    pub fn save(&mut self, 
        claimer: Pubkey,
        sender: Pubkey,
        blessing_id: Pubkey,
        blessing_img: String,
        claim_amount: u64,
        tax_amount: u64,
    ) -> Result<()> {
        let clock: Clock = Clock::get().unwrap();
        self.claimer = claimer;
        self.sender = sender;
        self.blessing_id = blessing_id;
        self.blessing_img = blessing_img;
        self.claim_timestamp = clock.unix_timestamp;
        self.claim_amount = claim_amount;
        self.tax_amount = tax_amount;
        Ok(())
    }

}