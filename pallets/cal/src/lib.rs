#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

// #[cfg(test)]
// mod mock;

// #[cfg(test)]
// mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::{*, OptionQuery};
	use frame_system::pallet_prelude::*;
	use frame_support::dispatch::EncodeLike;

	#[pallet::pallet]
	#[pallet::without_storage_info] 
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	// The pallet's runtime storage items.
	// https://docs.substrate.io/main-docs/build/runtime-storage/
	#[pallet::storage]
	#[pallet::getter(fn something)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/main-docs/build/runtime-somethingstorage/#declaring-storage-items
	pub type Something<T:Config> = StorageMap<_, Blake2_128Concat,T::AccountId,Result,OptionQuery>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored { something: u32, who: T::AccountId },
		CalculationPerformed(u32, u32, Calculator, u32),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,


	}
 #[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Encode, Decode, Debug, TypeInfo, MaxEncodedLen)]
	#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
	pub enum Calculator{

		Addition,
		Subtraction,
		Multiplication,
		Division


	}
	#[derive(Encode, Decode,TypeInfo)]
	pub struct Result {
		calculator: Calculator,
		something: u32,
		something2: u32,
		result: u32,
	}



	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		// #[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		// pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResult {
		// 	// Check that the extrinsic was signed and get the signer.
		// 	// This function will return an error if the extrinsic is not signed.
		// 	// https://docs.substrate.io/main-docs/build/origins/
		// 	let who = ensure_signed(origin)?;

		// 	// Update storage.
		// 	<Something<T>>::put(something);

		// 	// Emit an event.
		// 	Self::deposit_event(Event::SomethingStored { something, who });
		// 	// Return a successful DispatchResultWithPostInfo
		// 	Ok(())
		// }




		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn calculator(origin: OriginFor<T>, something: u32, something2 :u32, calculator: Option<Calculator> ) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/main-docs/build/origins/
			let who = ensure_signed(origin)?;


	let result = match calculator {
		Some(Calculator::Addition) => something + something2,
		Some(Calculator::Subtraction) => something - something2,
		Some(Calculator::Multiplication) => something * something2,
		Some(Calculator::Division) => something / something2,
		None => return Err(Error::<T>::NoneValue.into()),
			};

			let res = Result {
				calculator: calculator.clone().unwrap(),
				something,
				something2,
				result,
			};


			Something::<T>::insert(&who, &res);
		
		

			// Emit an event.
		    Self::deposit_event(Event::CalculationPerformed(
				something,
				something2,
			 calculator.clone().unwrap(),
				result,
			));
	
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		// An example dispatchable that may throw a custom error.
	//	#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1).ref_time())]
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
