use parity_codec::{Decode, Encode};
use support::{StorageValue, StorageMap, ensure, dispatch::Result, decl_module, decl_storage, decl_event};
use support::traits::{Currency, WithdrawReason, ExistenceRequirement};
use runtime_primitives::traits::{Zero, Hash, Saturating, As, CheckedAdd, CheckedMul, CheckedDiv};
use {system::ensure_signed, timestamp};
use rstd::prelude::*;
use crate::token;

pub trait Trait: balances::Trait + timestamp::Trait + token::Trait {
    //type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Encode, Decode, Default, Clone, PartialEq, Eq)]
pub struct Key {
    purpose: u8,
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
    trait Store for Module<T: Trait> as Identity {
        //Keys Store
        Keys get(getKeyFor): map (T::AccountId, Vec<u8>) => Key;
        KeysByPurpose get(getKeyForPurpose): map (T::AccountId, u8) => Vec<Vec<u8>>;
        
        //Claim Store
    }
}

// decl_event! {
//   pub enum Event<T> {

//   }
// }

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        //fn deposit_event<T>() = default;
        
        //ERC734(Setters + Side Effects)//
        fn addKey(origin, toAccount: T::AccountId, _key: Vec<u8>, _purpose: u8, _keyType: u32) -> Result {
            let sender = ensure_signed(origin)?;

            //Check if sender has management purpose for toAccount or is same as toAccount
            if(&sender != &toAccount) {
                ensure!(Self::keyHasPurpose(&sender, 1), "You are not authorized to do this.");
            }

            //Check if key already exists
            ensure!(!<Keys<T>>::exists((&toAccount, &_key)), "Key already exists - change purpose?");

            //Add key
            let key = Key {
                key: _key,
                purpose: _purpose,
                keyType: _keyType,
            };

            <Keys<T>>::insert((toAccount, _key), key); 

            //Emit event

            Ok(())
        }

        fn removeKey(origin, key: Vec<u8>, purpose: u8) -> Result {
            let sender = ensure_signed(origin)?;

            Ok(())
        }

        fn changeKeysRequired(origin, purpose: u8, number: u8) -> Result {
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
                    issuer: T::AccountId, 
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
                       issuer: T::AccountId, 
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
}

impl<T: Trait> Module<T> {
    //ERC734 Getters//
    pub fn getKey(_key: Vec<u8>) -> Key {

    }

    pub fn keyHasPurpose(forAccount: T::AccountId, _key: Vec<u8>, _purpose: u8) -> bool {
        //Check if key exists
        ensure!(<Keys<T>>::exists(forAccount, &_key), "Key not found");

        //Return if key has intended purpose or greater
        return (<Keys<T>>::getKeyFor(forAccount, &_key) >= _purpose)
    }

    pub fn getKeysByPurpose(_purpose: u8) -> Vec<Vec<u8>> {
        
    }

    pub fn getKeysRequired(_purpose: u8) -> u8 {

    }

    //ERC735 Getters//
    pub fn getClaim(claimId: Vec<u8>) -> Claim<T::AccountId> {

    }

    pub fn getClaimIdsByTopic(_topic: u32) -> Vec<u32> {

    }
}
