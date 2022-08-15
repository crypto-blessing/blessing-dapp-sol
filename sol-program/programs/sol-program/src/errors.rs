use anchor_lang::error_code;

#[error_code]
pub enum CryptoBlessingError {

    #[msg("Can not find this blessing.")]
    BlessingNotFound,

    #[msg("This blessing is deleted.")]
    BlessingDeleted,

    #[msg("Blessing price is zero.")]
    BlessingPriceZero,

    #[msg("Blessing owner not match.")]
    BlessingOwnerNotMatch,

    #[msg("Blessing claim quantity not match.")]
    BlessingClaimQuantityNotMatch,

    #[msg("claime quantity > 0 && <= 13.")]
    ClaimeQuantityGT0LTE13,

    #[msg("Admin Param already inited.")]
    AdminParamAlreadyInited,

    #[msg("Admin Param not inited.")]
    AdminParamNotInited,

    #[msg("Admin Param owner not match.")]
    AdminCanDoThis,

    #[msg("No claim keys found.")]
    NoKeysFound,

    #[msg("Blessing revoked.")]
    BlessingRevoked,

    #[msg("Claim key verify faild.")]
    ClaimKeyVerifyFailed,

    #[msg("No blessing left.")]
    NoBlessingLeft,

    #[msg("You have already claim this blessing yet.")]
    RepeatClaimErr,

    #[msg("Can not revoke because blessing is claiming.")]
    BlessingClaimingErr,
}
