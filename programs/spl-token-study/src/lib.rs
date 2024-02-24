use anchor_lang::prelude::borsh::BorshDeserialize;
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};


declare_id!("6DtxPizvZdMSVdzk7NuuxknPCNFDVdxDX137wVQhyuBi");

#[program]
pub mod spl_token_study {

    use anchor_lang::solana_program::program::invoke_signed;
    use anchor_spl::{token::{ self, mint_to,  transfer, Burn, Mint, MintTo, Token, TokenAccount, Transfer}, associated_token::AssociatedToken, };
    use mpl_token_metadata::{instructions::{CreateMetadataAccountV3, CreateMetadataAccountV3InstructionArgs}, types::DataV2};
    use super::*;
    pub fn init_token(ctx: Context<InitToken>, metadata:InitTokenParams ) -> Result<()> {
        
        let authority = &ctx.accounts.payer;
        let seeds = &["mint".as_bytes(), &authority.key.as_ref(), &[ctx.bumps.mint]];
        let signer = [&seeds[..]];

        let account_info = vec![
            ctx.accounts.metadata.to_account_info(),
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.payer.to_account_info(),
            ctx.accounts.token_metadata_program.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ];

        let create_token_intrusion_meta_data  = &CreateMetadataAccountV3 {
            metadata:  ctx.accounts.metadata.key(),
            mint: ctx.accounts.mint.key(),
            mint_authority: ctx.accounts.mint.key(),
            update_authority: (ctx.accounts.mint.key(), false),
            payer: ctx.accounts.payer.key(),
            system_program: ctx.accounts.system_program.key(),
            rent: Some(ctx.accounts.rent.key()),    
        }.instruction(CreateMetadataAccountV3InstructionArgs{
            data: DataV2 {
                name: metadata.name,
                symbol:  metadata.symbol,
                uri:  metadata.uri,
                seller_fee_basis_points: 0,
                creators: None,
                collection: None,
                uses: None,
            },
            is_mutable: false,
            collection_details: None,
        });
        invoke_signed(
            create_token_intrusion_meta_data,
            account_info.as_slice(),
            &signer,
        )?;
        Ok(())
    }

    pub fn mint_tokens(ctx: Context<MintTokens>, quantity: u64) -> Result<()> {
        let user = &ctx.accounts.payer.key();
        let seeds = &["mint".as_bytes(), user.as_ref(), &[ctx.bumps.mint]];
        let signer = [&seeds[..]];

        mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                MintTo {
                    authority: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.destination.to_account_info(),
                    mint: ctx.accounts.mint.to_account_info(),
                },
                &signer,
            ),
            quantity,
        )?;

        Ok(())
    }

    pub fn burn_spl_token(ctx: Context<BurnToken>, amount:u64)-> Result<()>{
        msg!("Started burning tokens");
        // let seeds = &["mint".as_bytes(), &[0]];
        // let signer = [&seeds[..]];

        let cpi_accounts = Burn {
            mint: ctx.accounts.mint.to_account_info(),
            from: ctx.accounts.from.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();
        // Create the CpiContext we need for the request
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        // Execute anchor's helper function to burn tokens
        token::burn(cpi_ctx, amount)?;

   
        
        Ok(())
    }

    pub fn transfer_spl_token(ctx: Context<TransferToken>, amount:u64)->Result<()>{

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

// 4. Define the context for each instruction
#[derive(Accounts)]
#[instruction(params: InitTokenParams)]
pub struct InitToken<'info> {
  /// CHECK: New Metaplex Account being created
  #[account(mut)]
  pub metadata: UncheckedAccount<'info>,
  #[account(
      init,
      seeds = [b"mint", payer.key().as_ref()],
      bump,
      payer = payer,
      mint::decimals = params.decimals,
      mint::authority = mint,
  )]
  pub mint: Account<'info, Mint>,
  #[account(mut)]
  pub payer: Signer<'info>,
  pub rent: Sysvar<'info, Rent>,
  pub system_program: Program<'info, System>,
  pub token_program: Program<'info, Token>,
  /// CHECK: account constraint checked in account trait
  #[account(address = mpl_token_metadata::ID)]
  pub token_metadata_program: UncheckedAccount<'info>,
}
#[derive(Accounts)]
pub struct MintTokens<'info> {
    #[account(
        mut,
        seeds = [b"mint", payer.key().as_ref()],
        bump,
        mint::authority = mint,
    )]
    pub mint: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = payer,
    )]
    pub destination: Account<'info, TokenAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

#[derive(Accounts)]
pub struct  BurnToken<'info> {
    #[account(
        mut,
        seeds = [b"mint", authority.key().as_ref()],
        bump,
        mint::authority = mint,
    )]
    pub mint: Account<'info, Mint>,
    pub authority: Signer<'info>,
    pub from: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,

}


#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct InitTokenParams {
    pub decimals: u8,
    pub name: String,
    pub symbol: String,
    pub uri: String,

}