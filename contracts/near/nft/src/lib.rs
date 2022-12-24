





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

            https://www.near-sdk.io/contract-structure/collections ➔ Near Sdk Collection Performence




            all NoneFungibleTokenCore trait methods {
                nft_approve
                nft_is_approved
                nft_revoke
                nft_revoke_all
                nft_transfer
                nft_transfer_call
                nft_token
                nft_payout
                nft_transfer_payout
            }
            
            must be implemented for:

            impl NoneFungibleTokenCore for NFTContract{

            }



*/






use std::{fmt, collections::HashMap};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize}; //-- self referes to the borsh struct itself cause there is a struct called borsh inside the borsh.rs file
use near_sdk::collections::{LazyOption, LookupMap, UnorderedMap, UnorderedSet}; //-- LookupMap and UnorderedMap are non-iterable implementations of a map that stores their contents directly on the trie - LazyOption stores a value in the storage lazily! 
use near_sdk::json_types::{Base64VecU8, U128, U64}; //-- Base64VecU8 is used to serialize/deserialize Vec<u8> to base64 string
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
                assert_one_yocto,
                require,
                serde_json::{self, json}, //-- self referes to the serde_json crate itself
                Gas, ext_contract, PromiseResult, 
                env, env::STORAGE_PRICE_PER_BYTE, near_bindgen, AccountId, Balance, 
                CryptoHash, PanicOnDefault, //-- PanicOnDefault macro must be used in case that the contract is required to be initialized with init methods which will be paniced on implemnted Default trait for the contract 
                Promise, PromiseOrValue //-- Promise struct is needed to handle async cross contract calls or message passing between contract actors
            };
use crate::utils::*;
use crate::approval::*;
use crate::enumeration::*;
use crate::mint::*;
use crate::royalty::*;
use crate::metadata::*;
use crate::nft_core::*;
use crate::internal::*;
use crate::constants::*;
use crate::events::*;




pub mod constants;
pub mod utils; //-- or crate::utils
pub mod approval;
pub mod enumeration;
pub mod events;
pub mod metadata;
pub mod mint;
pub mod nft_core;
pub mod royalty;
pub mod internal;










// NOTE - we have to make money from callers since updating structs cost money and we have to force the caller to deposit some amount to cover the updating cost in our contract thus we must not to spend from our moeny to do this and we have to make the method payable
// NOTE - HashMap keeps all data in memory, to access it, the contract needs to deserialize the whole map and it deserializes (and serializes) the entire collection in one storage operation; accessing the entire collection is cheaper in gas than accessing all elements through N storage operations
// NOTE - try to validate the input, context, state and access using require! before taking any actions; the earlier you panic, the more gas you will save for the caller
// NOTE - borsh is used for internal STATE serialization and serde for external JSON serialization
// NOTE - our `NFTContract` has all the minted nfts info inside of it with a mapping between their metadata and the owner
// NOTE - `NFTContract` struct contains some data structures to store on chain infos about tokens and their owners at runtime
// NOTE - whenever a function is called an ActionReceipt object will be created by NEAR runtime from the transaction in which the state will be loaded and deserialized, so it's important to keep this amount of data loaded as minimal as possible
// NOTE - all payable methods needs to deposit some yocto Ⓝ (1e24) since they might be mutations on contract state and ensuring that the user is not DDOSing on the method thus the cost must be paid by the caller not by the contract owner and will refunded any excess that is unused
// NOTE - we can't impl Default trait for the contract if the PanicOnDefault trait is implemented for that contract
// NOTE - near hashmap and set based data structures or collections are LookupMap, LookupSet, UnorderedMap, UnorderedSet and TreeSet; each of them will be cached on chain to minimize the amount of gas and the SDK collections should be used in most cases to reduce the gas fee since these collections deserialize the exact data that we need it instead of deserializing all entries each time the state and the app runtime is loaded like HashMap
// NOTE - current_account_id()     -> the id of the account that owns the current contract actor account
// NOTE - predecessor_account_id() -> the id of the account that was the previous contract actor account in the chain of cross-contract calls and if this is the first contract, it is equal to signer_account_id - the last (current) caller of a contract actor method which created and signed the transaction by calling that method
// NOTE - signer_account_id()      -> the id of the account that either signed the original transaction or issued the initial cross-contract call that led to this execution 
// NOTE - in private methods current_account_id(), predecessor_account_id() and signer_account_id() are the same an is the contract actor account owner itself











/*
 
  -----------------------------
 |          Contract 
  -----------------------------
 | FIELDS:
 |      owner_id --------------> this is the account_id which this contract is deployed on which contains all the market NFTs
 |      metadata
 |      tokens_per_owner
 |      tokens_by_id
 |      token_metadata_by_id
 |

*/

#[near_bindgen] //-- implementing the #[near_bindgen] proc macro attribute on `NFTContract` struct to compile all its methods to wasm so we can call them in near cli
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)] //-- borsh is need for serde and codec ops; deserialize or map utf8 bytes into this struct from where the contract has called and serialize it to utf8 bytes for compilling it to wasm to run on near blockchain   
pub struct NFTContract{ //-- can't implement Default trait for this contract cause Default is not implemented for LazyOption, LookupMap and UnorderedMap structs - our contract keeps track of some mapping between owner_id, token_id and the token_metadata inside some collections
    pub owner_id: AccountId, //-- contract owner who called the initialization method which can be anyone; this is the owner_id of the one which this contract must get deployed on to mint all his/her all NFTs on  
    pub metadata: LazyOption<NFTContractMetadata>, //-- keeps track of the metadata for the contract
    pub tokens_per_owner: LookupMap<AccountId, UnorderedSet<TokenId>>, //-- keeps track of all the token_ids for a given account using a none iterable map in a LookupMap collection - this field will be used to retrieve all nft_id(s) for a specific owner on this contract
    pub tokens_by_id: LookupMap<TokenId, Token>, //-- keepts track of the token struct (owner_id) for a given token_id using a none iterable map in a LookupMap collection - this field is a mapping between token_id(s) and the token object which contains the owner_id thus will be used to retrieve all owner_id(s) for a specific token_id on this contract
    pub token_metadata_by_id: UnorderedMap<TokenId, TokenMetadata>, //-- keeps track of the token metadata for a given token_id using a none iterable map in an UnorderedMap collection - this field will be used to retrieve a token metadata for a specific token_id on this contract
    pub token_events: HashMap<TokenId, Vec<EventLog>>,
    pub owners_per_token: LookupMap<TokenId, UnorderedSet<NFTOwnerInfo>>, //-- keeps track of all owners of a specific token
}


#[near_bindgen] //-- implementing the #[near_bindgen] proc macro attribute on `NFTContract` struct to compile all its methods to wasm so we can call them in near cli
impl NFTContract{ //-- we'll add bytes to the contract by creating entries in the data structures - we've defined the init methods of the `NFTContract` struct in here cause the lib.rs is our main crate

    #[init] //-- means the following would be a contract initialization method which must be called by the contract owner and verifies that the contract state doesn't exist on chain since can only be called once and will be paniced on second call
    pub fn new(owner_id: AccountId, metadata: NFTContractMetadata) -> Self{ //-- initialization function can only be called once when we first deploy the contract to runtime shards - this initializes the contract with metadata that was passed in and the owner_id
        let accounts_message = format!("SMARTIES : current account id is @{} | predecessor or the current caller account id is @{} | signer account id is @{}", env::current_account_id(), env::predecessor_account_id(), env::signer_account_id()); //-- format!() returns a String which takes 24 bytes storage, usize * 3 (pointer, len, capacity) bytes (usize is 64 bits or 24 bytes on 64 bits arch)
        // let accounts_message_bytes = accounts_message.as_bytes(); //-- as_bytes() returns &[u8] 
        env::log_str(&accounts_message); //-- passing the message in form of a borrowed type even though as_bytes() returns &[u8]
        Self{ //-- the return type is of type Self or the contract itself with initialized fields - this function will default all the collections to be empty
            owner_id,
            metadata: LazyOption::new(Storagekey::NFTContractMetadata.try_to_vec().unwrap(), Some(&metadata)), //-- LazyOption takes a vector of u8 bytes in it constructor argument as the prefix from the current storage taken by NFTContractMetadata collection or the 64 bits (8 bytes) address of the enum tag which is pointing to the current variant
            tokens_per_owner: LookupMap::new(utils::Storagekey::TokensPerOwner.try_to_vec().unwrap()), //-- LookupMap takes a vector of u8 bytes in it constructor argument as the prefix from the current storage taken by TokensPerOwner collection or the 64 bits (8 bytes) address of the enum tag which is pointing to the current variant
            tokens_by_id: LookupMap::new(Storagekey::TokensById.try_to_vec().unwrap()), //-- LookupMap takes a vector of u8 bytes in it constructor argument as the prefix from the current storage taken by TokensById collection or the 64 bits (8 bytes) address of the enum tag which is pointing to the current variant
            token_metadata_by_id: UnorderedMap::new(Storagekey::TokenMetadataById.try_to_vec().unwrap()), //-- UnorderedMap takes a vector of u8 bytes in it constructor argument as the prefix from the current storage taken by TokenMetadataById collection or the 64 bits (8 bytes) address of the enum tag which is pointing to the current variant
            token_events: HashMap::new(),
            owners_per_token: LookupMap::new(utils::Storagekey::OwnersPerToken.try_to_vec().unwrap()), //-- LookupMap takes a vector of u8 bytes in it constructor argument as the prefix from the current storage taken by OwnersPerToken collection or the 64 bits (8 bytes) address of the enum tag which is pointing to the current variant   
        }
    }

    #[init] //-- means the following would be a contract initialization method which must be called by the contract owner and verifies that the contract state doesn't exist on chain since can only be called once and will be paniced on second call
    pub fn new_default_meta(owner_id: AccountId) -> Self{ //-- initialization function can only be called once when we first deploy the contract to runtime shards - this initializes the contract with default metadata so the user don't have to manually type metadata
        Self::new( //-- calling new() method with some default metadata params and the owner_id passed in
            owner_id,
   NFTContractMetadata{
                spec: "nft-1.0.0".to_string(),
                name: "Smarties NFT Contract".to_string(),
                symbol: "FTC".to_string(),
                icon: None,
                base_uri: None,
                reference: None,
                reference_hash: None,
            },
        )
    }

}