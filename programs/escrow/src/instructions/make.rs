use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface::{Mint, TokenAccount, TokenInterface, TransferChecked, transfer_checked}};

use crate::Escrow;

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Make<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    #[account(
        init,
        payer = maker,
        space = 8 + Escrow::INIT_SPACE,
        seeds = [b"escrow", maker.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump,
    )]
    pub escrow: Account<'info, Escrow>,
    pub mint_a: InterfaceAccount<'info, Mint>,
    pub mint_b: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = maker,
    )]
    pub maker_mint_a_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = maker,
        associated_token::mint = mint_b,
        associated_token::authority = maker,
    )]
    pub maker_mint_b_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init,
        payer = maker,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
    )]
    pub escrow_mint_a_ata: InterfaceAccount<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>
}

impl<'info> Make<'info> {
    pub fn init_escrow(&mut self, seed: u64, mint_a_amount: u64, mint_b_amount: u64, bumps: &MakeBumps) -> Result<()> {
        self.escrow.set_inner(Escrow{
            seed,
            maker: self.maker.key(),
            mint_a: self.mint_a.key(),
            mint_a_amount,
            mint_b: self.mint_b.key(),
            mint_b_amount,
            bump: bumps.escrow,
        });
        Ok(())
    }
    
    pub fn deposit(&mut self) -> Result<()> {
        
        let token_program = self.token_program.to_account_info();
        
        let token_program_transfer_accounts = TransferChecked {
            from: self.maker_mint_a_ata.to_account_info(),
            to: self.escrow_mint_a_ata.to_account_info(),
            mint: self.mint_a.to_account_info(),
            authority: self.maker.to_account_info(),
        };

        let cpi_context = CpiContext::new(
            token_program, token_program_transfer_accounts
        );

        transfer_checked(
        cpi_context,
            self.escrow.mint_a_amount,
            self.mint_a.decimals
        )?;

        Ok(())
    }
}