use anchor_lang::prelude::*;
use anchor_spl::token::{self, TokenAccount, Transfer, Token};

declare_id!("G5GVsUgjwXR9umkLDJcuweR2hUrnMnAFNWxv3DoRJ69b");

#[program]
pub mod solana_swap_stake {
    use super::*;

    pub fn swap(ctx: Context<Swap>, amount: u64) -> Result<()> {
        let cpi_accounts = Transfer {
            from: ctx.accounts.from.to_account_info(),
            to: ctx.accounts.to.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, amount)?;
        Ok(())
    }

    pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()> {
        let cpi_accounts = Transfer {
            from: ctx.accounts.staker.to_account_info(),
            to: ctx.accounts.stake_pool.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, amount)?;

        // Update staked amount
        let state = &mut ctx.accounts.state;
        state.amount += amount;
        Ok(())
    }

    pub fn unstake(ctx: Context<Unstake>, amount: u64) -> Result<()> {
        let cpi_accounts = Transfer {
            from: ctx.accounts.stake_pool.to_account_info(),
            to: ctx.accounts.staker.to_account_info(),
            authority: ctx.accounts.pool_authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, amount)?;

        // Update staked amount
        let state = &mut ctx.accounts.state;
        state.amount = state.amount.checked_sub(amount).ok_or(ErrorCode::InsufficientFunds)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Swap<'info> {
    #[account(mut)]
    pub from: Account<'info, TokenAccount>,
    #[account(mut)]
    pub to: Account<'info, TokenAccount>,
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut)]
    pub staker: Account<'info, TokenAccount>,
    #[account(mut)]
    pub stake_pool: Account<'info, TokenAccount>,
    #[account(mut)]
    pub state: Account<'info, StakeState>,
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct Unstake<'info> {
    #[account(mut)]
    pub staker: Account<'info, TokenAccount>,
    #[account(mut)]
    pub stake_pool: Account<'info, TokenAccount>,
    #[account(mut)]
    pub state: Account<'info, StakeState>,
    pub pool_authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct StakeState {
    pub amount: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Insufficient funds")]
    InsufficientFunds,
}
