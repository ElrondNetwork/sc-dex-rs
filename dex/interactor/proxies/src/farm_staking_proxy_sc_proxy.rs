// Code generated by the multiversx-sc proxy generator. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![allow(dead_code)]
#![allow(clippy::all)]

use multiversx_sc::proxy_imports::*;
use multiversx_sc_scenario::multiversx_sc;

pub struct FarmStakingProxyProxy;

impl<Env, From, To, Gas> TxProxyTrait<Env, From, To, Gas> for FarmStakingProxyProxy
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    type TxProxyMethods = FarmStakingProxyProxyMethods<Env, From, To, Gas>;

    fn proxy_methods(self, tx: Tx<Env, From, To, (), Gas, (), ()>) -> Self::TxProxyMethods {
        FarmStakingProxyProxyMethods { wrapped_tx: tx }
    }
}

pub struct FarmStakingProxyProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    wrapped_tx: Tx<Env, From, To, (), Gas, (), ()>,
}

#[rustfmt::skip]
impl<Env, From, Gas> FarmStakingProxyProxyMethods<Env, From, (), Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    Gas: TxGas<Env>,
{
    pub fn init<
        Arg0: CodecInto<ManagedAddress<Env::Api>>,
        Arg1: CodecInto<ManagedAddress<Env::Api>>,
        Arg2: CodecInto<ManagedAddress<Env::Api>>,
        Arg3: CodecInto<ManagedAddress<Env::Api>>,
        Arg4: CodecInto<TokenIdentifier<Env::Api>>,
        Arg5: CodecInto<TokenIdentifier<Env::Api>>,
        Arg6: CodecInto<TokenIdentifier<Env::Api>>,
        Arg7: CodecInto<TokenIdentifier<Env::Api>>,
    >(
        self,
        energy_factory_address: Arg0,
        lp_farm_address: Arg1,
        staking_farm_address: Arg2,
        pair_address: Arg3,
        staking_token_id: Arg4,
        lp_farm_token_id: Arg5,
        staking_farm_token_id: Arg6,
        lp_token_id: Arg7,
    ) -> TxProxyDeploy<Env, From, Gas, ()> {
        self.wrapped_tx
            .raw_deploy()
            .argument(&energy_factory_address)
            .argument(&lp_farm_address)
            .argument(&staking_farm_address)
            .argument(&pair_address)
            .argument(&staking_token_id)
            .argument(&lp_farm_token_id)
            .argument(&staking_farm_token_id)
            .argument(&lp_token_id)
            .original_result()
    }
}

#[rustfmt::skip]
impl<Env, From, To, Gas> FarmStakingProxyProxyMethods<Env, From, To, Gas>
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
impl<Env, From, To, Gas> FarmStakingProxyProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    pub fn register_dual_yield_token<
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
            .raw_call("registerDualYieldToken")
            .argument(&token_display_name)
            .argument(&token_ticker)
            .argument(&num_decimals)
            .original_result()
    }

    pub fn dual_yield_token(
        self,
    ) -> TxProxyCall<Env, From, To, Gas, TokenIdentifier<Env::Api>> {
        self.wrapped_tx
            .raw_call("getDualYieldTokenId")
            .original_result()
    }

    pub fn lp_farm_address(
        self,
    ) -> TxProxyCall<Env, From, To, Gas, ManagedAddress<Env::Api>> {
        self.wrapped_tx
            .raw_call("getLpFarmAddress")
            .original_result()
    }

    pub fn staking_farm_address(
        self,
    ) -> TxProxyCall<Env, From, To, Gas, ManagedAddress<Env::Api>> {
        self.wrapped_tx
            .raw_call("getStakingFarmAddress")
            .original_result()
    }

    pub fn pair_address(
        self,
    ) -> TxProxyCall<Env, From, To, Gas, ManagedAddress<Env::Api>> {
        self.wrapped_tx
            .raw_call("getPairAddress")
            .original_result()
    }

    pub fn staking_token_id(
        self,
    ) -> TxProxyCall<Env, From, To, Gas, TokenIdentifier<Env::Api>> {
        self.wrapped_tx
            .raw_call("getStakingTokenId")
            .original_result()
    }

    pub fn staking_farm_token_id(
        self,
    ) -> TxProxyCall<Env, From, To, Gas, TokenIdentifier<Env::Api>> {
        self.wrapped_tx
            .raw_call("getFarmTokenId")
            .original_result()
    }

    pub fn lp_token_id(
        self,
    ) -> TxProxyCall<Env, From, To, Gas, TokenIdentifier<Env::Api>> {
        self.wrapped_tx
            .raw_call("getLpTokenId")
            .original_result()
    }

    pub fn lp_farm_token_id(
        self,
    ) -> TxProxyCall<Env, From, To, Gas, TokenIdentifier<Env::Api>> {
        self.wrapped_tx
            .raw_call("getLpFarmTokenId")
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

    pub fn stake_farm_tokens<
        Arg0: CodecInto<OptionalValue<ManagedAddress<Env::Api>>>,
    >(
        self,
        opt_orig_caller: Arg0,
    ) -> TxProxyCall<Env, From, To, Gas, StakeProxyResult<Env::Api>> {
        self.wrapped_tx
            .raw_call("stakeFarmTokens")
            .argument(&opt_orig_caller)
            .original_result()
    }

    pub fn claim_dual_yield_endpoint<
        Arg0: CodecInto<OptionalValue<ManagedAddress<Env::Api>>>,
    >(
        self,
        opt_orig_caller: Arg0,
    ) -> TxProxyCall<Env, From, To, Gas, ClaimDualYieldResult<Env::Api>> {
        self.wrapped_tx
            .raw_call("claimDualYield")
            .argument(&opt_orig_caller)
            .original_result()
    }

    pub fn unstake_farm_tokens<
        Arg0: CodecInto<BigUint<Env::Api>>,
        Arg1: CodecInto<BigUint<Env::Api>>,
        Arg2: CodecInto<OptionalValue<ManagedAddress<Env::Api>>>,
    >(
        self,
        pair_first_token_min_amount: Arg0,
        pair_second_token_min_amount: Arg1,
        opt_orig_caller: Arg2,
    ) -> TxProxyCall<Env, From, To, Gas, UnstakeResult<Env::Api>> {
        self.wrapped_tx
            .raw_call("unstakeFarmTokens")
            .argument(&pair_first_token_min_amount)
            .argument(&pair_second_token_min_amount)
            .argument(&opt_orig_caller)
            .original_result()
    }
}

#[derive(TopEncode, TopDecode)]
pub struct StakeProxyResult<Api>
where
    Api: ManagedTypeApi,
{
    pub dual_yield_tokens: EsdtTokenPayment<Api>,
    pub staking_boosted_rewards: EsdtTokenPayment<Api>,
    pub lp_farm_boosted_rewards: EsdtTokenPayment<Api>,
}

#[derive(TopEncode, TopDecode)]
pub struct ClaimDualYieldResult<Api>
where
    Api: ManagedTypeApi,
{
    pub lp_farm_rewards: EsdtTokenPayment<Api>,
    pub staking_farm_rewards: EsdtTokenPayment<Api>,
    pub new_dual_yield_tokens: EsdtTokenPayment<Api>,
}

#[derive(TopEncode, TopDecode)]
pub struct UnstakeResult<Api>
where
    Api: ManagedTypeApi,
{
    pub other_token_payment: EsdtTokenPayment<Api>,
    pub lp_farm_rewards: EsdtTokenPayment<Api>,
    pub staking_rewards: EsdtTokenPayment<Api>,
    pub unbond_staking_farm_token: EsdtTokenPayment<Api>,
}
