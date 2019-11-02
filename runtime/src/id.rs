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
    topic: u16,
    scheme: u16,
    issuer: AccountId,
    signature: Vec<u8>,
    data: Vec<u8>,
    uri: Vec<u8>,
}

decl_storage! {
    trait Store for Module<T: Trait> as Id {
        //Keys Store
        Keys get(getKeyFor): map (T::AccountId, T::AccountId) => Key<T::AccountId>;
        KeysByPurpose get(keysByPurpose): map (T::AccountId, u16) => Vec<T::AccountId>;
        
        //Claim Store
        Claims get(getClaimById): map Vec<u8> => Claim<T::AccountId>;
        ClaimsByTopic get(getClaimsByTopic): map (T::AccountId, u16) => Vec<Vec<u8>>;
    }
}

decl_event! {
    pub enum Event<T> where AccountId = <T as system::Trait>::AccountId, {
        //ERC734 events
        KeyAdded(AccountId, Key<AccountId>),
        KeyRemoved(AccountId, AccountId),
        KeysRequiredChanged(u16, u16),

        //ERC735 events
        ClaimAdded(AccountId, Vec<u8>),
        ClaimRemoved(AccountId, Vec<u8>),
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
            let mut keyVector = Self::keysByPurpose(purposeTuple.clone());
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
            let mut keysByPurposeVector = Self::keysByPurpose(&purposeTuple);
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
                    toAccount: T::AccountId,
                    topic: u16, 
                    scheme: u16, 
                    issuer: T::AccountId, 
                    signature: Vec<u8>, 
                    data: Vec<u8>,
                    uri: Vec<u8>) -> Result 
        {
            let sender = ensure_signed(origin)?;
            
            //Ensure sender is same as issuer, or has rights to add claim
            if(sender.clone() != issuer.clone()) {
                ensure!(Self::keyHasPurpose(issuer.clone(), sender.clone(), 3), "You are not authorized!");
            }
            
            //Generate ClaimId
            let mut to_account_as_bytes = toAccount.encode();
            let mut issuer_as_bytes = issuer.encode();
            let mut topic_as_bytes = topic.encode();
            let claimId_bytes = [issuer_as_bytes, topic_as_bytes, to_account_as_bytes].concat();
            let claimId = keccak_256(&claimId_bytes).to_vec();

            //Check if claim already exists
            ensure!(!<Claims<T>>::exists(&claimId), "Claim already exists!");

            //Add claim to claims
            let claim = Claim {
                topic,
                scheme,
                issuer: issuer.clone(),
                signature,
                data,
                uri,
            };
            <Claims<T>>::insert(claimId.clone(), claim);

            //Add to claims by topic
            let claim_by_type_tuple = (toAccount.clone(), topic.clone());
            let mut claims_vector = Self::getClaimsByTopic(claim_by_type_tuple.clone());
            claims_vector.push(claimId.clone());
            <ClaimsByTopic<T>>::insert(claim_by_type_tuple, claims_vector);

            //Emit event
            Self::deposit_event(RawEvent::ClaimAdded(toAccount, claimId));

            Ok(())
        }

        fn changeClaim(origin, 
                       claimId: Vec<u8>,
                       topic: u16, 
                       scheme: u16, 
                       issuer: T::AccountId, 
                       signature: Vec<u8>, 
                       data: Vec<u8>,
                       uri: Vec<u8>) -> Result 
        {
            let sender = ensure_signed(origin)?;

            Ok(())
        }

        fn removeClaim(origin, forAccount: T::AccountId, claimId: Vec<u8>) -> Result {
            let sender = ensure_signed(origin)?;
            
            //Check if claim exists
            ensure!(<Claims<T>>::exists(&claimId), "Claim not found!");

            //Check if sender is issuer, has requisite authority or is owner
            let claim_to_remove = Self::getClaimById(&claimId);
            let issuer = claim_to_remove.issuer.clone();
            if(sender.clone() != issuer.clone() && sender.clone() != forAccount) {
                ensure!(Self::keyHasPurpose(issuer, sender, 3), "You are not authorized!");
            }

            //Remove from Claims
            <Claims<T>>::remove(&claimId);

            //Remove from ClaimsByTopic
            let claim_by_topic_tuple = (forAccount.clone(), claim_to_remove.topic.clone());
            let claimId_to_remove = claimId.clone();
            let mut claims_by_topic_vector = Self::getClaimsByTopic(&claim_by_topic_tuple);
            let index = claims_by_topic_vector.iter().position(|item| *item == claimId_to_remove).unwrap();
            claims_by_topic_vector.remove(index);
            <ClaimsByTopic<T>>::insert(claim_by_topic_tuple, claims_by_topic_vector);
            
            //Emit event
            Self::deposit_event(RawEvent::ClaimRemoved(forAccount, claimId));

            Ok(())
        }
    }
}

impl<T: Trait> Module<T> {
    //ERC734 Getters//
    pub fn getKey(forAccount: T::AccountId, _key: T::AccountId) -> Key<T::AccountId> {
        Self::getKey(forAccount, _key)
    }

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

    pub fn getKeysByPurpose(_forAccount: T::AccountId, _purpose: u16) -> Vec<T::AccountId> {
        Self::keysByPurpose((_forAccount, _purpose))
    }

    // pub fn getKeysRequired(_purpose: u8) -> u8 {

    // }

    // //ERC735 Getters//
    pub fn getClaim(claimId: Vec<u8>) -> Claim<T::AccountId> {
        Self::getClaim(claimId)
    }

    pub fn getClaimIdsByTopic(forAccount: T::AccountId, _topic: u16) -> Vec<Vec<u8>> {
        Self::getClaimsByTopic((forAccount, _topic))
    }

    pub fn claimExists(claimId: Vec<u8>) -> Result {
        if <Claims<T>>::exists(claimId) {
            Ok(())
        } else {
            Err("Claim not found!")
        }
    }
}
