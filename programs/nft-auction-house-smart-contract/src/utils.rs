use anchor_lang::prelude::*;
use anchor_lang::solana_program::hash::hash;
use crate::state::MetadataEntry;

pub fn extract_asset_data_from_mint(
    merkle_tree: &Pubkey,
    leaf_owner: &Pubkey,
    metadata_args: &crate::instructions::mint_nft::MetadataArgs
) -> Result<MetadataEntry> {
    let merkle_bytes = merkle_tree.to_bytes();
    let owner_bytes = leaf_owner.to_bytes();

    let mut leaf_index_bytes = [0u8; 8];
    for i in 0..8 {
        leaf_index_bytes[i] = merkle_bytes[i] ^ owner_bytes[i];
    }
    let leaf_index = u64::from_le_bytes(leaf_index_bytes);

    let leaf_index = if leaf_index == 0 { 1 } else { leaf_index };

    let (asset_id, _bump) = Pubkey::find_program_address(
        &[b"asset", merkle_tree.as_ref(), &leaf_index.to_le_bytes()],
        &anchor_spl::token::ID
    );

    msg!("DEBUG: Generated Asset ID: {}", asset_id);
    msg!("DEBUG: Leaf index: {}", leaf_index);

    Ok(MetadataEntry {
        asset_id,
        name: metadata_args.name.clone(),
        symbol: metadata_args.symbol.clone(),
        uri: metadata_args.uri.clone(),
    })
}
