#![cfg_attr(not(feature = "std"), no_std)]

use frame_system::Config;

pub trait OrderEventEmitter<T: Config> {
    fn emit_event_order_failed(order_id: &T::Hash) -> ();
}
