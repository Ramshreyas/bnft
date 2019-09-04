use parity_codec::{Decode, Encode};
use support::{StorageValue, StorageMap, ensure, dispatch::Result, decl_module, decl_storage, decl_event};
use support::traits::{Currency, WithdrawReason, ExistenceRequirement};
use runtime_primitives::traits::{Zero, Hash, Saturating, As, CheckedAdd, CheckedMul, CheckedDiv};
use {system::ensure_signed, timestamp};
use rstd::prelude::*;

pub trait Trait: balances::Trait + timestamp::Trait {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Encode, Decode, Default, Clone, PartialEq, Eq)]
pub struct BnftClass<Hash, Balance, Moment, AccountId> {
    name: Hash,
    total_supply: u64,
    beneficiary_credential: Hash,
    verifier_credential: Hash,
    transfer_bounty: Balance,
    verification_bounty: Balance,
    stake: Balance,
    expiry: Moment,
    description: Hash,
    ricardian_contract: Hash,
    creator: AccountId,
    created_on: Moment,
}

#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Encode, Decode, Default, Clone, PartialEq)]
pub struct Bnft<Hash> {
    uri: Hash,
    class_id: Hash,
}

decl_storage! {
  trait Store for Module<T: Trait> as Bnft {
    Nonce get(nonce): u64;
    BnftClasses get(get_bnft_class): map u64 => BnftClass<T::Hash, T::Balance, T::Moment, T::AccountId>;
    Bnfts get(get_bnft): map T::AccountId => Bnft<T::Hash>; 
  }
}

decl_event! {
    pub enum Event<T> where 
      AccountId = <T as system::Trait>::AccountId, 
      Balance = <T as balances::Trait>::Balance,
      Hash = <T as system::Trait>::Hash,
      Moment = <T as timestamp::Trait>::Moment,
    {
        BnftClassCreated(u64, BnftClass<Hash, Balance, Moment, AccountId>),
    }
}

decl_module! {
  pub struct Module<T: Trait> for enum Call where origin: T::Origin {

    fn deposit_event<T>() = default;

    fn create_bnft_class(origin, 
                         name: T::Hash, 
                         total_supply: u64,
                         beneficiary_credential: T::Hash,
                         verifier_credential: T::Hash,
                         transfer_bounty: T::Balance,
                         verification_bounty: T::Balance,
                         stake: T::Balance,
                         validity: T::Moment,
                         description: T::Hash,
                         ricardian_contract: T::Hash) -> Result {
        //Ensure signed
        let sender = ensure_signed(origin)?;

        //Generate id for new bnft
        let mut nonce = Self::nonce();

        //Get creation time
        let now = <timestamp::Module<T>>::get();

        //Calculate expiry
        let expiry = now.checked_add(&validity).ok_or("Overflow when setting expiry")?;

        //Create struct
        let bnft_class = BnftClass {
            name: name.clone(),
            total_supply,
            beneficiary_credential,
            verifier_credential,
            transfer_bounty,
            verification_bounty,
            stake,
            expiry,
            description,
            ricardian_contract,
            creator: sender.clone(),
            created_on: now,
        };

        //Transfer payment for creation
        

        //Save struct, nonce
        <BnftClasses<T>>::insert(nonce, bnft_class.clone());
        nonce = nonce.wrapping_add(1);
        <Nonce<T>>::put(nonce);

        //Emit event
        Self::deposit_event(RawEvent::BnftClassCreated(nonce, bnft_class));

        Ok(())
    }

    //fn issue_bnft(origin, bnft_class_index: u64, uri: T::Hash) -> Result {
        //Ensure Signed
        //let sender = ensure_signed(origin)?;

        //Ensure bnft class exists

        //Ensure uri is unique

        //Ensure beneficiary has correct credential

        //Transfer stake

        //Create bnft

        //Save bnft

        //Emit event

        //Ok(())
    //} 
  } 
}

