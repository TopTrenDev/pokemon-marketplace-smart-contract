use crate::constants::BUBBLEGUM_PROGRAM_ID;
use crate::errors::NftAuctionError;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{ instruction::Instruction, program::invoke };
use anchor_spl::{
    token::{ Mint, Token, TokenAccount, MintTo, mint_to },
    associated_token::AssociatedToken,
};
use mpl_token_metadata::{
    instructions::{
        CreateMasterEditionV3Cpi,
        CreateMasterEditionV3CpiAccounts,
        CreateMasterEditionV3InstructionArgs,
        CreateMetadataAccountV3Cpi,
        CreateMetadataAccountV3CpiAccounts,
        CreateMetadataAccountV3InstructionArgs,
    },
    types::{ CollectionDetails, Creator as MetadataCreator, DataV2 },
};
use crate::utils::extract_asset_data_from_mint;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Creator {
    pub address: Pubkey,
    pub verified: bool,
    pub share: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, PartialEq, Eq, Debug, Clone)]
pub enum TokenProgramVersion {
    Original,
    Token2022,
}

#[derive(AnchorSerialize, AnchorDeserialize, PartialEq, Eq, Debug, Clone)]
pub struct Collection {
    pub verified: bool,
    pub key: Pubkey,
}

#[derive(AnchorSerialize, AnchorDeserialize, PartialEq, Eq, Debug, Clone)]
pub enum TokenStandard {
    NonFungible,
    FungibleAsset,
    Fungible,
    NonFungibleEdition,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Uses {
    pub use_method: UseMethod,
    pub remaining: u64,
    pub total: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum UseMethod {
    Burn,
    Multiple,
    Single,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct MetadataArgs {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub seller_fee_basis_points: u16,
    pub primary_sale_happened: bool,
    pub is_mutable: bool,
    pub edition_nonce: Option<u8>,
    pub token_standard: Option<TokenStandard>,
    pub collection: Option<Collection>,
    pub uses: Option<Uses>,
    pub token_program_version: TokenProgramVersion,
    pub creators: Vec<Creator>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct MintToCollectionV1Args {
    pub metadata_args: MetadataArgs,
}

#[derive(Accounts)]
pub struct CreateCollection<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub collection_authority: Signer<'info>,

    #[account(
        init,
        payer = payer,
        mint::decimals = 0,
        mint::authority = collection_authority,
        mint::freeze_authority = collection_authority
    )]
    pub collection_mint: Account<'info, Mint>,

    #[account(
        init,
        payer = payer,
        associated_token::mint = collection_mint,
        associated_token::authority = collection_authority
    )]
    pub collection_token_account: Account<'info, TokenAccount>,

    /// CHECK:
    #[account(mut)]
    pub collection_metadata: UncheckedAccount<'info>,

    /// CHECK:
    #[account(mut)]
    pub collection_master_edition: UncheckedAccount<'info>,

    /// CHECK:
    pub token_metadata_program: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct MintCnft<'info> {
    /// CHECK:
    #[account(mut)]
    pub tree_authority: UncheckedAccount<'info>,

    /// CHECK:
    pub leaf_owner: UncheckedAccount<'info>,

    /// CHECK:
    pub leaf_delegate: UncheckedAccount<'info>,

    /// CHECK:
    #[account(mut)]
    pub merkle_tree: UncheckedAccount<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub tree_delegate: Signer<'info>,

    #[account(mut)]
    pub collection_authority: Signer<'info>,

    /// CHECK:
    pub collection_authority_record: UncheckedAccount<'info>,

    pub collection_mint: Account<'info, Mint>,

    /// CHECK: Collection metadata
    #[account(mut)]
    pub collection_metadata: UncheckedAccount<'info>,

    /// CHECK:
    #[account(mut)]
    pub collection_master_edition: UncheckedAccount<'info>,

    /// CHECK:
    pub bubblegum_signer: UncheckedAccount<'info>,

    /// CHECK:
    pub log_wrapper: UncheckedAccount<'info>,

    /// CHECK:
    pub compression_program: UncheckedAccount<'info>,

    /// CHECK:
    pub token_metadata_program: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,

    /// CHECK:
    #[account(constraint = bubblegum_program.key() == BUBBLEGUM_PROGRAM_ID)]
    pub bubblegum_program: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [b"nft_list"],
        bump
    )]
    pub nft_list: Account<'info, crate::state::NftList>,
}

pub fn create_collection<'info>(
    ctx: Context<CreateCollection>,
    name: String,
    symbol: String,
    uri: String
) -> Result<()> {
    let creator = vec![MetadataCreator {
        address: ctx.accounts.collection_authority.key(),
        verified: true,
        share: 100,
    }];

    CreateMetadataAccountV3Cpi::new(
        &ctx.accounts.token_metadata_program,
        CreateMetadataAccountV3CpiAccounts {
            metadata: &ctx.accounts.collection_metadata,
            mint: &ctx.accounts.collection_mint.to_account_info(),
            mint_authority: &ctx.accounts.collection_authority,
            payer: &ctx.accounts.payer,
            update_authority: (&ctx.accounts.collection_authority, true),
            system_program: &ctx.accounts.system_program,
            rent: Some(&ctx.accounts.rent.to_account_info()),
        },
        CreateMetadataAccountV3InstructionArgs {
            data: DataV2 {
                name,
                symbol,
                uri,
                seller_fee_basis_points: 0,
                creators: Some(creator),
                collection: None,
                uses: None,
            },
            is_mutable: true,
            collection_details: Some(CollectionDetails::V1 { size: 0 }),
        }
    ).invoke()?;

    let mint_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), MintTo {
        mint: ctx.accounts.collection_mint.to_account_info(),
        to: ctx.accounts.collection_token_account.to_account_info(),
        authority: ctx.accounts.collection_authority.to_account_info(),
    });
    mint_to(mint_ctx, 1)?;

    CreateMasterEditionV3Cpi::new(
        &ctx.accounts.token_metadata_program,
        CreateMasterEditionV3CpiAccounts {
            edition: &ctx.accounts.collection_master_edition,
            update_authority: &ctx.accounts.collection_authority,
            mint_authority: &ctx.accounts.collection_authority,
            mint: &ctx.accounts.collection_mint.to_account_info(),
            payer: &ctx.accounts.payer,
            metadata: &ctx.accounts.collection_metadata,
            token_program: &ctx.accounts.token_program,
            system_program: &ctx.accounts.system_program,
            rent: Some(&ctx.accounts.rent.to_account_info()),
        },
        CreateMasterEditionV3InstructionArgs {
            max_supply: Some(0),
        }
    ).invoke()?;
    Ok(())
}

pub fn cpi_mint_cnft<'info>(
    ctx: Context<MintCnft>,
    name: String,
    symbol: String,
    uri: String
) -> Result<()> {
    let discriminator = anchor_lang::solana_program::hash
        ::hash(b"global:mint_to_collection_v1")
        .to_bytes();

    let mut data = discriminator[..8].to_vec();

    let metadata_args = MintToCollectionV1Args {
        metadata_args: MetadataArgs {
            name: name,
            symbol: symbol,
            uri: uri,
            seller_fee_basis_points: 500,
            primary_sale_happened: false,
            is_mutable: true,
            edition_nonce: None,
            token_standard: Some(TokenStandard::NonFungible),
            collection: Some(Collection {
                verified: false,
                key: ctx.accounts.collection_mint.key(),
            }),
            uses: None,
            token_program_version: TokenProgramVersion::Original,
            creators: vec![Creator {
                address: ctx.accounts.collection_authority.key(),
                verified: false,
                share: 100,
            }],
        },
    };

    data.extend(metadata_args.try_to_vec().map_err(|_| error!(NftAuctionError::SerializeError))?);

    let accounts = vec![
        AccountMeta::new(ctx.accounts.tree_authority.key(), false),
        AccountMeta::new_readonly(ctx.accounts.leaf_owner.key(), false),
        AccountMeta::new_readonly(ctx.accounts.leaf_delegate.key(), false),
        AccountMeta::new(ctx.accounts.merkle_tree.key(), false),
        AccountMeta::new(ctx.accounts.payer.key(), true),
        AccountMeta::new_readonly(ctx.accounts.tree_delegate.key(), true),
        AccountMeta::new_readonly(ctx.accounts.collection_authority.key(), true),
        AccountMeta::new_readonly(ctx.accounts.bubblegum_program.key(), false),
        AccountMeta::new_readonly(ctx.accounts.collection_mint.key(), false),
        AccountMeta::new(ctx.accounts.collection_metadata.key(), false),
        AccountMeta::new_readonly(ctx.accounts.collection_master_edition.key(), false),
        AccountMeta::new_readonly(ctx.accounts.bubblegum_signer.key(), false),
        AccountMeta::new_readonly(ctx.accounts.log_wrapper.key(), false),
        AccountMeta::new_readonly(ctx.accounts.compression_program.key(), false),
        AccountMeta::new_readonly(ctx.accounts.token_metadata_program.key(), false),
        AccountMeta::new_readonly(ctx.accounts.system_program.key(), false)
    ];

    let ix = Instruction {
        program_id: BUBBLEGUM_PROGRAM_ID,
        accounts,
        data,
    };

    invoke(
        &ix,
        &[
            ctx.accounts.tree_authority.to_account_info(),
            ctx.accounts.leaf_owner.to_account_info(),
            ctx.accounts.leaf_delegate.to_account_info(),
            ctx.accounts.merkle_tree.to_account_info(),
            ctx.accounts.payer.to_account_info(),
            ctx.accounts.tree_delegate.to_account_info(),
            ctx.accounts.collection_authority.to_account_info(),
            ctx.accounts.bubblegum_program.to_account_info(),
            ctx.accounts.collection_mint.to_account_info(),
            ctx.accounts.collection_metadata.to_account_info(),
            ctx.accounts.collection_master_edition.to_account_info(),
            ctx.accounts.bubblegum_signer.to_account_info(),
            ctx.accounts.log_wrapper.to_account_info(),
            ctx.accounts.compression_program.to_account_info(),
            ctx.accounts.token_metadata_program.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ]
    )?;

    let asset_data = extract_asset_data_from_mint(
        &ctx.accounts.merkle_tree.key(),
        &ctx.accounts.leaf_owner.key(),
        &metadata_args.metadata_args
    )?;

    Ok(())
}
