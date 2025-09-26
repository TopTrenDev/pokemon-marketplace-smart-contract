use anchor_lang::prelude::*;
use anchor_lang::solana_program::{ instruction::{ Instruction, AccountMeta }, program::invoke };
use crate::constants::TENSOR_CNFT_PROGRAM_ID;
use crate::errors::NftAuctionError;

#[derive(Accounts)]
pub struct ListCompressedNft<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub platform_vault: Signer<'info>,

    /// The owner/seller of the compressed NFT
    #[account(mut)]
    pub owner: Signer<'info>,

    /// CHECK: The merkle tree account that contains the compressed NFT
    #[account(mut)]
    pub merkle_tree: UncheckedAccount<'info>,

    /// CHECK: Tree authority for the merkle tree
    #[account(mut)]
    pub tree_authority: UncheckedAccount<'info>,

    /// CHECK: Listing state account (PDA owned by Tensor)
    #[account(mut)]
    pub list_state: UncheckedAccount<'info>,

    /// CHECK: Bubblegum program for compressed NFT operations
    pub bubblegum_program: UncheckedAccount<'info>,

    /// CHECK: Compression program
    pub compression_program: UncheckedAccount<'info>,

    /// CHECK: Log wrapper program
    pub log_wrapper: UncheckedAccount<'info>,

    /// CHECK: Rent payer
    #[account(mut)]
    pub rent_payer: AccountInfo<'info>,

    /// CHECK: Tensor swap program
    #[account(constraint = tensor_program.key() == TENSOR_CNFT_PROGRAM_ID)]
    pub tensor_program: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

pub fn list_compressed_nft(
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
    let discriminator = anchor_lang::solana_program::hash::hash(b"global:list").to_bytes();
    let mut data = discriminator[..8].to_vec();

    // Serialize the listing arguments
    #[derive(AnchorSerialize)]
    struct TensorListArgs {
        nonce: u64,
        index: u32,
        root: [u8; 32],
        data_hash: [u8; 32],
        creator_hash: [u8; 32],
        amount: u64,
        expire_in_sec: Option<u64>,
        currency: Option<Pubkey>,
        private_taker: Option<Pubkey>,
        maker_broker: Option<Pubkey>,
    }

    let tensor_args = TensorListArgs {
        nonce,
        index,
        root,
        data_hash,
        creator_hash,
        amount,
        expire_in_sec,
        currency,
        private_taker,
        maker_broker,
    };

    data.extend(tensor_args.try_to_vec().map_err(|_| NftAuctionError::SerializeError)?);

    // Prepare accounts for the CPI call
    let accounts = vec![
        AccountMeta::new_readonly(ctx.accounts.tree_authority.key(), false),
        AccountMeta::new_readonly(ctx.accounts.user.key(), true),
        AccountMeta::new_readonly(ctx.accounts.user.key(), true),
        AccountMeta::new(ctx.accounts.merkle_tree.key(), false)
        /*.....*/
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
        ctx.accounts.user.to_account_info(),
        ctx.accounts.user.to_account_info(),
        ctx.accounts.merkle_tree.to_account_info()
        /*.....*/
    ];

    invoke(&instruction, &account_infos)?;

    Ok(())
}
