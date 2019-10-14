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
pub struct Key<AccountId> {
    purposes: Vec<u8>,
    keyType: u8,
    key: AccountId,
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
  trait Store for Module<T: Trait> as Identity {

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
      fn addKey(origin, key: Vec<u8>, purpose: Vec<u8>, keyType: u32) -> Result {
          let sender = ensure_signed(origin)?;

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
    //ERC734(Getters)//
    pub fn getKey(origin, key: AccountId) -> Key<AccountId> {

    }

    pub fn keyHasPurpose(origin, key: AccountId, purpose: u8) -> bool {
        
    }

    pub fn getKeysByPurpose(origin, purpose: u8) -> Vec<AccountId> {
        
    }

    pub fn getKeysRequired(origin, purpose: u8) -> u8 {

    }

    //ERC735(Getters)//
    pub fn getClaim(origin, claimId: Vec<u8>) -> Claim<AccountId> {

    }

    pub fn getClaimIdsByTopic(origin, topic: u32) -> Vec<u32> {

    }
}
