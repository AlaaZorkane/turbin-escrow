use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};

use crate::{Escrow, ESCROW_SEED};

pub fn withdraw_spl_from_vault(ctx: &Context<TakeAccounts>) -> Result<()> {
    let escrow = &ctx.accounts.escrow;
    let taker_ata = ctx.accounts.taker_ata_b.to_account_info();
    let mint_a = ctx.accounts.mint_a.to_account_info();
    let token_program = ctx.accounts.token_program.to_account_info();
    let decimals = ctx.accounts.mint_a.decimals;
    let amount = ctx.accounts.escrow.receive;
    let maker_key = ctx.accounts.maker.key();
    let seed = escrow.seed.to_le_bytes();

    let seeds = [
        ESCROW_SEED,
        maker_key.as_ref(),
        seed.as_ref(),
        &[escrow.bump],
    ];
    let signature = &[&seeds[..]];

    let cpi_accounts = TransferChecked {
        authority: ctx.accounts.vault.to_account_info(),
        from: ctx.accounts.vault.to_account_info(),
        mint: mint_a,
        to: taker_ata,
    };
    let cpi_ctx = CpiContext::new_with_signer(token_program, cpi_accounts, signature);

    transfer_checked(cpi_ctx, amount, decimals)
}

/// Send the receive amount of mint<b> from the taker ata to the maker ata
fn send_spl_to_maker(ctx: &Context<TakeAccounts>) -> Result<()> {
    let token_program = ctx.accounts.token_program.to_account_info();
    let amount = ctx.accounts.escrow.receive;
    let decimals = ctx.accounts.mint_b.decimals;

    let cpi_accounts = TransferChecked {
        authority: ctx.accounts.taker_ata_b.to_account_info(),
        from: ctx.accounts.taker_ata_b.to_account_info(),
        mint: ctx.accounts.mint_b.to_account_info(),
        to: ctx.accounts.maker_ata_b.to_account_info(),
    };

    let cpi_ctx = CpiContext::new(token_program, cpi_accounts);

    transfer_checked(cpi_ctx, amount, decimals)
}

pub fn _take(ctx: Context<TakeAccounts>) -> Result<()> {
    msg!("Taker taker");

    withdraw_spl_from_vault(&ctx)?;
    send_spl_to_maker(&ctx)?;

    Ok(())
}

#[derive(Accounts)]
pub struct TakeAccounts<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,
    pub maker: SystemAccount<'info>,
    #[account(
        mut,
        seeds = [ESCROW_SEED, maker.key().as_ref(), escrow.seed.to_le_bytes().as_ref()],
        bump = escrow.bump
    )]
    pub escrow: Account<'info, Escrow>,
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
        associated_token::token_program = token_program
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = mint_b,
        associated_token::authority = taker,
        associated_token::token_program = token_program
    )]
    pub taker_ata_b: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = mint_a,
        associated_token::authority = maker,
        associated_token::token_program = token_program
    )]
    pub maker_ata_b: InterfaceAccount<'info, TokenAccount>,
    pub mint_a: InterfaceAccount<'info, Mint>,
    pub mint_b: InterfaceAccount<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}
