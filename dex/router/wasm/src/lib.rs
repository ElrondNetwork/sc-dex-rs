// Code generated by the multiversx-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Upgrade:                              1
// Endpoints:                           30
// Async Callback:                       1
// Total number of exported functions:  33

#![no_std]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    router
    (
        init => init
        upgrade => upgrade
        setPairTemplateAddress => set_pair_template_address
        setPairCreationEnabled => set_pair_creation_enabled
        setPairCreationDisabled => set_pair_creation_disabled
        getPairCreationEnabled => pair_creation_enabled
        getOwner => owner
        getPairTemplateAddress => pair_template_address
        getCommonTokensForUserPairs => common_tokens_for_user_pairs
        configEnableByUserParameters => config_enable_by_user_parameters
        addCommonTokensForUserPairs => add_common_tokens_for_user_pairs
        removeCommonTokensForUserPairs => remove_common_tokens_for_user_pairs
        setSwapEnabledByUser => set_swap_enabled_by_user
        getEnableSwapByUserConfig => try_get_config
        multiPairSwap => multi_pair_swap
        createPair => create_pair_endpoint
        upgradePair => upgrade_pair_endpoint
        issueLpToken => issue_lp_token
        setLocalRoles => set_local_roles
        setFeeOn => set_fee_on
        setFeeOff => set_fee_off
        removePair => remove_pair
        pause => pause
        resume => resume
        getState => state
        setTemporaryOwnerPeriod => set_temporary_owner_period
        clearPairTemporaryOwnerStorage => clear_pair_temporary_owner_storage
        getTemporaryOwnerPeriod => temporary_owner_period
        getAllPairsManagedAddresses => get_all_pairs_addresses
        getAllPairTokens => get_all_token_pairs
        getAllPairContractMetadata => get_all_pair_contract_metadata
        getPair => get_pair
    )
}

multiversx_sc_wasm_adapter::async_callback! { router }
