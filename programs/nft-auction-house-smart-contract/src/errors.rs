use anchor_lang::prelude::*;

#[error_code]
pub enum NftAuctionError {
    #[msg("The randomness is still being processed.")]
    StillProcessing,

    #[msg("Missing Creation Vault Ata")]
    MissingVaultAta,

    #[msg("Missing Creation User Ata")]
    MissingUserAta,

    #[msg("Empty NFT data provided")]
    EmptyNftData,

    #[msg("Too many NFTs to mint in single transaction")]
    TooManyNfts,

    #[msg("Invalid count provided")]
    InvalidCount,

    #[msg("Serialize Error")]
    SerializeError,

    #[msg("CPI Failed")]
    CpiFailed,

    #[msg("Insufficient funds in vault USDC account")]
    InsufficientFunds,

    #[msg("NFT list is full")]
    NftListFull,
}

#[error_code]
pub enum CustomError {
    #[msg("Insufficient funds in user USDC account")]
    InsufficientFunds,

    #[msg("Invalid Pack Id")]
    InvalidPackId,

    #[msg("Invalid USDC Mint")]
    InvalidUsdcMint,
}
