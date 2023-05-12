use frame_support::{decl_error, decl_event, decl_module, dispatch::DispatchResult, ensure};
use frame_system::{self as system, ensure_signed};
use sp_std::vec::Vec;
use amiquip::{Connection, Exchange, Publish, Result};

#[cfg(test)]
mod tests;

pub trait Config: system::Config {
    type Event: From<Event<Self>> + Into<<Self as system::Config>::Event>;
}

decl_error! {
    pub enum Error for Module<T: Config> {
        CouldNotConnect,
        InvalidMessage,
        PublishFailed,
    }
}

decl_event!(
    pub enum Event<T> where AccountId = <T as system::Config>::AccountId {
        MessagePublished(AccountId),
    }
);

decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {
        fn deposit_event() = default;

        #[weight = 10_000]
        pub fn publish_message(origin, message: Vec<u8>, routing_key: Vec<u8>) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            ensure!(!message.is_empty(), Error::<T>::InvalidMessage);

            // Open connection.
            let mut connection = Connection::insecure_open("amqp://guest:guest@<RECEIVER_IP>:5672")
                .map_err(|_| Error::<T>::CouldNotConnect)?;

            // Open a channel - None says let the library choose the channel ID.
            let channel = connection.open_channel(None)
                .map_err(|_| Error::<T>::CouldNotConnect)?;

            // Get a handle to the direct exchange on our channel.
            let exchange = Exchange::direct(&channel);

            // Publish a message to the specified routing key.
            exchange.publish(Publish::new(&message, &routing_key))
                .map_err(|_| Error::<T>::PublishFailed)?;

            Self::deposit_event(RawEvent::MessagePublished(sender));
            Ok(())
        }
    }
}
