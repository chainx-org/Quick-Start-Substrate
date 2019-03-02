use support::{decl_event, decl_module, decl_storage, dispatch::Result, StorageValue};
use system::ensure_signed;

pub trait Trait: balances::Trait {
    // TODO: Add other types and constants required configure this module.
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

/// This module's storage items.
decl_storage! {
    trait Store for Module<T: Trait> as Game {
        // `get(count)` is the default getter which returns either the stored `u32`
        Count get(count) build(|_| 0u32): u32;
    }
}

decl_event!(
    /// An event in this module.
    pub enum Event<T>
    where
        AccountId = <T as system::Trait>::AccountId,
    {
        // Event `Win` is declared with a parameter of the type `AccountId` and `u32`
        Win(AccountId, u32),
    }
);

decl_module! {
    /// The module declaration.
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        // Initializing events
        // this is needed only if you are using events in your module
        fn deposit_event<T>() = default;

        // function that can be called by the external world as an extrinsics call
        // takes a parameter of the type `AccountId`, stores it and emits an event
        pub fn play(origin, lucky: u32) -> Result {
            // TODO: You only need this if you want to check it was signed.
            let who = ensure_signed(origin)?;
            //make sure < 10
            let lucky=lucky%10;
            //User must pays 10 coins
            <balances::Module<T>>::decrease_free_balance(&who, 10)?;

            // Then we flip a coin by generating a random seed
            // We pass the seed with our sender's account id into a hash algorithm
            // Then we check if the first byte of the hash is less than 128
            if (<system::Module<T>>::random_seed())
            .using_encoded(|e| e[0] % 10 == lucky)
            {
                //Catch Lucky , Double Coin Back!
                <balances::Module<T>>::increase_free_balance_creating(&who, 20);

                //update  count
                let count=<Count<T>>::get();
                count=count+1;
                <Count<T>>::put(count);
            }

            // here we are raising the Something event
            Self::deposit_event(RawEvent::Win(who, lucky));
            Ok(())
        }
    }
}
