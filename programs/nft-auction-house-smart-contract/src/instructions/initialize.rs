use anchor_lang::prelude::*;
use crate::{ constants::{ CONFIG_SEED, NFT_LIST_SEED }, state::Config };

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(init, payer = admin, space = 8 + 32 + 32 + 2, seeds = [CONFIG_SEED], bump)]
    pub global_config: Account<'info, Config>,

    pub system_program: Program<'info, System>,
}

pub fn initialize(
    ctx: Context<Initialize>,
    platform_vault: Pubkey,
    fee_percentage: u16
) -> Result<()> {
    let config = &mut ctx.accounts.global_config;

    config.admin = ctx.accounts.admin.key();
    config.platform_vault = platform_vault;
    config.fee_percentage = fee_percentage;

    Ok(())
}
