use anchor_lang::prelude::*;

#[account]
pub struct Blessing {
    pub image: String,   // 4 + 64 * 4
    pub owner_id: Pubkey,   // 32
    pub price: u64,         // 8
    pub deleted: bool,    // 1
    pub tax_rate: u16,  // 2
    pub ipfs: String,   // 4 + 256 * 4
}

impl Blessing {
    
    pub const LEN: usize = 4 + 64 * 4 + 8 + 32 + 1 + 2 + 4 + 256 * 4;


    pub fn save(&mut self, 
        image: String, 
        owner_id: Pubkey,
        price: u64,
        tax_rate: u16,
        ipfs: String,
    ) -> Result<()> {
        self.image = image;
        self.price = price;
        self.owner_id = owner_id;
        self.tax_rate = tax_rate;
        self.deleted = false;
        self.ipfs = ipfs;
        Ok(())
    }

}
