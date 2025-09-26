use anchor_lang::prelude::*;
use anchor_spl::{ token::Token, associated_token::AssociatedToken };
use orao_solana_vrf::RANDOMNESS_ACCOUNT_SEED;
use crate::errors::NftAuctionError;
use crate::state::NftList;
use crate::misc::*;

#[derive(Accounts)]
#[instruction(force: [u8; 32])]
pub struct FulfillRandom<'info> {
    #[account(mut)]
    pub platform_vault: Signer<'info>,

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub nft_list: Account<'info, NftList>,

    /// CHECK: Randomness
    #[account(
        mut,
        seeds = [RANDOMNESS_ACCOUNT_SEED, &force],
        bump,
        seeds::program = orao_solana_vrf::ID
    )]
    pub random: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub system_program: Program<'info, System>,

    pub rent: Sysvar<'info, Rent>,
}

pub fn fulfill_random(ctx: Context<FulfillRandom>, _force: [u8; 32]) -> Result<()> {
    let rand_acc = crate::misc::get_account_data(&ctx.accounts.random)?;

    let randomness = current_state(&rand_acc);
    msg!("Orao Random number: {}", randomness);
    require!(randomness != 0, NftAuctionError::StillProcessing);

    Ok(())
}
