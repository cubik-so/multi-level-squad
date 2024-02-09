use anchor_lang::prelude::*;
use anchor_lang::solana_program::{self, system_program, sysvar::rent::Rent};
use squads_multisig_program::{Member, Permission, Permissions, SEED_PREFIX, SEED_VAULT};

declare_id!("9LS4pBp29znJwJABdzpsMwoef9rYLcJnzKsGaANh4zEQ");

#[program]
pub mod multi_level_squads {
    use super::*;
    pub fn create_admin_squad(
        ctx: Context<CreateAdminSquad>,
        members_keys: Vec<Pubkey>,
        threshold: u16,
        config_authority: Option<Pubkey>,
        time_lock: u32,
        memo: Option<String>,
    ) -> Result<()> {
        let squad_account = &mut ctx.accounts.admin_squad;

        let create_multisig = squads_multisig_program::cpi::accounts::MultisigCreate {
            create_key: ctx.accounts.create_key.to_account_info(),
            creator: ctx.accounts.authority.to_account_info(),
            multisig: ctx.accounts.multisig.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
        };

        let cpi_ctx_squads = CpiContext::new(
            ctx.accounts.squads_program.to_account_info(),
            create_multisig,
        );
        let all_permissions = [Permission::Initiate, Permission::Vote, Permission::Execute];

        let permission = Permissions::from_vec(&all_permissions);
        let (vault_pubkey, _vault_bump_seed) = Pubkey::find_program_address(
            &[
                SEED_PREFIX,
                &ctx.accounts.multisig.key().to_bytes(),
                SEED_VAULT,
                &[0],
            ],
            &squads_multisig_program::ID,
        );
        let members: Vec<Member> = members_keys
            .iter()
            .map(|key| Member {
                key: *key,
                permissions: permission,
            })
            .collect();

        squads_multisig_program::cpi::multisig_create(
            cpi_ctx_squads,
            squads_multisig_program::MultisigCreateArgs {
                config_authority,
                members,
                memo,
                threshold,
                time_lock,
            },
        )?;

        squad_account.multisig = ctx.accounts.multisig.key();
        squad_account.vault_key = vault_pubkey;
        squad_account.bump = ctx.bumps.admin_squad;

        Ok(())
    }

    pub fn create_member_squad(
        ctx: Context<CreateMemberSquad>,
        members_keys: Vec<Pubkey>,
        threshold: u16,
        config_authority: Option<Pubkey>,
        time_lock: u32,
        memo: Option<String>,
    ) -> Result<()> {
        let squad_account = &mut ctx.accounts.member_squad;

        let create_multisig = squads_multisig_program::cpi::accounts::MultisigCreate {
            create_key: ctx.accounts.create_key.to_account_info(),
            creator: ctx.accounts.authority.to_account_info(),
            multisig: ctx.accounts.multisig.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
        };

        let cpi_ctx_squads = CpiContext::new(
            ctx.accounts.squads_program.to_account_info(),
            create_multisig,
        );
        let all_permissions = [Permission::Initiate, Permission::Vote, Permission::Execute];

        let permission = Permissions::from_vec(&all_permissions);
        let (vault_pubkey, _vault_bump_seed) = Pubkey::find_program_address(
            &[
                SEED_PREFIX,
                &ctx.accounts.multisig.key().to_bytes(),
                SEED_VAULT,
                &[0],
            ],
            &squads_multisig_program::ID,
        );
        let members: Vec<Member> = members_keys
            .iter()
            .map(|key| Member {
                key: *key,
                permissions: permission,
            })
            .collect();

        squads_multisig_program::cpi::multisig_create(
            cpi_ctx_squads,
            squads_multisig_program::MultisigCreateArgs {
                config_authority,
                members,
                memo,
                threshold,
                time_lock,
            },
        )?;

        squad_account.multisig = ctx.accounts.multisig.key();
        squad_account.vault_key = vault_pubkey;
        squad_account.bump = ctx.bumps.member_squad;

        Ok(())
    }

    pub fn create_tx(ctx: Context<CreateTx>, create_key: Pubkey) -> Result<()> {
        let member_squad = &mut ctx.accounts.member_squad;
        let admin_squad = &mut ctx.accounts.admin_squad;

        let (vault_pubkey, vault_bump_seed) = Pubkey::find_program_address(
            &[
                SEED_PREFIX,
                &ctx.accounts.multisig.key().to_bytes(),
                SEED_VAULT,
                &[0],
            ],
            &squads_multisig_program::ID,
        );

        let opportunity_seeds = &[
            SEED_PREFIX,
            &ctx.accounts.multisig.key().to_bytes(),
            SEED_VAULT,
            &[0],
            &[vault_bump_seed],
        ];
        let opportunity_signer = &[&opportunity_seeds[..]];

        let vault = squads_multisig_program::VaultTransactionCreate {
            multisig: ctx.accounts.multisig.to_account_info(),
            creator: admin_squad.vault_key,                       //
            transaction: ctx.accounts.multisig.to_account_info(), //
            rent_payer: ctx.accounts.authority.to_account_info(), //
        };

        let cpi_ctx_squads = CpiContext::new(
            ctx.accounts.squads_program.to_account_info(),
            vault, // todo - fix
        )
        .with_signer(opportunity_signer);

        squads_multisig_program::cpi::vault_transaction_create(
            cpi_ctx_squads,
            squads_multisig_program::VaultTransactionCreateArgs {
                vault_index: 0,
                ephemeral_signers: 0,
                memo: None,
                transaction_message: None, //
            },
        );

        Ok(())
    }
}

#[account]
pub struct Vault {
    pub multisig: Pubkey,
    pub vault_key: Pubkey,
    pub bump: u8,
}

#[derive(Accounts)]
pub struct CreateAdminSquad<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut)]
    pub create_key: Signer<'info>,

    #[account(init,
        payer = authority,
        space = 8 + 32 + 32 + 8,
        seeds = [b"admin"],
        bump
    )]
    pub admin_squad: Box<Account<'info, Vault>>,

    /// CHECK: This is a CPI account
    #[account(mut)]
    pub multisig: UncheckedAccount<'info>,

    #[account(address = squads_multisig_program::ID)]
    pub squads_program: Program<'info, squads_multisig_program::program::SquadsMultisigProgram>,

    // Misc Accounts
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
    #[account(address = solana_program::sysvar::rent::ID)]
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct CreateMemberSquad<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut)]
    pub create_key: Signer<'info>,

    #[account(init,
        payer = authority,
        space = 8 + 32 + 32 + 8,
        seeds = [b"member", create_key.key().as_ref()],
        bump
    )]
    pub member_squad: Box<Account<'info, Vault>>,

    /// CHECK: This is a CPI account
    #[account(mut)]
    pub multisig: UncheckedAccount<'info>,

    #[account(address = squads_multisig_program::ID)]
    pub squads_program: Program<'info, squads_multisig_program::program::SquadsMultisigProgram>,

    // Misc Accounts
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
    #[account(address = solana_program::sysvar::rent::ID)]
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
#[instruction(create_key: Pubkey)]
pub struct CreateTx<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut,
        seeds = [b"member", create_key.key().as_ref()],
        bump = member_squad.bump
    )]
    pub member_squad: Box<Account<'info, Vault>>,

    #[account(mut,
        seeds = [b"admin"],
        bump = member_squad.bump
    )]
    pub admin_squad: Box<Account<'info, Vault>>,

    /// CHECK: This is a CPI account
    #[account(mut)]
    pub multisig: UncheckedAccount<'info>,

    #[account(address = squads_multisig_program::ID)]
    pub squads_program: Program<'info, squads_multisig_program::program::SquadsMultisigProgram>,

    // Misc Accounts
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
    #[account(address = solana_program::sysvar::rent::ID)]
    pub rent: Sysvar<'info, Rent>,
}
