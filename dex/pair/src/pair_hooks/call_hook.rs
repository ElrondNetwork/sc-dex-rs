use common_structs::PaymentsVec;

use super::hook_type::{Hook, HookType};

multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait CallHookModule {
    fn call_hook(
        &self,
        hook_type: HookType,
        caller: ManagedAddress,
        input_payments: PaymentsVec<Self::Api>,
        args: ManagedVec<ManagedBuffer>,
    ) -> PaymentsVec<Self::Api> {
        let hooks = self.hooks(hook_type).get();
        if hooks.is_empty() {
            return input_payments;
        }

        let payments_len = input_payments.len();
        let mut call_args = ManagedArgBuffer::new();
        call_args.push_arg(caller);

        for arg in &args {
            call_args.push_arg(arg);
        }

        let mut output_payments = input_payments;
        for hook in &hooks {
            let (_, back_transfers) =
                ContractCallNoPayment::<_, MultiValueEncoded<ManagedBuffer>>::new(
                    hook.dest_address,
                    hook.endpoint_name,
                )
                .with_raw_arguments(call_args.clone())
                .with_multi_token_transfer(output_payments)
                .execute_on_dest_context_with_back_transfers::<MultiValueEncoded<ManagedBuffer>>();

            output_payments = back_transfers.esdt_payments;
            require!(
                output_payments.len() == payments_len,
                "Wrong payments received from SC"
            );
        }

        output_payments
    }

    fn encode_arg_to_vec<T: TopEncode>(&self, arg: &T, vec: &mut ManagedVec<ManagedBuffer>) {
        let mut encoded_value = ManagedBuffer::new();
        let _ = arg.top_encode(&mut encoded_value);
        vec.push(encoded_value);
    }

    #[storage_mapper("hooks")]
    fn hooks(&self, hook_type: HookType) -> SingleValueMapper<ManagedVec<Hook<Self::Api>>>;
}
