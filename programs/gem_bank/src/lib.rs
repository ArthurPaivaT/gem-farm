use anchor_lang::prelude::*;
use instructions::*;

declare_id!("HH3mNHoxYP8riakmQMcYhGaY4ToXzyrt14Kiit4JK9S");

pub mod instructions;
pub mod state;

#[program]
pub mod gem_bank {
    use super::*;

    pub fn init_bank(ctx: Context<InitBank>) -> ProgramResult {
        instructions::init_bank::handler(ctx)
    }

    pub fn set_bank_flags(ctx: Context<SetBankFlags>, flags: u32) -> ProgramResult {
        instructions::set_bank_flags::handler(ctx, flags)
    }

    pub fn init_vault(
        ctx: Context<InitVault>,
        _bump: u8,
        owner: Pubkey,
        name: String,
    ) -> ProgramResult {
        instructions::init_vault::handler(ctx, owner, name)
    }

    pub fn set_vault_lock(ctx: Context<SetVaultLock>, vault_lock: bool) -> ProgramResult {
        instructions::set_vault_lock::handler(ctx, vault_lock)
    }

    pub fn update_vault_owner(ctx: Context<UpdateVaultOwner>, new_owner: Pubkey) -> ProgramResult {
        instructions::update_vault_owner::handler(ctx, new_owner)
    }

    pub fn deposit_gem(
        ctx: Context<DepositGem>,
        _bump_auth: u8,
        _bump_gem_box: u8,
        _bump_gdr: u8,
        _bump_rarity: u8,
        amount: u64,
    ) -> ProgramResult {
        instructions::deposit_gem::handler(ctx, amount)
    }

    pub fn withdraw_gem(
        ctx: Context<WithdrawGem>,
        _bump_auth: u8,
        _bump_gem_box: u8,
        _bump_gdr: u8,
        _bump_rarity: u8,
        amount: u64,
    ) -> ProgramResult {
        instructions::withdraw_gem::handler(ctx, amount)
    }

    pub fn add_to_whitelist(
        ctx: Context<AddToWhitelist>,
        _bump: u8,
        whitelist_type: u8,
    ) -> ProgramResult {
        instructions::add_to_whitelist::handler(ctx, whitelist_type)
    }

    pub fn remove_from_whitelist(ctx: Context<RemoveFromWhitelist>, _bump: u8) -> ProgramResult {
        instructions::remove_from_whitelist::handler(ctx)
    }

    pub fn update_bank_manager(
        ctx: Context<UpdateBankManager>,
        new_manager: Pubkey,
    ) -> ProgramResult {
        instructions::update_bank_manager::handler(ctx, new_manager)
    }

    pub fn record_rarity_points<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, RecordRarityPoints<'info>>,
        rarity_configs: Vec<RarityConfig>,
    ) -> ProgramResult {
        msg!("record rarity points");
        instructions::record_rarity_points::handler(ctx, rarity_configs)
    }
}
