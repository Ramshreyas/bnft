use parity_codec::{Decode, Encode};
use support::{StorageValue, StorageMap, ensure, dispatch::Result, decl_module, decl_storage, decl_event};
use support::traits::{Currency, WithdrawReason, ExistenceRequirement};
use runtime_primitives::traits::{Zero, Hash, Saturating, As, CheckedAdd, CheckedMul, CheckedDiv};
use {system::ensure_signed, timestamp};
use rstd::prelude::*;
use runtime_io::keccak_256;
use crate::token;

pub trait Trait: balances::Trait + timestamp::Trait + token::Trait {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Encode, Decode, Default, Clone, PartialEq, Eq)]
pub struct Key<AccountId> {
    purpose: u16,
    keyType: u16,
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
        //Keys Store
        Keys get(getKeyFor): map (T::AccountId, T::AccountId) => Key<T::AccountId>;
        KeysByPurpose get(getKeysByPurpose): map (T::AccountId, u16) => Vec<T::AccountId>;
        
        //Claim Store
    }
}

decl_event! {
    pub enum Event<T> where AccountId = <T as system::Trait>::AccountId, {
        KeyAdded(AccountId, Key<AccountId>),
        KeyRemoved(AccountId, AccountId),
        KeysRequiredChanged(u16, u16),
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn deposit_event<T>() = default;
        
        //ERC734(Setters + Side Effects)//
        fn addKey(origin, toAccount: T::AccountId, _key: T::AccountId, _purpose: u16, _keyType: u16) -> Result {
            let sender = ensure_signed(origin)?;

            //Check if sender has management clearance
            if(sender.clone() != toAccount.clone()) {
                ensure!(Self::keyHasPurpose(toAccount.clone(), sender, 1), "You are not authorized to do this.");
            }

            //Check if key already exists
            let keyTuple = (toAccount.clone(), _key.clone());
            ensure!(!<Keys<T>>::exists(keyTuple), "Key already exists - change purpose?");

            //Add key to Keys
            let key = Key {
                key: _key.clone(),
                purpose: _purpose.clone(),
                keyType: _keyType,
            };
            <Keys<T>>::insert((toAccount.clone(), _key.clone()), key.clone()); 
            
            //Add Key to  KeysByPurpose
            let purposeTuple = (toAccount.clone(), _purpose.clone());
            let mut keyVector = Self::getKeysByPurpose(purposeTuple.clone());
            keyVector.push(_key.clone());
            <KeysByPurpose<T>>::insert(purposeTuple, keyVector);

            //Emit event
            Self::deposit_event(RawEvent::KeyAdded(toAccount, key));

            Ok(())
        }

        fn removeKey(origin, _key: T::AccountId, _purpose: u16) -> Result {
            let sender = ensure_signed(origin)?;

            //Ensure key exists
            let keyTuple = (sender.clone(), _key.clone());
            ensure!(<Keys<T>>::exists(&keyTuple), "Key not found");

            //Remove key from Keys
            <Keys<T>>::remove(keyTuple);
            
            //Remove Key from KeysByPurpose
            let purposeTuple = (sender.clone(), _purpose.clone());
            let keyToRemove = _key.clone();
            let mut keysByPurposeVector = Self::getKeysByPurpose(&purposeTuple);
            let index = keysByPurposeVector.iter().position(|item| *item == keyToRemove).unwrap();
            keysByPurposeVector.remove(index);
            <KeysByPurpose<T>>::insert(purposeTuple, keysByPurposeVector);

            //Emit event
            Self::deposit_event(RawEvent::KeyRemoved(sender, _key));
            
            Ok(())
        }

        fn changeKeysRequired(origin, purpose: u16, number: u16) -> Result {
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

        fn removeClaim(origin, claimId: Vec<u8>) -> Result {
            let sender = ensure_signed(origin)?;

            Ok(())
        }
    }
}

impl<T: Trait> Module<T> {
    //ERC734 Getters//
    // pub fn getKey(_key: Vec<U8>) -> Key {
        
    // }

    pub fn keyHasPurpose(forAccount: T::AccountId, _key: T::AccountId, _purpose: u16) -> bool {
        //Check if key exists
        let mut hasPurpose = false;
        let keyTuple = (forAccount.clone(), _key.clone());
        if (<Keys<T>>::exists(keyTuple.clone())) {
            //Return if key has intended purpose or greater
            hasPurpose = Self::getKeyFor(keyTuple).purpose <= _purpose;
        };

        hasPurpose
    }

    // pub fn getKeysByPurpose(_purpose: u8) -> Vec<Vec<U8>> {
        
    // }

    // pub fn getKeysRequired(_purpose: u8) -> u8 {

    // }

    // //ERC735 Getters//
    // pub fn getClaim(claimId: Vec<U8>) -> Claim<T::AccountId> {

    // }

    // pub fn getClaimIdsByTopic(_topic: u32) -> Vec<u32> {

    // }
}
