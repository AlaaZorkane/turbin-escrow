use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{
        close_account, transfer_checked, CloseAccount, Mint, TokenAccount, TokenInterface,
        TransferChecked,
    },
};

use crate::{Escrow, ESCROW_SEED};

pub fn drain_vault(ctx: &Context<RefundAccounts>) -> Result<()> {
    let authority = ctx.accounts.maker.to_account_info();
    let mint = ctx.accounts.mint_a.to_account_info();
    let from = ctx.accounts.vault.to_account_info();
    let to = ctx.accounts.maker_ata_a.to_account_info();
    let decimals = ctx.accounts.mint_a.decimals;
    let amount = ctx.accounts.escrow.receive;

    let cpi_accounts = TransferChecked {
        authority,
        from,
        mint,
        to,
    };

    let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);

    transfer_checked(cpi_ctx, amount, decimals)
}

fn close_vault(ctx: &Context<RefundAccounts>) -> Result<()> {
    let cpi_accounts = CloseAccount {
        account: ctx.accounts.vault.to_account_info(),
        authority: ctx.accounts.escrow.to_account_info(),
        destination: ctx.accounts.maker.to_account_info(),
    };
    let maker_key = ctx.accounts.maker.key();
    let seed = ctx.accounts.escrow.seed.to_le_bytes();

    let seeds = [
        ESCROW_SEED,
        maker_key.as_ref(),
        seed.as_ref(),
        &[ctx.accounts.escrow.bump],
    ];
    let signature = &[&seeds[..]];
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        cpi_accounts,
        signature,
    );

    close_account(cpi_ctx)
}

pub fn _refund(ctx: Context<RefundAccounts>) -> Result<()> {
    drain_vault(&ctx)?;
    close_vault(&ctx)?;

    Ok(())
}

#[derive(Accounts)]
pub struct RefundAccounts<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    #[account(
        mut,
        seeds = [ESCROW_SEED, maker.key().as_ref(), escrow.seed.to_le_bytes().as_ref()],
        bump
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
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = maker,
        associated_token::token_program = token_program
    )]
    pub maker_ata_a: InterfaceAccount<'info, TokenAccount>,
    pub mint_a: InterfaceAccount<'info, Mint>,
    pub mint_b: InterfaceAccount<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}
