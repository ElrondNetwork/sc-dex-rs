// Code generated by the multiversx-sc proxy generator. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![allow(dead_code)]
#![allow(clippy::all)]

use multiversx_sc::proxy_imports::*;
use multiversx_sc_scenario::multiversx_sc;

pub struct FarmProxy;

impl<Env, From, To, Gas> TxProxyTrait<Env, From, To, Gas> for FarmProxy
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    type TxProxyMethods = FarmProxyMethods<Env, From, To, Gas>;

    fn proxy_methods(self, tx: Tx<Env, From, To, (), Gas, (), ()>) -> Self::TxProxyMethods {
        FarmProxyMethods { wrapped_tx: tx }
    }
}

pub struct FarmProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    wrapped_tx: Tx<Env, From, To, (), Gas, (), ()>,
}

#[rustfmt::skip]
impl<Env, From, Gas> FarmProxyMethods<Env, From, (), Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    Gas: TxGas<Env>,
{
    pub fn init<
        Arg0: CodecInto<TokenIdentifier<Env::Api>>,
        Arg1: CodecInto<TokenIdentifier<Env::Api>>,
        Arg2: CodecInto<BigUint<Env::Api>>,
        Arg3: CodecInto<ManagedAddress<Env::Api>>,
        Arg4: CodecInto<ManagedAddress<Env::Api>>,
        Arg5: CodecInto<MultiValueEncoded<Env::Api, ManagedAddress<Env::Api>>>,
    >(
        self,
        reward_token_id: Arg0,
        farming_token_id: Arg1,
        division_safety_constant: Arg2,
        pair_contract_address: Arg3,
        owner: Arg4,
        admins: Arg5,
    ) -> TxProxyDeploy<Env, From, Gas, ()> {
        self.wrapped_tx
            .raw_deploy()
            .argument(&reward_token_id)
            .argument(&farming_token_id)
            .argument(&division_safety_constant)
            .argument(&pair_contract_address)
            .argument(&owner)
            .argument(&admins)
            .original_result()
    }
}

#[rustfmt::skip]
impl<Env, From, To, Gas> FarmProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    pub fn upgrade(
        self,
    ) -> TxProxyUpgrade<Env, From, To, Gas, ()> {
        self.wrapped_tx
            .raw_upgrade()
            .original_result()
    }
}

#[rustfmt::skip]
impl<Env, From, To, Gas> FarmProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    pub fn enter_farm_endpoint<
        Arg0: CodecInto<OptionalValue<ManagedAddress<Env::Api>>>,
    >(
        self,
        opt_orig_caller: Arg0,
    ) -> TxProxyCall<Env, From, To, Gas, MultiValue2<EsdtTokenPayment<Env::Api>, EsdtTokenPayment<Env::Api>>> {
        self.wrapped_tx
            .raw_call("enterFarm")
            .argument(&opt_orig_caller)
            .original_result()
    }

    pub fn claim_rewards_endpoint<
        Arg0: CodecInto<OptionalValue<ManagedAddress<Env::Api>>>,
    >(
        self,
        opt_orig_caller: Arg0,
    ) -> TxProxyCall<Env, From, To, Gas, MultiValue2<EsdtTokenPayment<Env::Api>, EsdtTokenPayment<Env::Api>>> {
        self.wrapped_tx
            .raw_call("claimRewards")
            .argument(&opt_orig_caller)
            .original_result()
    }

    pub fn exit_farm_endpoint<
        Arg0: CodecInto<OptionalValue<ManagedAddress<Env::Api>>>,
    >(
        self,
        opt_orig_caller: Arg0,
    ) -> TxProxyCall<Env, From, To, Gas, MultiValue2<EsdtTokenPayment<Env::Api>, EsdtTokenPayment<Env::Api>>> {
        self.wrapped_tx
            .raw_call("exitFarm")
            .argument(&opt_orig_caller)
            .original_result()
    }

    pub fn merge_farm_tokens_endpoint<
        Arg0: CodecInto<OptionalValue<ManagedAddress<Env::Api>>>,
    >(
        self,
        opt_orig_caller: Arg0,
    ) -> TxProxyCall<Env, From, To, Gas, MultiValue2<EsdtTokenPayment<Env::Api>, EsdtTokenPayment<Env::Api>>> {
        self.wrapped_tx
            .raw_call("mergeFarmTokens")
            .argument(&opt_orig_caller)
            .original_result()
    }

    pub fn claim_boosted_rewards<
        Arg0: CodecInto<OptionalValue<ManagedAddress<Env::Api>>>,
    >(
        self,
        opt_user: Arg0,
    ) -> TxProxyCall<Env, From, To, Gas, EsdtTokenPayment<Env::Api>> {
        self.wrapped_tx
            .raw_call("claimBoostedRewards")
            .argument(&opt_user)
            .original_result()
    }

    pub fn start_produce_rewards_endpoint(
        self,
    ) -> TxProxyCall<Env, From, To, Gas, ()> {
        self.wrapped_tx
            .raw_call("startProduceRewards")
            .original_result()
    }

    pub fn end_produce_rewards_endpoint(
        self,
    ) -> TxProxyCall<Env, From, To, Gas, ()> {
        self.wrapped_tx
            .raw_call("endProduceRewards")
            .original_result()
    }

    pub fn set_per_block_rewards_endpoint<
        Arg0: CodecInto<BigUint<Env::Api>>,
    >(
        self,
        per_block_amount: Arg0,
    ) -> TxProxyCall<Env, From, To, Gas, ()> {
        self.wrapped_tx
            .raw_call("setPerBlockRewardAmount")
            .argument(&per_block_amount)
            .original_result()
    }

    pub fn set_boosted_yields_rewards_percentage<
        Arg0: CodecInto<u64>,
    >(
        self,
        percentage: Arg0,
    ) -> TxProxyCall<Env, From, To, Gas, ()> {
        self.wrapped_tx
            .raw_call("setBoostedYieldsRewardsPercentage")
            .argument(&percentage)
            .original_result()
    }

    pub fn calculate_rewards_for_given_position<
        Arg0: CodecInto<ManagedAddress<Env::Api>>,
        Arg1: CodecInto<BigUint<Env::Api>>,
        Arg2: CodecInto<common_structs::farm_types::FarmTokenAttributes<Env::Api>>,
    >(
        self,
        user: Arg0,
        farm_token_amount: Arg1,
        attributes: Arg2,
    ) -> TxProxyCall<Env, From, To, Gas, BigUint<Env::Api>> {
        self.wrapped_tx
            .raw_call("calculateRewardsForGivenPosition")
            .argument(&user)
            .argument(&farm_token_amount)
            .argument(&attributes)
            .original_result()
    }

    pub fn reward_per_share(
        self,
    ) -> TxProxyCall<Env, From, To, Gas, BigUint<Env::Api>> {
        self.wrapped_tx
            .raw_call("getRewardPerShare")
            .original_result()
    }

    pub fn reward_reserve(
        self,
    ) -> TxProxyCall<Env, From, To, Gas, BigUint<Env::Api>> {
        self.wrapped_tx
            .raw_call("getRewardReserve")
            .original_result()
    }

    pub fn allow_external_claim_boosted_rewards<
        Arg0: CodecInto<bool>,
    >(
        self,
        allow_external_claim: Arg0,
    ) -> TxProxyCall<Env, From, To, Gas, ()> {
        self.wrapped_tx
            .raw_call("allowExternalClaimBoostedRewards")
            .argument(&allow_external_claim)
            .original_result()
    }

    pub fn get_allow_external_claim_rewards<
        Arg0: CodecInto<ManagedAddress<Env::Api>>,
    >(
        self,
        user: Arg0,
    ) -> TxProxyCall<Env, From, To, Gas, bool> {
        self.wrapped_tx
            .raw_call("getAllowExternalClaimRewards")
            .argument(&user)
            .original_result()
    }

    pub fn farming_token_id(
        self,
    ) -> TxProxyCall<Env, From, To, Gas, TokenIdentifier<Env::Api>> {
        self.wrapped_tx
            .raw_call("getFarmingTokenId")
            .original_result()
    }

    pub fn reward_token_id(
        self,
    ) -> TxProxyCall<Env, From, To, Gas, TokenIdentifier<Env::Api>> {
        self.wrapped_tx
            .raw_call("getRewardTokenId")
            .original_result()
    }

    pub fn per_block_reward_amount(
        self,
    ) -> TxProxyCall<Env, From, To, Gas, BigUint<Env::Api>> {
        self.wrapped_tx
            .raw_call("getPerBlockRewardAmount")
            .original_result()
    }

    pub fn last_reward_block_nonce(
        self,
    ) -> TxProxyCall<Env, From, To, Gas, u64> {
        self.wrapped_tx
            .raw_call("getLastRewardBlockNonce")
            .original_result()
    }

    pub fn division_safety_constant(
        self,
    ) -> TxProxyCall<Env, From, To, Gas, BigUint<Env::Api>> {
        self.wrapped_tx
            .raw_call("getDivisionSafetyConstant")
            .original_result()
    }

    pub fn user_total_farm_position<
        Arg0: CodecInto<ManagedAddress<Env::Api>>,
    >(
        self,
        user: Arg0,
    ) -> TxProxyCall<Env, From, To, Gas, config::UserTotalFarmPosition<Env::Api>> {
        self.wrapped_tx
            .raw_call("getUserTotalFarmPosition")
            .argument(&user)
            .original_result()
    }

    pub fn farm_position_migration_nonce(
        self,
    ) -> TxProxyCall<Env, From, To, Gas, u64> {
        self.wrapped_tx
            .raw_call("getFarmPositionMigrationNonce")
            .original_result()
    }

    pub fn set_locking_sc_address<
        Arg0: CodecInto<ManagedAddress<Env::Api>>,
    >(
        self,
        new_address: Arg0,
    ) -> TxProxyCall<Env, From, To, Gas, ()> {
        self.wrapped_tx
            .raw_call("setLockingScAddress")
            .argument(&new_address)
            .original_result()
    }

    pub fn set_lock_epochs<
        Arg0: CodecInto<u64>,
    >(
        self,
        lock_epochs: Arg0,
    ) -> TxProxyCall<Env, From, To, Gas, ()> {
        self.wrapped_tx
            .raw_call("setLockEpochs")
            .argument(&lock_epochs)
            .original_result()
    }

    pub fn locking_sc_address(
        self,
    ) -> TxProxyCall<Env, From, To, Gas, ManagedAddress<Env::Api>> {
        self.wrapped_tx
            .raw_call("getLockingScAddress")
            .original_result()
    }

    pub fn lock_epochs(
        self,
    ) -> TxProxyCall<Env, From, To, Gas, u64> {
        self.wrapped_tx
            .raw_call("getLockEpochs")
            .original_result()
    }

    pub fn register_farm_token<
        Arg0: CodecInto<ManagedBuffer<Env::Api>>,
        Arg1: CodecInto<ManagedBuffer<Env::Api>>,
        Arg2: CodecInto<usize>,
    >(
        self,
        token_display_name: Arg0,
        token_ticker: Arg1,
        num_decimals: Arg2,
    ) -> TxProxyCall<Env, From, To, Gas, ()> {
        self.wrapped_tx
            .raw_call("registerFarmToken")
            .argument(&token_display_name)
            .argument(&token_ticker)
            .argument(&num_decimals)
            .original_result()
    }

    pub fn farm_token(
        self,
    ) -> TxProxyCall<Env, From, To, Gas, TokenIdentifier<Env::Api>> {
        self.wrapped_tx
            .raw_call("getFarmTokenId")
            .original_result()
    }

    pub fn farm_token_supply(
        self,
    ) -> TxProxyCall<Env, From, To, Gas, BigUint<Env::Api>> {
        self.wrapped_tx
            .raw_call("getFarmTokenSupply")
            .original_result()
    }

    pub fn add_to_pause_whitelist<
        Arg0: CodecInto<MultiValueEncoded<Env::Api, ManagedAddress<Env::Api>>>,
    >(
        self,
        address_list: Arg0,
    ) -> TxProxyCall<Env, From, To, Gas, ()> {
        self.wrapped_tx
            .raw_call("addToPauseWhitelist")
            .argument(&address_list)
            .original_result()
    }

    pub fn remove_from_pause_whitelist<
        Arg0: CodecInto<MultiValueEncoded<Env::Api, ManagedAddress<Env::Api>>>,
    >(
        self,
        address_list: Arg0,
    ) -> TxProxyCall<Env, From, To, Gas, ()> {
        self.wrapped_tx
            .raw_call("removeFromPauseWhitelist")
            .argument(&address_list)
            .original_result()
    }

    pub fn pause(
        self,
    ) -> TxProxyCall<Env, From, To, Gas, ()> {
        self.wrapped_tx
            .raw_call("pause")
            .original_result()
    }

    pub fn resume(
        self,
    ) -> TxProxyCall<Env, From, To, Gas, ()> {
        self.wrapped_tx
            .raw_call("resume")
            .original_result()
    }

    pub fn state(
        self,
    ) -> TxProxyCall<Env, From, To, Gas, pausable::State> {
        self.wrapped_tx
            .raw_call("getState")
            .original_result()
    }

    pub fn add_admin_endpoint<
        Arg0: CodecInto<ManagedAddress<Env::Api>>,
    >(
        self,
        address: Arg0,
    ) -> TxProxyCall<Env, From, To, Gas, ()> {
        self.wrapped_tx
            .raw_call("addAdmin")
            .argument(&address)
            .original_result()
    }

    pub fn remove_admin_endpoint<
        Arg0: CodecInto<ManagedAddress<Env::Api>>,
    >(
        self,
        address: Arg0,
    ) -> TxProxyCall<Env, From, To, Gas, ()> {
        self.wrapped_tx
            .raw_call("removeAdmin")
            .argument(&address)
            .original_result()
    }

    pub fn update_owner_or_admin_endpoint<
        Arg0: CodecInto<ManagedAddress<Env::Api>>,
    >(
        self,
        previous_owner: Arg0,
    ) -> TxProxyCall<Env, From, To, Gas, ()> {
        self.wrapped_tx
            .raw_call("updateOwnerOrAdmin")
            .argument(&previous_owner)
            .original_result()
    }

    pub fn permissions<
        Arg0: CodecInto<ManagedAddress<Env::Api>>,
    >(
        self,
        address: Arg0,
    ) -> TxProxyCall<Env, From, To, Gas, permissions_module::permissions::Permissions> {
        self.wrapped_tx
            .raw_call("getPermissions")
            .argument(&address)
            .original_result()
    }

    pub fn add_sc_address_to_whitelist<
        Arg0: CodecInto<ManagedAddress<Env::Api>>,
    >(
        self,
        address: Arg0,
    ) -> TxProxyCall<Env, From, To, Gas, ()> {
        self.wrapped_tx
            .raw_call("addSCAddressToWhitelist")
            .argument(&address)
            .original_result()
    }

    pub fn remove_sc_address_from_whitelist<
        Arg0: CodecInto<ManagedAddress<Env::Api>>,
    >(
        self,
        address: Arg0,
    ) -> TxProxyCall<Env, From, To, Gas, ()> {
        self.wrapped_tx
            .raw_call("removeSCAddressFromWhitelist")
            .argument(&address)
            .original_result()
    }

    pub fn is_sc_address_whitelisted<
        Arg0: CodecInto<ManagedAddress<Env::Api>>,
    >(
        self,
        address: Arg0,
    ) -> TxProxyCall<Env, From, To, Gas, bool> {
        self.wrapped_tx
            .raw_call("isSCAddressWhitelisted")
            .argument(&address)
            .original_result()
    }

    pub fn set_penalty_percent<
        Arg0: CodecInto<u64>,
    >(
        self,
        percent: Arg0,
    ) -> TxProxyCall<Env, From, To, Gas, ()> {
        self.wrapped_tx
            .raw_call("set_penalty_percent")
            .argument(&percent)
            .original_result()
    }

    pub fn set_minimum_farming_epochs<
        Arg0: CodecInto<u64>,
    >(
        self,
        epochs: Arg0,
    ) -> TxProxyCall<Env, From, To, Gas, ()> {
        self.wrapped_tx
            .raw_call("set_minimum_farming_epochs")
            .argument(&epochs)
            .original_result()
    }

    pub fn set_burn_gas_limit<
        Arg0: CodecInto<u64>,
    >(
        self,
        gas_limit: Arg0,
    ) -> TxProxyCall<Env, From, To, Gas, ()> {
        self.wrapped_tx
            .raw_call("set_burn_gas_limit")
            .argument(&gas_limit)
            .original_result()
    }

    pub fn penalty_percent(
        self,
    ) -> TxProxyCall<Env, From, To, Gas, u64> {
        self.wrapped_tx
            .raw_call("getPenaltyPercent")
            .original_result()
    }

    pub fn minimum_farming_epochs(
        self,
    ) -> TxProxyCall<Env, From, To, Gas, u64> {
        self.wrapped_tx
            .raw_call("getMinimumFarmingEpoch")
            .original_result()
    }

    pub fn burn_gas_limit(
        self,
    ) -> TxProxyCall<Env, From, To, Gas, u64> {
        self.wrapped_tx
            .raw_call("getBurnGasLimit")
            .original_result()
    }

    pub fn pair_contract_address(
        self,
    ) -> TxProxyCall<Env, From, To, Gas, ManagedAddress<Env::Api>> {
        self.wrapped_tx
            .raw_call("getPairContractManagedAddress")
            .original_result()
    }

    pub fn collect_undistributed_boosted_rewards(
        self,
    ) -> TxProxyCall<Env, From, To, Gas, ()> {
        self.wrapped_tx
            .raw_call("collectUndistributedBoostedRewards")
            .original_result()
    }

    pub fn boosted_yields_rewards_percentage(
        self,
    ) -> TxProxyCall<Env, From, To, Gas, u64> {
        self.wrapped_tx
            .raw_call("getBoostedYieldsRewardsPercentage")
            .original_result()
    }

    pub fn accumulated_rewards_for_week<
        Arg0: CodecInto<usize>,
    >(
        self,
        week: Arg0,
    ) -> TxProxyCall<Env, From, To, Gas, BigUint<Env::Api>> {
        self.wrapped_tx
            .raw_call("getAccumulatedRewardsForWeek")
            .argument(&week)
            .original_result()
    }

    pub fn farm_supply_for_week<
        Arg0: CodecInto<usize>,
    >(
        self,
        week: Arg0,
    ) -> TxProxyCall<Env, From, To, Gas, BigUint<Env::Api>> {
        self.wrapped_tx
            .raw_call("getFarmSupplyForWeek")
            .argument(&week)
            .original_result()
    }

    pub fn remaining_boosted_rewards_to_distribute<
        Arg0: CodecInto<usize>,
    >(
        self,
        week: Arg0,
    ) -> TxProxyCall<Env, From, To, Gas, BigUint<Env::Api>> {
        self.wrapped_tx
            .raw_call("getRemainingBoostedRewardsToDistribute")
            .argument(&week)
            .original_result()
    }

    pub fn undistributed_boosted_rewards(
        self,
    ) -> TxProxyCall<Env, From, To, Gas, BigUint<Env::Api>> {
        self.wrapped_tx
            .raw_call("getUndistributedBoostedRewards")
            .original_result()
    }

    pub fn set_boosted_yields_factors<
        Arg0: CodecInto<BigUint<Env::Api>>,
        Arg1: CodecInto<BigUint<Env::Api>>,
        Arg2: CodecInto<BigUint<Env::Api>>,
        Arg3: CodecInto<BigUint<Env::Api>>,
        Arg4: CodecInto<BigUint<Env::Api>>,
    >(
        self,
        max_rewards_factor: Arg0,
        user_rewards_energy_const: Arg1,
        user_rewards_farm_const: Arg2,
        min_energy_amount: Arg3,
        min_farm_amount: Arg4,
    ) -> TxProxyCall<Env, From, To, Gas, ()> {
        self.wrapped_tx
            .raw_call("setBoostedYieldsFactors")
            .argument(&max_rewards_factor)
            .argument(&user_rewards_energy_const)
            .argument(&user_rewards_farm_const)
            .argument(&min_energy_amount)
            .argument(&min_farm_amount)
            .original_result()
    }

    pub fn get_boosted_yields_factors(
        self,
    ) -> TxProxyCall<Env, From, To, Gas, farm_boosted_yields::boosted_yields_factors::BoostedYieldsFactors<Env::Api>> {
        self.wrapped_tx
            .raw_call("getBoostedYieldsFactors")
            .original_result()
    }

    /// Week starts from 1 
    pub fn get_current_week(
        self,
    ) -> TxProxyCall<Env, From, To, Gas, usize> {
        self.wrapped_tx
            .raw_call("getCurrentWeek")
            .original_result()
    }

    pub fn first_week_start_epoch(
        self,
    ) -> TxProxyCall<Env, From, To, Gas, u64> {
        self.wrapped_tx
            .raw_call("getFirstWeekStartEpoch")
            .original_result()
    }

    pub fn get_last_active_week_for_user_view<
        Arg0: CodecInto<ManagedAddress<Env::Api>>,
    >(
        self,
        user: Arg0,
    ) -> TxProxyCall<Env, From, To, Gas, usize> {
        self.wrapped_tx
            .raw_call("getLastActiveWeekForUser")
            .argument(&user)
            .original_result()
    }

    pub fn get_user_energy_for_week_view<
        Arg0: CodecInto<ManagedAddress<Env::Api>>,
        Arg1: CodecInto<usize>,
    >(
        self,
        user: Arg0,
        week: Arg1,
    ) -> TxProxyCall<Env, From, To, Gas, OptionalValue<energy_factory::energy::Energy<Env::Api>>> {
        self.wrapped_tx
            .raw_call("getUserEnergyForWeek")
            .argument(&user)
            .argument(&week)
            .original_result()
    }

    pub fn last_global_update_week(
        self,
    ) -> TxProxyCall<Env, From, To, Gas, usize> {
        self.wrapped_tx
            .raw_call("getLastGlobalUpdateWeek")
            .original_result()
    }

    pub fn total_rewards_for_week<
        Arg0: CodecInto<usize>,
    >(
        self,
        week: Arg0,
    ) -> TxProxyCall<Env, From, To, Gas, ManagedVec<Env::Api, EsdtTokenPayment<Env::Api>>> {
        self.wrapped_tx
            .raw_call("getTotalRewardsForWeek")
            .argument(&week)
            .original_result()
    }

    pub fn total_energy_for_week<
        Arg0: CodecInto<usize>,
    >(
        self,
        week: Arg0,
    ) -> TxProxyCall<Env, From, To, Gas, BigUint<Env::Api>> {
        self.wrapped_tx
            .raw_call("getTotalEnergyForWeek")
            .argument(&week)
            .original_result()
    }

    pub fn total_locked_tokens_for_week<
        Arg0: CodecInto<usize>,
    >(
        self,
        week: Arg0,
    ) -> TxProxyCall<Env, From, To, Gas, BigUint<Env::Api>> {
        self.wrapped_tx
            .raw_call("getTotalLockedTokensForWeek")
            .argument(&week)
            .original_result()
    }

    pub fn update_energy_for_user<
        Arg0: CodecInto<ManagedAddress<Env::Api>>,
    >(
        self,
        user: Arg0,
    ) -> TxProxyCall<Env, From, To, Gas, ()> {
        self.wrapped_tx
            .raw_call("updateEnergyForUser")
            .argument(&user)
            .original_result()
    }

    pub fn current_claim_progress<
        Arg0: CodecInto<ManagedAddress<Env::Api>>,
    >(
        self,
        user: Arg0,
    ) -> TxProxyCall<Env, From, To, Gas, weekly_rewards_splitting::ClaimProgress<Env::Api>> {
        self.wrapped_tx
            .raw_call("getCurrentClaimProgress")
            .argument(&user)
            .original_result()
    }

    pub fn set_energy_factory_address<
        Arg0: CodecInto<ManagedAddress<Env::Api>>,
    >(
        self,
        sc_address: Arg0,
    ) -> TxProxyCall<Env, From, To, Gas, ()> {
        self.wrapped_tx
            .raw_call("setEnergyFactoryAddress")
            .argument(&sc_address)
            .original_result()
    }

    pub fn energy_factory_address(
        self,
    ) -> TxProxyCall<Env, From, To, Gas, ManagedAddress<Env::Api>> {
        self.wrapped_tx
            .raw_call("getEnergyFactoryAddress")
            .original_result()
    }
}
