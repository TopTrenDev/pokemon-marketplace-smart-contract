use anchor_lang::prelude::*;
use anchor_spl::{ associated_token::AssociatedToken, token::{ self, Token, TokenAccount } };
use orao_solana_vrf::program::OraoVrf;
use orao_solana_vrf::state::NetworkState;
use orao_solana_vrf::CONFIG_ACCOUNT_SEED;
use orao_solana_vrf::RANDOMNESS_ACCOUNT_SEED;
use crate::{ constants::USDC_MINT, errors::CustomError };   

#[derive(Accounts)]
#[instruction(force: [u8; 32])]
pub struct BuyPack<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    /// CHECK: USDC mint
    #[account(constraint = usdc_mint.key() == USDC_MINT @ CustomError::InvalidUsdcMint)]
    pub usdc_mint: AccountInfo<'info>,

    /// CHECK: platfrom vault wallet
    #[account(mut)]
    pub platform_vault: Signer<'info>,

    #[account(
        init_if_needed,
        payer = platform_vault,
        associated_token::mint = usdc_mint,
        associated_token::authority = platform_vault,
        associated_token::token_program = token_program
    )]
    pub vault_usdc_account: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = usdc_mint,
        associated_token::authority = user,
        associated_token::token_program = token_program
    )]
    pub user_usdc_account: Account<'info, TokenAccount>,

    /// CHECK: Treasury
    #[account(mut)]
    pub treasury: AccountInfo<'info>,

    /// CHECK: Randomness
    #[account(
        mut,
        seeds = [RANDOMNESS_ACCOUNT_SEED, &force],
        bump,
        seeds::program = orao_solana_vrf::ID
    )]
    pub random: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [CONFIG_ACCOUNT_SEED],
        bump,
        seeds::program = orao_solana_vrf::ID
    )]
    pub config: Account<'info, NetworkState>,

    pub vrf: Program<'info, OraoVrf>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn buy_pack(ctx: Context<BuyPack>, force: [u8; 32], pack_id: u8, pack_count: u8) -> Result<()> {
    let pack_price = match pack_id {
        0..=5 => crate::constants::PACK_PRICES[pack_id as usize],
        _ => {
            return Err(CustomError::InvalidPackId.into());
        }
    };
    let buy_amount = (pack_price as u64) * (pack_count as u64) * 1_000_000; // USDC has 6 decimals

    require!(ctx.accounts.user_usdc_account.amount >= buy_amount, CustomError::InsufficientFunds);

    token::transfer(
        CpiContext::new(ctx.accounts.token_program.to_account_info(), token::Transfer {
            from: ctx.accounts.user_usdc_account.to_account_info(),
            to: ctx.accounts.vault_usdc_account.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        }),
        buy_amount
    )?;

    let cpi_program = ctx.accounts.vrf.to_account_info();
    let cpi_accounts = orao_solana_vrf::cpi::accounts::RequestV2 {
        payer: ctx.accounts.user.to_account_info(),
        network_state: ctx.accounts.config.to_account_info(),
        treasury: ctx.accounts.treasury.to_account_info(),
        request: ctx.accounts.random.to_account_info(),
        system_program: ctx.accounts.system_program.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    orao_solana_vrf::cpi::request_v2(cpi_ctx, force)?;

    Ok(())
}
