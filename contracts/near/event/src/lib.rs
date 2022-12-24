




/*



Coded by



 █     █░ ██▓ ██▓    ▓█████▄  ▒█████   ███▄    █  ██▓ ▒█████   ███▄    █ 
▓█░ █ ░█░▓██▒▓██▒    ▒██▀ ██▌▒██▒  ██▒ ██ ▀█   █ ▓██▒▒██▒  ██▒ ██ ▀█   █ 
▒█░ █ ░█ ▒██▒▒██░    ░██   █▌▒██░  ██▒▓██  ▀█ ██▒▒██▒▒██░  ██▒▓██  ▀█ ██▒
░█░ █ ░█ ░██░▒██░    ░▓█▄   ▌▒██   ██░▓██▒  ▐▌██▒░██░▒██   ██░▓██▒  ▐▌██▒
░░██▒██▓ ░██░░██████▒░▒████▓ ░ ████▓▒░▒██░   ▓██░░██░░ ████▓▒░▒██░   ▓██░
░ ▓░▒ ▒  ░▓  ░ ▒░▓  ░ ▒▒▓  ▒ ░ ▒░▒░▒░ ░ ▒░   ▒ ▒ ░▓  ░ ▒░▒░▒░ ░ ▒░   ▒ ▒ 
  ▒ ░ ░   ▒ ░░ ░ ▒  ░ ░ ▒  ▒   ░ ▒ ▒░ ░ ░░   ░ ▒░ ▒ ░  ░ ▒ ▒░ ░ ░░   ░ ▒░
  ░   ░   ▒ ░  ░ ░    ░ ░  ░ ░ ░ ░ ▒     ░   ░ ░  ▒ ░░ ░ ░ ▒     ░   ░ ░ 
    ░     ░      ░  ░   ░        ░ ░           ░  ░      ░ ░           ░ 
                      ░                                                  



*/










use std::{fmt, collections::HashMap};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize}; //-- self referes to the borsh struct itself cause there is a struct called borsh inside the borsh.rs file
use near_sdk::collections::{LazyOption, LookupMap, UnorderedMap, UnorderedSet}; //-- LookupMap and UnorderedMap are non-iterable implementations of a map that stores their contents directly on the trie - LazyOption stores a value in the storage lazily! 
use near_sdk::json_types::{Base64VecU8, U128}; //-- Base64VecU8 is used to serialize/deserialize Vec<u8> to base64 string
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
                assert_one_yocto,
                require,
                serde_json::{self, json}, //-- self referes to the serde_json crate itself
                Gas, ext_contract, PromiseResult, 
                env, near_bindgen, AccountId, Balance, 
                CryptoHash, PanicOnDefault, //-- PanicOnDefault macro must be used in case that the contract is required to be initialized with init methods which will be paniced on implemnted Default trait for the contract 
                Promise, PromiseOrValue //-- Promise struct is needed to handle async cross contract calls or message passing between contract actors
            };




use crate::constants::*;
use crate::utils::*;
use crate::expire::*;
use crate::lock::*;
use crate::vote::*;
use crate::metadata::*;
use crate::enumeration::*;




pub mod constants;
pub mod utils;
pub mod create;
pub mod expire;
pub mod lock;
pub mod vote;
pub mod metadata;
pub mod enumeration;















#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct EventContract{
    pub owner_id: AccountId, //-- the owner of this contract
    pub events_by_id_collection_name: UnorderedMap<EventId, CollectionName>,
    pub events_by_id: UnorderedMap<EventId, Event>, //-- keeping track of all events related to a specific collection - all events' collection
}


#[near_bindgen] //-- compile all the following methods to wasm so we can call them in near cli
impl EventContract{

    #[init]
    pub fn new(owner_id: AccountId) -> Self{
        Self{
            owner_id,
            events_by_id: UnorderedMap::new(StorageKey::ProposalById.try_to_vec().unwrap()),
            events_by_id_collection_name: UnorderedMap::new(StorageKey::ProposalByIdCollectionName.try_to_vec().unwrap()),
        }
    }    

}