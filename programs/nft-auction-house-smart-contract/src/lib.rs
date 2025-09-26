use anchor_lang::prelude::*;

pub mod constants;
pub mod errors;
pub mod instructions;
pub mod misc;
pub mod state;
pub mod utils;

use crate::state::TransferArgs;
use instructions::*;

declare_id!("988hJvsyGwL2TM5WRGKdAxWAUvWx97dDQANPiRt5Mukn");

#[program]
pub mod nft_auction_house_smart_contract {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        platform_vault: Pubkey,
        fee_percentage: u16
    ) -> Result<()> {
        initialize::initialize(ctx, platform_vault, fee_percentage)?;
        Ok(())
    }

    // Create collection NFT (call this once per collection)
    pub fn create_collection(
        ctx: Context<CreateCollection>,
        name: String,
        symbol: String,
        uri: String
    ) -> Result<()> {
        mint_nft::create_collection(ctx, name, symbol, uri)?;
        Ok(())
    }

    // Mint cNFT to existing collection (can be called multiple times)
    pub fn mint_nft(
        ctx: Context<MintCnft>,
        name: String,
        symbol: String,
        uri: String
    ) -> Result<()> {
        mint_nft::cpi_mint_cnft(ctx, name, symbol, uri)?;
        Ok(())
    }

    pub fn buy_pack(
        ctx: Context<BuyPack>,
        force: [u8; 32],
        pack_id: u8,
        pack_count: u8
    ) -> Result<()> {
        buy_pack::buy_pack(ctx, force, pack_id, pack_count)?;
        Ok(())
    }

    pub fn fulfill_random(ctx: Context<FulfillRandom>, force: [u8; 32]) -> Result<()> {
        fulfill_random::fulfill_random(ctx, force)?;
        Ok(())
    }

    pub fn transfer(ctx: Context<Transfer>, transfer_args: TransferArgs) -> Result<()> {
        transfer::transfer(ctx, transfer_args)?;
        Ok(())
    }

    pub fn instant_sell(
        ctx: Context<InstantSell>,
        pack_id: u8,
        transfer_args: TransferArgs
    ) -> Result<()> {
        instant_sell::instant_sell(ctx, pack_id, transfer_args)?;
        Ok(())
    }

    pub fn list(
        ctx: Context<ListCompressedNft>,
        nonce: u64,
        index: u32,
        root: [u8; 32],
        data_hash: [u8; 32],
        creator_hash: [u8; 32],
        amount: u64,
        expire_in_sec: Option<u64>,
        currency: Option<Pubkey>,
        private_taker: Option<Pubkey>,
        maker_broker: Option<Pubkey>
    ) -> Result<()> {
        list::list_compressed_nft(
            ctx,
            nonce,
            index,
            root,
            data_hash,
            creator_hash,
            amount,
            expire_in_sec,
            currency,
            private_taker,
            maker_broker
        )?;
        Ok(())
    }

    pub fn delist(
        ctx: Context<DelistCompressedNft>,
        delist_args: DelistCompressedNftArgs
    ) -> Result<()> {
        delist::delist_compressed_nft(ctx, delist_args)?;
        Ok(())
    }
}
