use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};

use crate::{Escrow, ESCROW_SEED};

pub fn deposit_spl_into_vault(ctx: &Context<MakeAccounts>, amount: &u64) -> Result<()> {
    let authority = ctx.accounts.maker.to_account_info();
    let mint = ctx.accounts.mint_a.to_account_info();
    let from = ctx.accounts.maker_ata_a.to_account_info();
    let to = ctx.accounts.vault.to_account_info();
    let decimals = ctx.accounts.mint_a.decimals;

    let cpi_accounts = TransferChecked {
        authority,
        from,
        mint,
        to,
    };

    let cpi_ctx = CpiContext::new(
        ctx.accounts.deposit_token_program.to_account_info(),
        cpi_accounts,
    );

    transfer_checked(cpi_ctx, *amount, decimals)
}

pub fn _make(ctx: Context<MakeAccounts>, input: MakeInput) -> Result<()> {
    let escrow = &mut ctx.accounts.escrow;
    escrow.set_inner(Escrow {
        bump: ctx.bumps.escrow,
        mint_a: ctx.accounts.mint_a.key(),
        mint_b: ctx.accounts.mint_b.key(),
        receive: input.receive,
        seed: input.seed,
    });

    deposit_spl_into_vault(&ctx, &input.receive)
}

#[derive(Accounts)]
#[instruction(input: MakeInput)]
pub struct MakeAccounts<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    #[account(
        init,
        payer = maker,
        space = 8 + Escrow::INIT_SPACE,
        seeds = [ESCROW_SEED, maker.key().as_ref(), input.seed.to_le_bytes().as_ref()],
        bump
    )]
    pub escrow: Account<'info, Escrow>,
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
        associated_token::token_program = deposit_token_program
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = maker,
        associated_token::token_program = deposit_token_program
    )]
    pub maker_ata_a: InterfaceAccount<'info, TokenAccount>,
    pub mint_a: InterfaceAccount<'info, Mint>,
    pub mint_b: InterfaceAccount<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub deposit_token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct MakeInput {
    pub seed: u64,
    pub receive: u64,
}
