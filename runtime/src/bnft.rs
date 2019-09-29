use parity_codec::{Decode, Encode};
use support::{StorageValue, StorageMap, ensure, dispatch::Result, decl_module, decl_storage, decl_event};
use support::traits::{Currency, WithdrawReason, ExistenceRequirement};
use runtime_primitives::traits::{Zero, Hash, Saturating, As, CheckedAdd, CheckedMul, CheckedDiv};
use {system::ensure_signed, timestamp};
use rstd::prelude::*;
use crate::token;

pub trait Trait: balances::Trait + timestamp::Trait + token::Trait {
  type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Encode, Decode, Default, Clone, PartialEq, Eq)]
pub struct BnftClass<Hash, Balance, Moment, AccountId> {
  name: Hash,
  total_supply: Balance,
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
  funded: bool,
  funded_on: Option<Moment>,
  funding_period: Moment,
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
    // stores the owner and admins in the genesis config and after init()
    Owner get(owner) config(): T::AccountId;
    Admins get(admins): map T::AccountId => bool;

    //Current index for Bnft Classes & Bnfts
    ClassCursor get(classCursor): u32;
    BnftCursor get(bnftCursor): u32;

    //Bnft Class storage
    BnftClasses get(get_bnft_class): map u32 => BnftClass<T::Hash, T::TokenBalance, T::Moment, T::AccountId>;
    RemainingBnftsForClass get(remaining_bnfts_for): map u32 => u64;

    //Issued Bnft Storage
    Bnfts get(get_bnft): map (T::AccountId, u32) => Bnft<T::AccountId>; 
    BnftIndex get(get_bnft_index_for): map (T::AccountId, u32) => u32;
    VerifiedBnfts get(get_verified_bnft): map T::AccountId => Bnft<T::AccountId>;

    //Agent ownership storage
    BnftOwner get(owner_of): map (T::AccountId, u32) => Option<T::AccountId>;    
    OwnedBnftsCount get(bnft_count_for): map T::AccountId => u32;
    OwnedBnftsArray get(get_bnft_for): map (T::AccountId, u32) => (T::AccountId, u32);

    //Funders storage
    BnftClassFunder get(funder_of): map u32 => T::AccountId;
  }
}

decl_event! {
  pub enum Event<T> where 
    AccountId = <T as system::Trait>::AccountId, 
    Balance = <T as token::Trait>::TokenBalance,
    Hash = <T as system::Trait>::Hash,
    Moment = <T as timestamp::Trait>::Moment,
  {
    BnftClassCreated(u32, BnftClass<Hash, Balance, Moment, AccountId>),
    BnftClassFunded(u32, AccountId, BnftClass<Hash, Balance, Moment, AccountId>),
    BnftIssued(AccountId, Bnft<AccountId>),  
    BnftVerified(AccountId, AccountId, Bnft<AccountId>),
  }
}

decl_module! {
  pub struct Module<T: Trait> for enum Call where origin: T::Origin {
    fn deposit_event<T>() = default;

    fn init(origin) {
      let sender = ensure_signed(origin)?;
      ensure!(sender == Self::owner(), "Only the owner set in genesis config can initialize the TCR");
      <token::Module<T>>::init(sender.clone())?;
      <Admins<T>>::insert(sender, true);
    }

    fn create_bnft_class(origin, 
                         name: T::Hash, 
                         total_supply: u64,
                         beneficiary_credential: T::Hash,
                         verifier_credential: T::Hash,
                         transfer_bounty: u64,
                         verification_bounty: u64,
                         stake: u64,
                         validity: T::Moment,
                         description: T::Hash,
                         ricardian_contract: T::Hash,
                         funding_period: T::Moment) -> Result {
      //Ensure signed
      let sender = ensure_signed(origin)?;

      //Ensure name is unique

      //Generate id for new bnft
      let mut classCursor = Self::classCursor();

      //Get creation time
      let now = <timestamp::Module<T>>::get();

      //Calculate expiry
      let expiry = now.checked_add(&validity).ok_or("Overflow when setting expiry")?;

      //Create struct
      let bnft_class = BnftClass {
        name: name.clone(),
        total_supply: <T::TokenBalance as As<u64>>::sa(total_supply),
        beneficiary_credential,
        verifier_credential,
        transfer_bounty: <T::TokenBalance as As<u64>>::sa(transfer_bounty),
        verification_bounty: <T::TokenBalance as As<u64>>::sa(verification_bounty),
        stake: <T::TokenBalance as As<u64>>::sa(stake),
        expiry,
        description,
        ricardian_contract,
        creator: sender.clone(),
        created_on: now.clone(),
        funded: false,
        funded_on: None,
        funding_period,
      };

      //Transfer payment for creation    

      //Save BnftClass, remaining supply, classCursor
      <BnftClasses<T>>::insert(classCursor, bnft_class.clone());
      <RemainingBnftsForClass<T>>::insert(classCursor, total_supply);

      //Emit event
      Self::deposit_event(RawEvent::BnftClassCreated(classCursor, bnft_class));

      //Increment classCursor
      let classCursor = classCursor.wrapping_add(1);
      <ClassCursor<T>>::put(classCursor);

      Ok(())
    }

    fn fund_bnft_class(origin,
                       class_index: u32) -> Result {
      //Ensure Signed
      let sender = ensure_signed(origin)?;

      //Ensure bnft class exists
      let classCursor = Self::classCursor();
      ensure!(class_index < classCursor, "BNFT Class does not exist!"); 

      //Ensure not funded already
      let mut bnftClass = Self::get_bnft_class(class_index);
      ensure!(!bnftClass.funded, "BNFT Class is already funded");

      //Ensure not expired
      let now = <timestamp::Module<T>>::get();
      let fundingDeadline = bnftClass.created_on.checked_add(&bnftClass.funding_period).ok_or("Error!")?;
      ensure!(fundingDeadline > now, "BnftClass has expired!");

      //Transfer funds
      let transfer_bounty = bnftClass.transfer_bounty;
      let verification_bounty = bnftClass.verification_bounty;
      let total_supply = bnftClass.total_supply;
      let total_transfer_bounty = transfer_bounty.checked_mul(&total_supply).ok_or("Overflow")?;
      let total_verification_bounty = verification_bounty.checked_mul(&total_supply).ok_or("Overflow")?;
      let amount = total_transfer_bounty.checked_add(&total_verification_bounty).ok_or("Overflow")?;
      <token::Module<T>>::lock(sender.clone(), amount, (sender.clone(), class_index));
            
      //Update storage
      bnftClass.funded = true;
      <BnftClasses<T>>::insert(class_index, bnftClass.clone());
      <BnftClassFunder<T>>::insert(class_index, sender.clone());

      //Emit event
      Self::deposit_event(RawEvent::BnftClassFunded(class_index, sender, bnftClass));

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

      //Ensure BnftClass is funded
      let bnftClass = Self::get_bnft_class(class_index);
      ensure!(bnftClass.funded, "BNFT class is not yet funded!");

      // Ensure uri is unique
      let uriClassIndexTuple = (uri.clone(), class_index);
      ensure!(!<Bnfts<T>>::exists(&uriClassIndexTuple), "Bnft already issued");

      // Ensure beneficiary has correct credential

      //Ensure total supply has not been exceeded
      let remainingBnftsForClass = Self::remaining_bnfts_for(class_index);
      ensure!(remainingBnftsForClass > 0, "All BNFTs have been issued for this class");

      // Lock stake
      <token::Module<T>>::lock(sender.clone(), bnftClass.stake, uriClassIndexTuple.clone())?;

      // Create bnft
      let bnft = Bnft {
        uri: uri.clone(),
        class_index: class_index.clone(),
        verified: false,
      };

      // Update Bnft storage
      let mut bnftCursor = Self::bnftCursor();
      let accountIdBnftIndexTuple = (sender.clone(), bnftCursor.clone()); 
      <Bnfts<T>>::insert(&uriClassIndexTuple, &bnft);
      <BnftIndex<T>>::insert(&uriClassIndexTuple, &bnftCursor);
      
      //Update Bnft storage
      let bnftCount = Self::bnft_count_for(&sender).wrapping_add(1);
      <BnftOwner<T>>::insert(&uriClassIndexTuple, sender.clone());
      <OwnedBnftsCount<T>>::insert(&sender, &bnftCount);
      <OwnedBnftsArray<T>>::insert(accountIdBnftIndexTuple, uriClassIndexTuple.clone());

      //Increment BnftCursor
      <BnftCursor<T>>::put(bnftCursor.wrapping_add(1));

      //Decrement remaining Bnfts for class
      <RemainingBnftsForClass<T>>::insert(class_index, remainingBnftsForClass.clone() - 1);

      // Emit event
      Self::deposit_event(RawEvent::BnftIssued(sender, bnft));

      Ok(())
    }

    fn verifyAndBurn(origin,
                     agent: T::AccountId,
                     class_index: u32,
                     uri: T::AccountId) -> Result {
      //Ensure signed
      let sender = ensure_signed(origin)?;

      //Ensure BNFT exists
      let uriClassIndexTuple = (uri.clone(), class_index);
      ensure!(<Bnfts<T>>::exists(uriClassIndexTuple.clone()), "Bnft does not exist or is already verified");

      //Ensure Agent owns BNFT
      ensure!(Self::owner_of(uriClassIndexTuple.clone()).unwrap() == agent, "Agent does not own BNFT");   

      //Verify verifier has required credential
      
      //Remove from Bnfts
      let mut bnft = Self::get_bnft(uriClassIndexTuple.clone());
      <Bnfts<T>>::remove(uriClassIndexTuple.clone());

      //Verify BNFT (Move to verified bnfts)
      bnft.verified = true;
      <VerifiedBnfts<T>>::insert(uri.clone(), &bnft);

      //Remove from Owned BNFTs
      let bnftIndex = Self::get_bnft_index_for(uriClassIndexTuple.clone());
      <BnftOwner<T>>::remove(uriClassIndexTuple.clone());
      <OwnedBnftsArray<T>>::remove((agent.clone(), bnftIndex));
      
      //Decrement Owned Bnft count
      let mut ownedBnftsCount = Self::bnft_count_for(&agent);
      <OwnedBnftsCount<T>>::insert(&agent, ownedBnftsCount.saturating_sub(1));

      //Release stake
      let bnftClass = Self::get_bnft_class(class_index);
      <token::Module<T>>::unlock(agent.clone(), bnftClass.stake, uriClassIndexTuple.clone()); 

      //Transfer bounty to agent and verifier
      let funder = Self::funder_of(class_index);
      <token::Module<T>>::unlock(agent.clone(), bnftClass.transfer_bounty, (funder.clone(), class_index));
      <token::Module<T>>::unlock(sender.clone(), bnftClass.verification_bounty, (funder.clone(), class_index));

      //Emit events
      Self::deposit_event(RawEvent::BnftVerified(sender, agent, bnft));

      Ok(())
    }
  }
}

