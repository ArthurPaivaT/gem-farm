use anchor_lang::prelude::*;
use gem_bank::instructions::record_rarity_points::RarityConfig;
use instructions::*;
use state::*;

pub mod instructions;
pub mod number128;
pub mod state;

declare_id!("2yYDX8THp9hLM6YfTrhRUUSWtWJM9b87uEwCuFAXv26y");

#[program]
pub mod gem_farm {
    use super::*;

    // --------------------------------------- core

    pub fn init_farm(
        ctx: Context<InitFarm>,
        bump_auth: u8,
        _bump_treasury: u8,
        _bump_pot_a: u8,
        _bump_pot_b: u8,
        reward_type_a: RewardType,
        reward_type_b: RewardType,
        farm_config: FarmConfig,
    ) -> ProgramResult {
        msg!("init farm");
        instructions::init_farm::handler(ctx, bump_auth, reward_type_a, reward_type_b, farm_config)
    }

    pub fn update_farm(
        ctx: Context<UpdateFarm>,
        config: Option<FarmConfig>,
        manager: Option<Pubkey>,
    ) -> ProgramResult {
        instructions::update_farm::handler(ctx, config, manager)
    }

    pub fn payout_from_treasury(
        ctx: Context<TreasuryPayout>,
        _bump_auth: u8,
        bump_treasury: u8,
        lamports: u64,
    ) -> ProgramResult {
        msg!("payout");
        instructions::treasury_payout::handler(ctx, bump_treasury, lamports)
    }

    pub fn add_to_bank_whitelist(
        ctx: Context<AddToBankWhitelist>,
        _bump_auth: u8,
        bump_wl: u8,
        whitelist_type: u8,
    ) -> ProgramResult {
        msg!("add to bank whitelist");
        instructions::add_to_bank_whitelist::handler(ctx, bump_wl, whitelist_type)
    }

    pub fn remove_from_bank_whitelist(
        ctx: Context<RemoveFromBankWhitelist>,
        _bump_auth: u8,
        bump_wl: u8,
    ) -> ProgramResult {
        msg!("remove from bank whitelist");
        instructions::remove_from_bank_whitelist::handler(ctx, bump_wl)
    }

    // --------------------------------------- farmer ops

    pub fn init_farmer(
        ctx: Context<InitFarmer>,
        _bump_farmer: u8,
        bump_vault: u8,
    ) -> ProgramResult {
        msg!("init farmer");
        instructions::init_farmer::handler(ctx, bump_vault)
    }

    pub fn stake(ctx: Context<Stake>, _bump_auth: u8, _bump_farmer: u8) -> ProgramResult {
        msg!("stake");
        instructions::stake::handler(ctx)
    }

    pub fn unstake(
        ctx: Context<Unstake>,
        _bump_auth: u8,
        _bump_treasury: u8,
        _bump_farmer: u8,
    ) -> ProgramResult {
        msg!("unstake");
        instructions::unstake::handler(ctx)
    }

    pub fn claim(
        ctx: Context<Claim>,
        _bump_auth: u8,
        _bump_farmer: u8,
        _bump_pot_a: u8,
        _bump_pot_b: u8,
    ) -> ProgramResult {
        msg!("claim");
        instructions::claim::handler(ctx)
    }

    pub fn flash_deposit<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, FlashDeposit<'info>>,
        _bump_farmer: u8,
        bump_vault_auth: u8,
        bump_gem_box: u8,
        bump_gdr: u8,
        bump_rarity: u8,
        amount: u64,
    ) -> ProgramResult {
        // msg!("flash deposit"); //have to remove all msgs! or run out of compute budget for this ix
        instructions::flash_deposit::handler(
            ctx,
            bump_vault_auth,
            bump_gem_box,
            bump_gdr,
            bump_rarity,
            amount,
        )
    }

    pub fn refresh_farmer(ctx: Context<RefreshFarmer>, _bump: u8) -> ProgramResult {
        msg!("refresh farmer");
        instructions::refresh_farmer::handler(ctx)
    }

    /// this one needs to be called by the farmer themselves
    /// it's useful if for some reason they can't re-enroll in another fixed reward cycle (eg reward exhausted)
    /// but they want to be able to refresh themselves and claim their earned rewards up to this point
    pub fn refresh_farmer_signed(
        ctx: Context<RefreshFarmerSigned>,
        _bump: u8,
        reenroll: bool,
    ) -> ProgramResult {
        msg!("refresh farmer");
        instructions::refresh_farmer_signed::handler(ctx, reenroll)
    }

    // --------------------------------------- funder ops

    pub fn authorize_funder(ctx: Context<AuthorizeFunder>, _bump: u8) -> ProgramResult {
        msg!("authorize funder");
        instructions::authorize_funder::handler(ctx)
    }

    pub fn deauthorize_funder(ctx: Context<DeauthorizeFunder>, _bump: u8) -> ProgramResult {
        msg!("feauthorize funder");
        instructions::deauthorize_funder::handler(ctx)
    }

    // --------------------------------------- reward ops

    pub fn fund_reward(
        ctx: Context<FundReward>,
        _bump_proof: u8,
        _bump_pot: u8,
        variable_rate_config: Option<VariableRateConfig>,
        fixed_rate_config: Option<FixedRateConfig>,
    ) -> ProgramResult {
        msg!("fund reward");
        instructions::fund_reward::handler(ctx, variable_rate_config, fixed_rate_config)
    }

    pub fn cancel_reward(
        ctx: Context<CancelReward>,
        _bump_auth: u8,
        _bump_pot: u8,
    ) -> ProgramResult {
        msg!("cancel reward");
        instructions::cancel_reward::handler(ctx)
    }

    pub fn lock_reward(ctx: Context<LockReward>) -> ProgramResult {
        msg!("lock reward");
        instructions::lock_reward::handler(ctx)
    }

    // --------------------------------------- rarities

    pub fn add_rarities_to_bank<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, AddRaritiesToBank<'info>>,
        _bump_auth: u8,
        rarity_configs: Vec<RarityConfig>,
    ) -> ProgramResult {
        msg!("add rarities to bank");
        instructions::add_rarities_to_bank::handler(ctx, rarity_configs)
    }
}
