use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface::{Mint, TokenAccount, TokenInterface, TransferChecked, transfer_checked, CloseAccount, close_account}};

use crate::Escrow;

#[derive(Accounts)]
pub struct Take<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,
    pub maker: SystemAccount<'info>,
    #[account(
        mut,
        close = maker,
        has_one = maker,
        has_one = mint_a,
        has_one = mint_b,
        seeds = [b"escrow", maker.key().as_ref(), escrow.seed.to_le_bytes().as_ref()],
        bump = escrow.bump,
    )]
    pub escrow: Account<'info, Escrow>,
    pub mint_a: InterfaceAccount<'info, Mint>,
    pub mint_b: InterfaceAccount<'info, Mint>,
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = mint_a,
        associated_token::authority = taker,
    )]
    pub taker_mint_a_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint_b,
        associated_token::authority = taker,
    )]
    pub taker_mint_b_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint_b,
        associated_token::authority = maker,
    )]
    pub maker_mint_b_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
    )]
    pub escrow_mint_a_ata: InterfaceAccount<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>
}

impl<'info> Take<'info> {
    pub fn deposit(&mut self) -> Result<()> {

        let token_program = self.token_program.to_account_info();

        let token_program_transfer_accounts = TransferChecked {
            from: self.taker_mint_b_ata.to_account_info(),
            mint: self.mint_b.to_account_info(),
            to: self.maker_mint_b_ata.to_account_info(),
            authority: self.taker.to_account_info(),
        };

        let cpi_context = CpiContext::new(
            token_program,
            token_program_transfer_accounts,
        );

        transfer_checked(
            cpi_context,
            self.escrow.mint_b_amount,
            self.mint_b.decimals,
        )?;

        Ok(())
    }

    pub fn withdraw(&mut self) -> Result<()> {

        let token_program = self.token_program.to_account_info();

        let token_program_transfer_accounts = TransferChecked {
            from: self.escrow_mint_a_ata.to_account_info(),
            mint: self.mint_a.to_account_info(),
            to: self.taker_mint_a_ata.to_account_info(),
            authority: self.escrow.to_account_info(),
        };

        let escrow_signer_seeds: [&[&[u8]]; 1] = [&[
            b"escrow",
            self.maker.to_account_info().key.as_ref(),
            &self.escrow.seed.to_be_bytes()[..],
            &[self.escrow.bump],
        ]];


        let cpi_context = CpiContext::new_with_signer(
            token_program,
            token_program_transfer_accounts,
            &escrow_signer_seeds,
        );

        transfer_checked(
            cpi_context,
            self.escrow.mint_b_amount,
            self.mint_b.decimals,
        )?;

        Ok(())
    }

    pub fn close(&mut self)  -> Result<()> {

        let token_program = self.token_program.to_account_info();

        let token_program_close_accounts = CloseAccount {
            account: self.escrow_mint_a_ata.to_account_info(),
            destination: self.maker.to_account_info(),
            authority: self.escrow.to_account_info(),
        };

        let escrow_signer_seeds: [&[&[u8]]; 1] = [&[
            b"escrow",
            self.maker.to_account_info().key.as_ref(),
            &self.escrow.seed.to_be_bytes()[..],
            &[self.escrow.bump],
        ]];


        let cpi_context = CpiContext::new_with_signer(
            token_program,
            token_program_close_accounts,
            &escrow_signer_seeds,
        );

        close_account(cpi_context)?;

        Ok(())

    }
}