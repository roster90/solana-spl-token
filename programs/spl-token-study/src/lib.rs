use anchor_lang::prelude::borsh::BorshDeserialize;
use anchor_lang::prelude::*;

declare_id!("26nVq6oP7XqZiaavfZCMhjDm1nGUrx4PRzoaKL2Vk3LY");

#[program]
pub mod spl_token_study {

    use anchor_lang::system_program;
    use anchor_spl::{token::{ burn, close_account, freeze_account, initialize_mint, mint_to, set_authority, spl_token::instruction::AuthorityType, thaw_account, transfer, Burn, CloseAccount, FreezeAccount, InitializeMint, Mint, MintTo, SetAuthority, ThawAccount, Token, TokenAccount, Transfer}, associated_token::{self, AssociatedToken}, };

    use super::*;
    pub fn create_token(ctx: Context<CreateToken>,decimals:u8,amount:u64) -> Result<()> {
        system_program::create_account(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(), 
                system_program::CreateAccount { from: ctx.accounts.signer.to_account_info(), to: ctx.accounts.mint_token.to_account_info() }
            ), 
            10_000_000, 
            82, 
            ctx.accounts.token_program.key
        )?;

        initialize_mint(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                InitializeMint{mint:ctx.accounts.mint_token.to_account_info(),rent:ctx.accounts.rent.to_account_info()}
            ), 
            decimals, 
            ctx.accounts.signer.key, 
            Some(ctx.accounts.signer.key)
        )?;


        associated_token::create(
            CpiContext::new(
                ctx.accounts.associate_token_program.to_account_info(), 
                associated_token::Create { 
                    payer: ctx.accounts.signer.to_account_info(), 
                    associated_token: ctx.accounts.token_account.to_account_info(), 
                    authority: ctx.accounts.signer.to_account_info(), 
                    mint: ctx.accounts.mint_token.to_account_info(), 
                    system_program: ctx.accounts.system_program.to_account_info(), 
                    token_program: ctx.accounts.token_program.to_account_info() 
                }
            )
        )?;

        mint_to(
            CpiContext::new(
                ctx.accounts.token_account.to_account_info(), 
                MintTo{
                    authority:ctx.accounts.signer.to_account_info(),
                    mint:ctx.accounts.mint_token.to_account_info(),
                    to:ctx.accounts.token_account.to_account_info()}
            ), 
            amount
        )?;

        Ok(())
    }
    pub fn transer_token(ctx: Context<TransferToken>,amount:u64)->Result<()>{

        msg!("Started {:} tokens transfer from account {:} to {:}",amount,ctx.accounts.from_account.key(),ctx.accounts.to_account.key());

        transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(), 
                Transfer{
                    authority:ctx.accounts.signer.to_account_info(),
                    from:ctx.accounts.from_account.to_account_info(),
                    to:ctx.accounts.to_account.to_account_info()}
            ), 
            amount
        )?;

        Ok(())
    }

#[derive(Accounts)]
pub struct CreateToken<'info> {
    #[account(mut)]
    pub mint_token:Signer<'info>,
    #[account(mut)]
    pub signer:Signer<'info>,
    ///CHECK:
    #[account(mut)]
    pub token_account:AccountInfo<'info>,
    pub system_program:Program<'info,System>,
    pub token_program:Program<'info,Token>,
    pub associate_token_program:Program<'info,AssociatedToken>,
    pub rent:Sysvar<'info,Rent>
}
#[derive(Accounts)]
pub struct TransferToken<'info>{    
    #[account(mut)]
    pub mint_token:Account<'info,Mint>,
    #[account(mut)]
    pub from_account:Account<'info,TokenAccount>,
    #[account(mut)]
    pub to_account:Account<'info,TokenAccount>,
    #[account(mut)]
    pub signer:Signer<'info>,
    pub system_program:Program<'info,System>,
    pub token_program:Program<'info,Token>,
    pub associate_token_program:Program<'info,AssociatedToken>,
}
}