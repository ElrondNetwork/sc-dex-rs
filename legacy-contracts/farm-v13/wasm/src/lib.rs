// Code generated by the multiversx-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           39
// Async Callback (empty):               1
// Total number of exported functions:  41

#![no_std]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    farm_v13
    (
        init => init
        enterFarm => enter_farm
        exitFarm => exit_farm
        claimRewards => claim_rewards
        compoundRewards => compound_rewards
        mergeFarmTokens => merge_farm_tokens
        calculateRewardsForGivenPosition => calculate_rewards_for_given_position
        end_produce_rewards => end_produce_rewards
        setPerBlockRewardAmount => set_per_block_rewards
        set_penalty_percent => set_penalty_percent
        set_minimum_farming_epochs => set_minimum_farming_epochs
        set_transfer_exec_gas_limit => set_transfer_exec_gas_limit
        set_burn_gas_limit => set_burn_gas_limit
        pause => pause
        resume => resume
        startProduceRewards => start_produce_rewards_as_owner
        setFarmTokenSupply => set_farm_token_supply
        setFarmMigrationConfig => set_farm_migration_config
        registerFarmToken => register_farm_token
        setLocalRolesFarmToken => set_local_roles_farm_token
        setRpsAndStartRewards => set_rps_and_start_rewards
        migrateFromV1_2Farm => migrate_from_v1_2_farm
        getFarmMigrationConfiguration => farm_migration_config
        getFarmTokenSupply => farm_token_supply
        getLastErrorMessage => last_error_message
        getState => state
        getFarmingTokenId => farming_token_id
        getRewardTokenId => reward_token_id
        getPenaltyPercent => penalty_percent
        getMinimumFarmingEpoch => minimum_farming_epochs
        getPerBlockRewardAmount => per_block_reward_amount
        getLastRewardBlockNonce => last_reward_block_nonce
        getFarmTokenId => farm_token_id
        getDivisionSafetyConstant => division_safety_constant
        getPairContractManagedAddress => pair_contract_address
        getBurnGasLimit => burn_gas_limit
        getLockedAssetFactoryManagedAddress => locked_asset_factory_address
        getRewardPerShare => reward_per_share
        getRewardReserve => reward_reserve
        getTransferExecGasLimit => transfer_exec_gas_limit
    )
}

multiversx_sc_wasm_adapter::async_callback_empty! {}
