use parity_codec::Encode;
use support::{StorageValue, dispatch::Result, decl_module, decl_storage, decl_event};
use support::traits::{Currency, WithdrawReason, ExistenceRequirement};
use runtime_primitives::traits::{Zero, Hash, Saturating};
use system::ensure_signed;

pub trait Trait: balances::Trait {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

decl_storage! {
  trait Store for Module<T: Trait> as Demo {
    Payment get(payment): Option<T::Balance>;
    Pot get(pot): T::Balance;
    Nonce get(nonce): u64;
  }
}

decl_event! {
    pub enum Event<T> where <T as system::Trait>::AccountId, <T as balances::Trait>::Balance {
        PlayEvent(Balance, AccountId),
    }
}

decl_module! {
  pub struct Module<T: Trait> for enum Call where origin: T::Origin {

    fn deposit_event<T>() = default;

    fn set_payment(origin, value: T::Balance) -> Result {
        //Ensure signed
        let _ = ensure_signed(origin)?;

        //If payment is not initialized
        if Self::payment().is_none() {
            //Set the value of payment
            <Payment<T>>::put(value);

            //Initialize the pot with the same value
            <Pot<T>>::put(value);
        }

        //Return Ok on success
        Ok(())
    }

    fn play(origin) -> Result {
        //Ensure signed
        let sender = ensure_signed(origin)?;

        //Ensure payment has been set
        let payment = Self::payment().ok_or("Payment must be set")?;

        //Get our current storage values
        let mut nonce = Self::nonce();
        let mut pot = Self::pot();

        //Try to withdraw the payment, making sure it will not kill the account
        let _ = <balances::Module<T> as Currency<_>>::withdraw(&sender, payment, WithdrawReason::Reserve, ExistenceRequirement::KeepAlive)?;

        //Generate a random number between 0-255
        if(<system::Module<T>>::random_seed(), &sender, nonce)
            .using_encoded(<T as system::Trait>::Hashing::hash)
            .using_encoded(|e| e[0] < 128)
        {
            //If the user won, deposit the pot winnings in her account
            let _ = <balances::Module<T> as Currency<_>>::deposit_into_existing(&sender, pot)
                .expect("`sender` must exist since a transactoin is being made");

            //Reduce the pot to zero
            pot = Zero::zero();
        }

        //Increase the pot value by the payment
        pot = pot.saturating_add(payment);

        //Increment the nonce
        nonce = nonce.wrapping_add(1);

        //Store the updated values
        <Pot<T>>::put(pot);
        <Nonce<T>>::put(nonce);

        //Deposit event
        Self::deposit_event(RawEvent::PlayEvent(payment, sender));

        //Return ok
        Ok(())
    }
  } 
}
