elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use week_timekeeping::Week;

#[elrond_wasm::module]
pub trait FeesAccumulationModule:
    crate::config::ConfigModule
    + crate::events::FeesCollectorEventsModule
    + week_timekeeping::WeekTimekeepingModule
{
    /// Pair SC will deposit the fees through this endpoint
    /// Deposits for current week are accessible starting next week
    #[payable("*")]
    #[endpoint(depositSwapFees)]
    fn deposit_swap_fees(&self) {
        let caller = self.blockchain().get_caller();
        require!(
            self.known_contracts().contains(&caller),
            "Only known contracts can deposit"
        );

        let payment = self.call_value().single_esdt();
        require!(
            self.known_tokens().contains(&payment.token_identifier),
            "Invalid payment token"
        );

        let current_week = self.get_current_week();
        if payment.token_nonce == 0 {
            self.accumulated_fees(current_week, &payment.token_identifier)
                .update(|amt| *amt += &payment.amount);
        } else {
            self.accumulated_locked_fees(current_week, &payment.token_identifier)
                .update(|locked_fees| locked_fees.push(payment.clone()));
        }

        self.emit_deposit_swap_fees_event(
            caller,
            current_week,
            payment
        );
    }

    fn collect_accumulated_fees_for_week(
        &self,
        week: Week,
    ) -> ManagedVec<EsdtTokenPayment<Self::Api>> {
        let mut results = ManagedVec::new();
        let all_tokens = self.all_tokens().get();
        for token in &all_tokens {
            let opt_accumulated_fees = self.get_and_clear_acccumulated_fees(week, &token);
            if let Some(accumulated_fees) = opt_accumulated_fees {
                results.push(EsdtTokenPayment::new(token.clone(), 0, accumulated_fees));
            }

            let opt_accumulated_locked_fees =
                self.get_and_clear_acccumulated_locked_fees(week, &token);
            if let Some(accumulated_locked_fees) = opt_accumulated_locked_fees {
                results.append_vec(accumulated_locked_fees);
            }
        }
        results
    }

    fn get_and_clear_acccumulated_fees(
        &self,
        week: Week,
        token: &TokenIdentifier,
    ) -> Option<BigUint> {
        let mapper = self.accumulated_fees(week, token);
        let value = mapper.get();
        if value > 0 {
            mapper.clear();
            Some(value)
        } else {
            None
        }
    }

    fn get_and_clear_acccumulated_locked_fees(
        &self,
        week: Week,
        token: &TokenIdentifier,
    ) -> Option<ManagedVec<EsdtTokenPayment<Self::Api>>> {
        let mapper = self.accumulated_locked_fees(week, token);
        let value = mapper.get();
        if !value.is_empty() {
            mapper.clear();
            Some(value)
        } else {
            None
        }
    }

    #[view(getAccumulatedFees)]
    #[storage_mapper("accumulatedFees")]
    fn accumulated_fees(&self, week: Week, token: &TokenIdentifier) -> SingleValueMapper<BigUint>;

    #[view(getAccumulatedLockedFees)]
    #[storage_mapper("accumulatedLockedFees")]
    fn accumulated_locked_fees(
        &self,
        week: Week,
        token: &TokenIdentifier<Self::Api>,
    ) -> SingleValueMapper<ManagedVec<EsdtTokenPayment<Self::Api>>>;
}
