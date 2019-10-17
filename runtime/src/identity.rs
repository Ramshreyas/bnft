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
pub struct Key {
    purposes: Vec<u8>,
    keyType: u8,
    key: Vec<u8>,
}

#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Encode, Decode, Default, Clone, PartialEq, Eq)]
pub struct Claim<AccountId> {
    topic: u32,
    scheme: u32,
    issuer: AccountId,
    signature: Vec<u8>,
    data: Vec<u8>,
    uri: Vec<u8>,
}

decl_storage! {
    trait Store for Module<T: Trait> as Identity where
    AccountId = <T as system::Trait>::AccountId,
    {
        //Key STore
        Keys get(getKeys): map (AccountId, Vec<u8>) => Vec<Key>;
        KeysByPurpose get(getKeyForPurpose): map (AccountId, u8) => Vec<Vec<u8>>;
        
        //Claim Store
    }
}

decl_event! {
  pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {

  }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
      fn deposit_event<T>() = default;
      
      //ERC734(Setters + Side Effects)//
      fn addKey(origin, toAccount: AccountId, _key: Vec<u8>, _purpose: Vec<u8>, _keyType: u32) -> Result {
          let sender = ensure_signed(origin)?;

          //Check if sender has management purpose for toAccount or is same as toAccount
          if(&sender != &toAccount) {
              ensure!(keyHasPurpose(&sender, 1), "You are not authorized to do this.");
          }

          //Check if key already exists
          ensure!(!<Keys<T>>::exists((&toAccount, &_key)), "Key already exists - change purpose?");

          //Add key
          let key = Key {
              key: _key,
              purpose: _purpose,
              keyType: keyType,
          }

          <Keys<T>>::insert((toAccount, key), key) 

          //Emit event

          Ok(())
      }

       fn removeKey(origin, key: Vec<u8>, purpose: Vec<u8>) -> Result {
          let sender = ensure_signed(origin)?;

          Ok(())
      }

      fn changeKeysRequired(origin, purpose: Vec<u8>, number: u8) -> Result {
          let sender = ensure_signed(origin)?;

          Ok(())
      }

      fn execute(origin, value: u32, data: Vec<u8>) -> Result {
          let sender = ensure_signed(origin)?;

          Ok(())
      }

      fn approve(origin, id: u32, approval: bool) -> Result {
          let sender = ensure_signed(origin)?;

          Ok(())
      }

      //ERC735(Setters + Side Effects)//
      fn addClaim(origin, 
                  topic: u32, 
                  scheme: u32, 
                  issuer: AccountId, 
                  signature: Vec<u8>, 
                  data: Vec<u8>,
                  uri: Vec<u8>) -> Result 
      {
          let sender = ensure_signed(origin)?;

          Ok(())
      }

      fn changeClaim(origin, 
                     claimId: Vec<u8>,
                     topic: u32, 
                     scheme: u32, 
                     issuer: AccountId, 
                     signature: Vec<u8>, 
                     data: Vec<u8>,
                     uri: Vec<u8>) -> Result 
      {
          let sender = ensure_signed(origin)?;

          Ok(())
      }

      fn removeClaim(origin, claimId: Vec<u8>) {
          let sender = ensure_signed(origin)?;

          Ok(())
      }
}

impl<T: Trait> Module<T> {
    //ERC734 Getters//
    pub fn getKey(origin, _key: Vec<u8>) -> Key {

    }

    pub fn keyHasPurpose(origin, forAccount: AccountId, _key: Vec<u8>, _purpose: u8) -> bool {
        //Check if key exists
        ensure!(<Keys<T>>::exists(forAccount, &key), "Key not found");

        //Check if key has purpose
        return (
    }

    pub fn getKeysByPurpose(origin, _purpose: u8) -> Vec<Vec<u8>> {
        
    }

    pub fn getKeysRequired(origin, _purpose: u8) -> u8 {

    }

    //ERC735 Getters//
    pub fn getClaim(origin, claimId: Vec<u8>) -> Claim<AccountId> {

    }

    pub fn getClaimIdsByTopic(origin, _topic: u32) -> Vec<u32> {

    }
}
