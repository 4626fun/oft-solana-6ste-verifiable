use crate::*;
use anchor_lang::solana_program;
use anchor_spl::token_2022::spl_token_2022::{
    instruction::{set_authority, AuthorityType},
    solana_program::program_option::COption,
};
use anchor_spl::token_interface::{Mint, TokenInterface};

/// Admin-only: CPI Token-2022 `SetAuthority(MintTokens)` with the OFT Store as the
/// current mint authority (PDA signer). Used for one-shot Metaplex metadata create
/// when mint authority was transferred to the store before metadata existed.
#[derive(Accounts)]
pub struct AdminSetTokenMintAuthority<'info> {
    pub admin: Signer<'info>,
    #[account(
        seeds = [OFT_SEED, oft_store.token_escrow.as_ref()],
        bump = oft_store.bump,
        has_one = admin @OFTError::Unauthorized,
        has_one = token_mint @OFTError::InvalidMintAuthority,
    )]
    pub oft_store: Account<'info, OFTStore>,
    #[account(
        mut,
        mint::token_program = token_program,
        constraint = token_mint.mint_authority == COption::Some(oft_store.key()) @OFTError::InvalidMintAuthority
    )]
    pub token_mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
}

impl AdminSetTokenMintAuthority<'_> {
    pub fn apply(
        ctx: &mut Context<AdminSetTokenMintAuthority>,
        params: &AdminSetTokenMintAuthorityParams,
    ) -> Result<()> {
        require!(
            ctx.accounts.oft_store.oft_type == OFTType::Native,
            OFTError::InvalidMintAuthority
        );

        let oft_store_seed = ctx.accounts.oft_store.token_escrow.key();
        let seeds: &[&[u8]] =
            &[OFT_SEED, oft_store_seed.as_ref(), &[ctx.accounts.oft_store.bump]];

        let ix = set_authority(
            ctx.accounts.token_program.key,
            &ctx.accounts.token_mint.key(),
            Some(&params.new_authority),
            AuthorityType::MintTokens,
            &ctx.accounts.oft_store.key(),
            &[],
        )?;

        solana_program::program::invoke_signed(
            &ix,
            &[
                ctx.accounts.token_mint.to_account_info(),
                ctx.accounts.oft_store.to_account_info(),
            ],
            &[seeds],
        )?;

        Ok(())
    }
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct AdminSetTokenMintAuthorityParams {
    pub new_authority: Pubkey,
}
