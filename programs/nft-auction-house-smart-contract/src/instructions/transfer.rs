use anchor_lang::prelude::*;
use anchor_lang::solana_program::{ instruction::{ Instruction, AccountMeta }, program::invoke };
use anchor_spl::{ token::Token, associated_token::AssociatedToken };
use crate::constants::BUBBLEGUM_PROGRAM_ID;
use crate::errors::NftAuctionError;
use crate::state::{ NftList, TransferArgs };

#[derive(Accounts)]
pub struct Transfer<'info> {
    #[account(mut)]
    pub platform_vault: Signer<'info>,

    /// CHECK:
    #[account(mut)]
    pub user: AccountInfo<'info>,

    #[account(mut)]
    pub nft_list: Account<'info, NftList>,

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

    pub token_program: Program<'info, Token>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub system_program: Program<'info, System>,

    pub rent: Sysvar<'info, Rent>,
}

pub fn transfer(ctx: Context<Transfer>, transfer_args: TransferArgs) -> Result<()> {
    let transfer_discriminator: [u8; 8] = [163, 52, 200, 231, 140, 3, 69, 186];

    let mut data = transfer_discriminator[..8].to_vec();
    data.extend(transfer_args.try_to_vec().map_err(|_| error!(NftAuctionError::SerializeError))?);

    let accounts = vec![
        AccountMeta::new_readonly(ctx.accounts.tree_authority.key(), false),
        AccountMeta::new(ctx.accounts.platform_vault.key(), true),
        AccountMeta::new(ctx.accounts.platform_vault.key(), true),
        AccountMeta::new_readonly(ctx.accounts.user.key(), false),
        AccountMeta::new(ctx.accounts.merkle_tree.key(), false),
        AccountMeta::new_readonly(ctx.accounts.compression_program.key(), false),
        AccountMeta::new_readonly(ctx.accounts.system_program.key(), false)
    ];

    let ix = Instruction {
        program_id: ctx.accounts.bubblegum_program.key(),
        accounts,
        data,
    };

    invoke(
        &ix,
        &[
            ctx.accounts.tree_authority.to_account_info(),
            ctx.accounts.platform_vault.to_account_info(),
            ctx.accounts.platform_vault.to_account_info(),
            ctx.accounts.user.to_account_info(),
            ctx.accounts.merkle_tree.to_account_info(),
            ctx.accounts.compression_program.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ]
    )?;

    Ok(())
}
