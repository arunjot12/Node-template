#![cfg_attr(not(feature = "std"), no_std)]
// extern crate amiquip;
/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;
//  use amiquip::{Connection, Exchange, Publish, Result};
// extern crate amiquip;
pub use amiquip;


 
#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;


#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
    use scale_info::prelude::string::String;
    use frame_support::inherent::Vec;
    // use amiquip::{Connection, Exchange, Publish, Result};
	use frame_system::pallet_prelude::*;


    #[cfg(all(feature = "std"))]
    use amiquip::{Connection, Exchange, Publish, Result};
   
    // use amiquip;
    // use std::str;
    //pub use amiquip::{Connection, Exchange, Publish, Result};

    #[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);


/// Configure the pallet by specifying the parameters and types on which it depends.
#[pallet::config]
pub trait Config: frame_system::Config {
    /// Because this pallet emits events, it depends on the runtime's definition of an event.
    type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
}

// Errors inform users that something went wrong.
#[pallet::error]
pub enum Error<T> {
    /// Error names should be descriptive.
    NoneValue,
    /// Errors should have helpful documentation associated with them.
    StorageOverflow,
    InvalidMessage,
    CouldNotConnect,
    PublishFailed,
}

#[pallet::event]
#[pallet::generate_deposit(pub(super) fn deposit_event)]
pub enum Event<T: Config> {
    /// Event documentation should end with an array that provides descriptive names for event
    /// parameters. [something, who]
    MessagePublished(T::AccountId),
}


#[pallet::call]
impl<T: Config> Pallet<T> {
    #[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
        pub fn publish_message(origin: OriginFor<T>, message: Vec<u8>, routing_key: Vec<u8>) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            ensure!(!message.is_empty(), Error::<T>::InvalidMessage);

            // Open connection.
            let mut connection = amiquip::Connection::insecure_open("amqp://guest:guest@<RECEIVER_IP>:5672")
                .map_err(|_| Error::<T>::CouldNotConnect)?;

            // Open a channel - None says let the library choose the channel ID.
            let channel = connection.open_channel(None)
                .map_err(|_| Error::<T>::CouldNotConnect)?;

            // Get a handle to the direct exchange on our channel.
            let exchange = Exchange::direct(&channel);

            // let message_str = unsafe { str::get_unchecked(&message) };
            let routing_key_str: String = routing_key.iter().map(|&c| c as char).collect();

    
            // Publish a message to the specified routing key.
            exchange.publish(Publish::new(&message, &routing_key_str))
                .map_err(|_| Error::<T>::PublishFailed)?;

            Self::deposit_event(Event::MessagePublished(sender));
            Ok(())
        }
    }
}
