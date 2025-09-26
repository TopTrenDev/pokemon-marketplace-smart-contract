use anchor_lang::prelude::*;
use anchor_lang::solana_program::{ instruction::{ Instruction, AccountMeta }, program::invoke };
use crate::constants::TENSOR_CNFT_PROGRAM_ID;
use crate::errors::NftAuctionError;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct DelistCompressedNftArgs {
    pub nonce: u64,
    pub index: u32,
    pub root: [u8; 32],
    pub data_hash: [u8; 32],
    pub creator_hash: [u8; 32],
}

#[derive(Accounts)]
pub struct DelistCompressedNft<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub platform_vault: Signer<'info>,

    /// The owner/seller of the compressed NFT who wants to delist
    #[account(mut)]
    pub owner: Signer<'info>,

    /// CHECK: The merkle tree account that contains the compressed NFT
    #[account(mut)]
    pub merkle_tree: UncheckedAccount<'info>,

    /// CHECK: Tree authority for the merkle tree
    #[account(mut)]
    pub tree_authority: UncheckedAccount<'info>,

    /// CHECK: Listing state account (PDA owned by Tensor) - will be closed
    #[account(mut)]
    pub list_state: UncheckedAccount<'info>,

    /// CHECK: Rent destination account (where rent lamports will be returned)
    #[account(mut)]
    pub rent_destination: UncheckedAccount<'info>,

    /// CHECK: Bubblegum program for compressed NFT operations
    pub bubblegum_program: UncheckedAccount<'info>,

    /// CHECK: Compression program
    pub compression_program: UncheckedAccount<'info>,

    /// CHECK: Log wrapper program
    pub log_wrapper: UncheckedAccount<'info>,

    /// CHECK: Tensor swap program
    #[account(constraint = tensor_program.key() == TENSOR_CNFT_PROGRAM_ID)]
    pub tensor_program: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

pub fn delist_compressed_nft(
    ctx: Context<DelistCompressedNft>,
    args: DelistCompressedNftArgs
) -> Result<()> {
    let discriminator = anchor_lang::solana_program::hash::hash(b"global:delist").to_bytes();
    let mut data = discriminator[..8].to_vec();

    // Serialize the delist arguments
    #[derive(AnchorSerialize)]
    struct TensorDelistArgs {
        pub nonce: u64,
        pub index: u32,
        pub root: [u8; 32],
        pub data_hash: [u8; 32],
        pub creator_hash: [u8; 32],
    }

    let tensor_args = TensorDelistArgs {
        nonce: args.nonce,
        index: args.index,
        root: args.root,
        data_hash: args.data_hash,
        creator_hash: args.creator_hash,
    };

    data.extend(tensor_args.try_to_vec().map_err(|_| NftAuctionError::SerializeError)?);

    // Prepare accounts for the CPI call
    let accounts = vec![
        AccountMeta::new_readonly(ctx.accounts.tree_authority.key(), false),
        AccountMeta::new(ctx.accounts.merkle_tree.key(), false),
        AccountMeta::new_readonly(ctx.accounts.log_wrapper.key(), false),
        AccountMeta::new_readonly(ctx.accounts.compression_program.key(), false),
        AccountMeta::new_readonly(ctx.accounts.system_program.key(), false),
        AccountMeta::new_readonly(ctx.accounts.bubblegum_program.key(), false),
        AccountMeta::new(ctx.accounts.list_state.key(), false),
        AccountMeta::new_readonly(ctx.accounts.user.key(), true),
        AccountMeta::new_readonly(ctx.accounts.tensor_program.key(), false),
        AccountMeta::new(ctx.accounts.rent_destination.key(), true)
    ];

    // Create the instruction
    let instruction = Instruction {
        program_id: TENSOR_CNFT_PROGRAM_ID,
        accounts,
        data,
    };

    // Prepare account infos for invoke
    let account_infos = vec![
        ctx.accounts.tree_authority.to_account_info(),
        ctx.accounts.merkle_tree.to_account_info(),
        ctx.accounts.log_wrapper.to_account_info(),
        ctx.accounts.compression_program.to_account_info(),
        ctx.accounts.system_program.to_account_info(),
        ctx.accounts.bubblegum_program.to_account_info(),
        ctx.accounts.list_state.to_account_info(),
        ctx.accounts.user.to_account_info(),
        ctx.accounts.tensor_program.to_account_info(),
        ctx.accounts.rent_destination.to_account_info()
    ];

    // Perform the CPI call to Tensor
    invoke(&instruction, &account_infos)?;

    Ok(())
}
