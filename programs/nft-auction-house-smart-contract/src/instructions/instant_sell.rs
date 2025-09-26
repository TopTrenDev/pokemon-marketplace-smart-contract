use anchor_lang::{ prelude::*, solana_program::{ instruction::Instruction, program::invoke } };
use anchor_spl::{ associated_token::AssociatedToken, token::{ self, Token, TokenAccount } };
use crate::{
    constants::{ BUBBLEGUM_PROGRAM_ID, CONFIG_SEED, USDC_MINT },
    errors::{ CustomError, NftAuctionError },
    state::{ Config, TransferArgs },
};

#[derive(Accounts)]
pub struct InstantSell<'info> {
    #[account(
        mut,
        seeds = [CONFIG_SEED],
        bump
    )]
    pub global_config: Account<'info, Config>,

    #[account(mut)]
    pub user: Signer<'info>,

    /// CHECK:
    #[account(mut)]
    pub platform_vault: Signer<'info>,

    /// CHECK: USDC mint
    #[account(constraint = usdc_mint.key() == USDC_MINT @ CustomError::InvalidUsdcMint)]
    pub usdc_mint: AccountInfo<'info>,

    #[account(
        mut,
        associated_token::mint = usdc_mint,
        associated_token::authority = platform_vault,
        associated_token::token_program = token_program
    )]
    pub vault_usdc_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = usdc_mint,
        associated_token::authority = user,
        associated_token::token_program = token_program
    )]
    pub user_usdc_account: Account<'info, TokenAccount>,

    /// CHECK:
    #[account(mut)]
    pub tree_authority: UncheckedAccount<'info>,

    /// CHECK:
    #[account(mut)]
    pub merkle_tree: UncheckedAccount<'info>,

    /// CHECK: Bubblegum program
    #[account(constraint = bubblegum_program.key() == BUBBLEGUM_PROGRAM_ID)]
    pub bubblegum_program: AccountInfo<'info>,

    /// CHECK:
    pub compression_program: UncheckedAccount<'info>,

    /// CHECK:
    pub log_wrapper: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub system_program: Program<'info, System>,

    pub rent: Sysvar<'info, Rent>,
}

pub fn instant_sell(
    ctx: Context<InstantSell>,
    pack_id: u8,
    transfer_args: TransferArgs
) -> Result<()> {
    let config = &ctx.accounts.global_config;
    let pack_price = match pack_id {
        0..=5 => crate::constants::PACK_PRICES[pack_id as usize],
        _ => {
            return Err(CustomError::InvalidPackId.into());
        }
    };
    let sell_amount =
        ((pack_price as u64) * ((10000 - config.fee_percentage) as u64) * 1_000_000) / 10000;

    require!(
        ctx.accounts.vault_usdc_account.amount >= sell_amount,
        NftAuctionError::InsufficientFunds
    );

    token::transfer(
        CpiContext::new(ctx.accounts.token_program.to_account_info(), token::Transfer {
            from: ctx.accounts.vault_usdc_account.to_account_info(),
            to: ctx.accounts.user_usdc_account.to_account_info(),
            authority: ctx.accounts.platform_vault.to_account_info(),
        }),
        sell_amount
    )?;

    let transfer_discriminator: [u8; 8] = [163, 52, 200, 231, 140, 3, 69, 186];

    let mut data = transfer_discriminator[..8].to_vec();
    data.extend(transfer_args.try_to_vec().map_err(|_| error!(NftAuctionError::SerializeError))?);

    // transfer cNFT via Bubblegum CPI
    // .......

    Ok(())
}
