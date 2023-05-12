#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;



#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::{pallet_prelude::*, pallet};
	use frame_support::inherent::Vec;
	use codec::alloc::string::ToString;
	use frame_support::traits::Currency;
	// use frame_support::traits::tokens::Balance;



	#[pallet::pallet]
	#[pallet::without_storage_info] 
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_balances::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		// The balance of an account.
		// type Balance: Parameter
		// 	+ Member
		// 	//+ AtLeast32BitUnsigned
		// 	//+ Codec
		// 	+ Default
		// 	+ Copy
		// 	+ MaybeSerializeDeserialize
		// 	//+ Debug
		// 	+ MaxEncodedLen
		// 	+ TypeInfo;

			type Currency: Currency<Self::AccountId>;
		
	}

	// The pallet's runtime storage items.
	// https://docs.substrate.io/main-docs/build/runtime-storage/
	#[pallet::storage]
	#[pallet::getter(fn number)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/main-docs/build/runtime-storage/#declaring-storage-items
	pub type Number<T> = StorageMap<_, Blake2_128Concat,u32,Vec<u8>,ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn hello)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/main-docs/build/runtime-storage/#declaring-storage-items
	pub type Hello<T> = StorageValue<_,Vec<u8>>;

	#[pallet::storage]
	#[pallet::getter(fn new)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/main-docs/build/runtime-storage/#declaring-storage-items
	pub type NewStorage<T> = StorageValue<_, u32>;


	// #[pallet::storage]
	// #[pallet::getter(fn data)]
	// // Learn more about declaring storage items:
	// // https://docs.substrate.io/main-docs/build/runtime-storage/#declaring-storage-items
	// pub type Data<T> = StorageValue<_, Vec<u8>>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored (T::AccountId ,u32,Vec<u8> ),
		ChangesUpdated (T::AccountId ,u32,Vec<u8> ),
		Removed (T::AccountId) ,
		Data (Vec<(u32, Vec<u8>)>),
		Hello(Vec<u8>),
		 Balance(T::AccountId,T::Balance),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
		DataNotFound,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn store_data(origin: OriginFor<T>, number: u32, data: Vec<u8>) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/main-docs/build/origins/
			let who = ensure_signed(origin)?;

			// Update storage.
			<Number<T>>::insert(number,data.clone());
			//<Data<T>>::put(data.clone());

			// Emit an event.
			Self::deposit_event(Event::SomethingStored ( who.clone(),number.clone(),data.clone() ));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn hello_world(origin: OriginFor<T>,) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/main-docs/build/origins/
			let who = ensure_signed(origin)?;

			let my_string ="Hello,World";

			// for byte in my_string.bytes() {
			// 	data.push(byte);
			// }

			let my_string = "Hello, World".to_string();
			let my_bytes = my_string.as_bytes().to_vec();

			<Hello<T>>::put(my_bytes);
		

			// Emit an event.
			Self::deposit_event(Event::Hello( my_string.into()));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}


		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn show_balance(origin: OriginFor<T>) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/main-docs/build/origins/
			let who = ensure_signed(origin)?;


			let total_balances = pallet_balances::Pallet::<T>::free_balance(&who);
			// let _total_balances 
			// = T::Currency::total_issuance();

			// Emit an event.
			Self::deposit_event(Event::Balance(who,total_balances.clone()));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}


		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn update_data(origin: OriginFor<T>, number: u32, data: Vec<u8>) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			let who = ensure_signed(origin)?;
		
			// Update storage.
			<Number<T>>::insert(number, data.clone());
		
			// Emit an event.
			Self::deposit_event(Event::ChangesUpdated(who.clone(), number.clone(), data.clone()));
		
			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn remove_data(origin: OriginFor<T>) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			let who = ensure_signed(origin)?;

			for (key, _) in <Number<T>>::iter() {
				<Number<T>>::remove(&key);
			};

		
			// Remove the value associated with the key from storage.
			// <Number<T>>::remove(&key.encode());
		
			// Update storage.
			// <Number<T>>::remove(number);
			//<Number<T>>::remove(&data);
		
			// Emit an event.
			Self::deposit_event(Event::Removed(who));
		
			Ok(())
		}


		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn show_data(origin: OriginFor<T> ) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			let who = ensure_signed(origin)?;
		
			
    // Get all keys stored in the Number storage
	let data = <Number<T>>::iter().collect::<Vec<_>>();

    // Print the key-value pairs to the debug console.

			// Emit an event.
			Self::deposit_event(Event::Data(data.clone()));
		
			Ok(())
		}

		

		
		// #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1).ref_time())]
		// pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
		// 	let _who = ensure_signed(origin)?;

		// 	// Read a value from storage.
		// 	match <Something<T>>::get() {
		// 		// Return an error if the value has not been set.
		// 		None => return Err(Error::<T>::NoneValue.into()),
		// 		Some(old) => {
		// 			// Increment the value read from storage; will error in the event of overflow.
		// 			let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
		// 			// Update the value in storage with the incremented result.
		// 			<Something<T>>::put(new);
		// 			Ok(())
		// 		},
		// 	}
		// }
	}
}
