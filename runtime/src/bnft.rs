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
  total_supply: u32,
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
#[derive(Encode, Decode, Default, Clone, PartialEq, Eq)]
pub struct Bnft<AccountId> {
  uri: AccountId,
  class_index: u32,
  verified: bool,
}

decl_storage! {
  trait Store for Module<T: Trait> as Bnft {
    ClassCursor get(classCursor): u32; 
    BnftCuror get(bnftCursor): u32;  
    BnftClasses get(get_bnft_class): map u32 => BnftClass<T::Hash, T::Balance, T::Moment, T::AccountId>;
    RemainingBnftsForClass get(remaining_bnfts_for): map u32 => u32;

    Bnfts get(get_bnft): map (T::AccountId, u32) => Bnft<T::AccountId>; 
    BnftIndex get(index_for): map (T::AccountId, u32) => u32

    BnftOwner get(owner_of): map (T::AccountId, u32) => Option<T::AccountId>;    
    OwnedBnftsArray get(owner_by_index): map u32 => AccountId;

    VerifiedBnfts get(get_verified_bnft): map T::AccountId => Bnft<T::AccountId>;
  }
}

decl_event! {
  pub enum Event<T> where 
    AccountId = <T as system::Trait>::AccountId, 
    Balance = <T as balances::Trait>::Balance,
    Hash = <T as system::Trait>::Hash,
    Moment = <T as timestamp::Trait>::Moment,
  {
    BnftClassCreated(u32, BnftClass<Hash, Balance, Moment, AccountId>),
    BnftIssued(AccountId, Bnft<AccountId>),  
  }
}

decl_module! {
  pub struct Module<T: Trait> for enum Call where origin: T::Origin {
    fn deposit_event<T>() = default;

    fn create_bnft_class(origin, 
                         name: T::Hash, 
                         total_supply: u32,
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

      //Ensure name is unique

      //Generate id for new bnft
      let mut nonce = Self::nonce();

      //Get creation time
      let now = <timestamp::Module<T>>::get();

      //Calculate expiry
      let expiry = now.checked_add(&validity).ok_or("Overflow when setting expiry")?;

      //Create struct
      let bnft_class = BnftClass {
        name: name.clone(),
        total_supply: total_supply.clone(),
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

      //Save BnftClass, remaining supply, nonce
      <BnftClasses<T>>::insert(nonce, bnft_class.clone());
      <RemainingBnftsForClass<T>>::insert(nonce, total_supply);

      //Emit event
      Self::deposit_event(RawEvent::BnftClassCreated(nonce, bnft_class));

      //Increment nonce
      nonce = nonce.wrapping_add(1);
      <ClassCursor<T>>::put(classCursor);

      Ok(())
    }

    fn issue_bnft(origin, 
                  class_index: u32, 
                  uri: T::AccountId) -> Result {
      //Ensure Signed
      let sender = ensure_signed(origin)?;

      //Ensure bnft class exists
      let classCursor = Self::classCursor();
      ensure!(class_index < classCursor, "BNFT Class does not exist!"); 

      // Ensure uri is unique
      ensure!(!<Bnfts<T>>::exists(&uri, &class_index), "Bnft already issued");

      // Ensure beneficiary has correct credential

      //Ensure total supply has not been exceeded
      let remainingBnftsForClass = Self::remaining_bnfts_for(class_index);
      ensure!(remainingBnftsForClass > 0, "All BNFTs have been issued for this class");

      // Transfer stake

      // Create bnft
      let bnft = Bnft {
        uri: uri.clone(),
        class_index,
        verified: false,
      };

      // Save bnft, assign to owner, decrement remaining supply
      <Bnfts<T>>::insert((uri.clone(), class_index.clone()), bnft.clone());
      <BnftIdByOwner<T>>::insert(sender.clone(), uri.clone());
      <RemainingBnftsForClass<T>>::insert(class_index, remainingBnftsForClass.clone() - 1);

      // Emit event
      Self::deposit_event(RawEvent::BnftIssued(sender, bnft));

      Ok(())
    }

    fn verifyAndBurn(origin,
                     agent: T::AccountId
                     class_index: u32,
                     uri: T::AccountId,) -> Result {
      //Ensure signed
      let sender = ensure_signed(origin)?;

      //Ensure BNFT exists
      ensure!(<Bnfts<T>>::exists(&uri), "Bnft does not exist or is already verified");

      //Ensure Agent owns BNFT
      let bnft = Self::get_verified_bnft(&uri).ok_or("Bnft not found")?;
      ensure!(bnft.

      //Verify verifier has required credential

      //Verify BNFT (Move to verified bnfts)

      //Remove from Owned BNFTs

      //Remove from Bnfts

      //Release stake back to agent

      //Award bounty to agent

      //Emit events

      Ok(())
    }
  } 
}

