#![cfg_attr(target_arch = "wasm32", no_std)]

//* Credit to evgenykuzyakov for writing low-level contract for FT transfers:
//* https://github.com/near/core-contracts/pull/88
//*
//* Credit to austinabell for nesdie and examples:
//* https://github.com/austinabell/nesdie

#[cfg(target_arch = "wasm32")]
use nesdie::sys;

const MAX_ACCOUNT_ID_LENGTH: usize = 64;

/// Transfers given amount of $NEAR to given account.
/// Input format is just a JSON string of the receiver account id: "<receiver_id:string>"
#[cfg(target_arch = "wasm32")]
#[no_mangle]
pub fn transfer_near() {
    let mut input_data = [0u8; MAX_ACCOUNT_ID_LENGTH + 2];
    let receiver_account_id = {
        unsafe {
            sys::input(0);
        }
        let input_data = if let Ok(input_data_len) = nesdie::env::read_register(0, &mut input_data)
        {
            &input_data[..input_data_len]
        } else {
            nesdie::env::panic_str("Invalid input data");
        };
        if input_data.len() < 3
            || input_data.first() != Some(&b'"')
            || input_data.last() != Some(&b'"')
        {
            nesdie::env::panic_str(
                "Invalid receiver account ID. Input argument to the transfer_near function must be a JSON string like \"root.near\".",
            );
        }
        // Strip the JSON quotes
        &input_data[1..input_data.len() - 1]
    };

    unsafe {
        let attached_deposit = [0u8; core::mem::size_of::<nesdie::Balance>()];
        sys::attached_deposit(attached_deposit.as_ptr() as u64);

        let promise_id = sys::promise_batch_create(
            receiver_account_id.len() as u64,
            receiver_account_id.as_ptr() as u64,
        );
        sys::promise_batch_action_transfer(promise_id, attached_deposit.as_ptr() as u64);
    }
}
