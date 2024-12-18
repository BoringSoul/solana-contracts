use anchor_lang::prelude::*;
use anchor_spl::{
    token_2022,
    token_interface::Token2022,
};


// 新增 BurnNft 指令
pub fn burn_nft(ctx: Context<BurnNft>) -> Result<()> {
    let seeds = b"nft_authority";
    let bump = ctx.bumps.nft_authority;
    let signer: &[&[&[u8]]] = &[&[seeds, &[bump]]];

    token_2022::burn(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token_2022::Burn {
                mint: ctx.accounts.mint.to_account_info(),
                from: ctx.accounts.token_account.to_account_info(),
                authority: ctx.accounts.nft_authority.to_account_info(),
            },
            signer
        ),
        1
    )?;

    Ok(())
}

#[derive(Accounts)]
pub struct BurnNft<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut)]
    pub token_account: AccountInfo<'info>,
    #[account(mut)]
    pub mint: AccountInfo<'info>,
    #[account(init_if_needed, seeds = [b"nft_authority".as_ref()], bump, space = 8, payer = signer)]
    pub nft_authority: Account<'info, crate::mint_nft::NftAuthority>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token2022>,
}