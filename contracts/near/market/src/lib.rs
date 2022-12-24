





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
            https://stackoverflow.com/questions/72138820/near-marketplace-how-should-i-charge-the-transaction-fee-on-each-sales



*/








use std::{fmt, collections::HashMap};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize}; //-- self referes to the borsh struct itself cause there is a struct called borsh inside the borsh.rs file
use near_sdk::collections::{LazyOption, LookupMap, UnorderedMap, UnorderedSet}; //-- LookupMap and UnorderedMap are non-iterable implementations of a map that stores their contents directly on the trie - LazyOption stores a value in the storage lazily! 
use near_sdk::json_types::{Base64VecU8, U128, U64}; //-- Base64VecU8 is used to serialize/deserialize Vec<u8> to base64 string
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::serde_json::json;
use near_sdk::{ 
                serde_json,
                promise_result_as_success, //-- returns the result of the promise if successful, otherwise returns None
                env::STORAGE_PRICE_PER_BYTE, //-- loading the price of each byte in yocto Ⓝ (1e24)
                Gas, ext_contract, PromiseResult, env, near_bindgen, assert_one_yocto, //-- we're using the assert_one_yocto() function from the near_sdk cause it's using the env::panic_str() one the background 
                AccountId, Balance, CryptoHash, Promise, //-- Promise struct is needed to handle async cross contract calls or message passing between contract actors
                PanicOnDefault, PromiseOrValue, BorshStorageKey //-- PanicOnDefault macro must be used in case that the contract is required to be initialized with init methods which will be paniced on implemnted Default trait for the contract
            }; 






use crate::utils::*;
use crate::constants::*;
use crate::nft_callbacks::*;
use crate::sale::*;
use crate::internal::*;
use crate::storage::*;
use crate::offer::*;
use crate::auction_bids::*;






pub mod storage;
pub mod constants;
pub mod utils; //-- or crate::utils
pub mod internal;
pub mod sale;
pub mod nft_callbacks;
pub mod offer;
pub mod auction_bids;











// NOTE - HashMap keeps all data in memory, to access it, the contract needs to deserialize the whole map and it deserializes (and serializes) the entire collection in one storage operation; accessing the entire collection is cheaper in gas than accessing all elements through N storage operations
// NOTE - try to validate the input, context, state and access using require! before taking any actions; the earlier you panic, the more gas you will save for the caller
// NOTE - borsh is used for internal STATE serialization and serde for external JSON serialization
// NOTE - every cross contract calls for communicating between contract actor accounts in cross sharding pattern takes up cpu usage and network laoding costs which forces us to attach gas units in the contract method call in which the cross contract call method is calling to pass it through the calling of the cross contract call method
// NOTE - The NEAR whitepaper mentions that 30% of all gas fees go to smart contract accounts on which the fees are expensed
// NOTE - whenever a function is called an ActionReceipt object will be created by NEAR runtime from the transaction in which the state will be loaded and deserialized, so it's important to keep this amount of data loaded as minimal as possible
// NOTE - we can't impl Default trait for the contract if the PanicOnDefault trait is implemented for that contract
// NOTE - near hashmap and set based data structures or collections are LookupMap, LookupSet, UnorderedMap, UnorderedSet and TreeSet; each of them will be cached on chain to minimize the amount of gas and the SDK collections should be used in most cases to reduce the gas fee since these collections deserialize the exact data that we need it instead of deserializing all entries each time the state and the app runtime is loaded like HashMap
// NOTE - current_account_id()     -> the id of the account that owns the current contract actor account
// NOTE - predecessor_account_id() -> the id of the account that was the previous contract actor account in the chain of cross-contract calls and if this is the first contract, it is equal to signer_account_id - the last (current) caller of a contract actor method which created and signed the transaction by calling that method
// NOTE - signer_account_id()      -> the id of the account that either signed the original transaction or issued the initial cross-contract call that led to this execution 
// NOTE - in private methods current_account_id(), predecessor_account_id() and signer_account_id() are the same an is the contract actor account owner itself
// NOTE - since mutating the contract state on the chain will cost money thus in order to list an NFT on the market we have to create a sell object which is an object contains the NFT info for listing it on the market, since by listing the NFT we're mutating the state of the `MarketContract` on chain thus we must force the seller to deposit the storage cost for listing his/her NFT on the market by calling the storage_deposit() method 
// NOTE - in this contract owner_id is the current owner of the NFT which might be the minter on first sell and nft_contract_id is the account_id that the NFT contract is deployed on which is the one who owns the NFT contract and is not the buyer or second seller (since the first seller is the minter)
// NOTE - in this contract owner_id is the current owner of the NFT which might be the minter since the current NFT might be transferred to another owner or has beed sold out on the market thus the current NFT owner might not be the one who owns this contract therefore it can be the second sell
// NOTE - in private methods current_account_id(), predecessor_account_id() and signer_account_id() are the same an is the contract actor account owner itself






















/*
 
  -----------------------------
 |          Contract 
  -----------------------------
 | FIELDS:
 |      owner_id --------------> this is the owner of the market contract
 |      sales
 |      by_owner_id
 |      by_nft_contract_id
 |      storage_deposits
 |

*/

#[near_bindgen] //-- implementing the #[near_bindgen] proc macro attribute on `NFTContract` struct to compile all its methods to wasm so we can call them in near cli
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)] //-- borsh is need for serde and codec ops; deserialize or map utf8 bytes into this struct from where the contract has called and serialize it to utf8 bytes for compilling it to wasm to run on near blockchain 
pub struct MarketContract{ // NOTE - UnorderedMap is an iterable implementation of the map where as the LookupMap is a none iterable of the map
    pub owner_id: AccountId, //-- keeping the track of the owner of the contract which is the one who has called the initialization method and sign the transaction
    pub sales: UnorderedMap<ContractAndTokenId, Sale>, //-- sale_id: sale object - keeping the track of all the sales by mapping the ContractAndTokenId to a sale cause every sale has a unique identifier which is made up of the `nft_contract_actor_account_id + DELIMETER + token_id` 
    pub by_owner_id: LookupMap<AccountId, UnorderedSet<ContractAccountIdTokenId>>, //-- for auction and offer
    pub sales_by_owner_id: LookupMap<AccountId, UnorderedSet<ContractAndTokenId>>, //-- for sales
    pub by_nft_contract_id: LookupMap<AccountId, UnorderedSet<TokenId>>, //-- account_id: token_id - keeping the track of all the token_ids inside a set for a sale of a given account_id or the nft_contract_id which is the one who owns the NFT contract and is not the buyer or second seller (since the first seller is the minter)
    pub storage_deposits: LookupMap<AccountId, Balance>, //-- account_id: balance - mapping between all the storages paid in yocto Ⓝ (1e24) of type u128 by a specific account_id
    pub offers: UnorderedMap<ContractAccountIdTokenId, OfferData>,
    pub market: UnorderedMap<ContractAndTokenId, MarketData>,
}


#[near_bindgen]
impl MarketContract{ //-- we'll add bytes to the contract by creating entries in the data structures - we've defined the init methods of the `MarketContract` struct in here cause the lib.rs is our main crate


    #[init] //-- means the following would be a contract initialization method which must be called by the contract owner and verifies that the contract state doesn't exist on chain since can only be called once and will be paniced on second call
    pub fn new(owner_id: AccountId) -> Self{ //-- initialization function can only be called once when we first deploy the contract to runtime shards - this initializes the `MarketContract` on chain with the passed in owner_id
        let accounts_message = format!("SMARTIES : current account id is @{} | predecessor or the current caller account id is @{} | signer account id is @{}", env::current_account_id(), env::predecessor_account_id(), env::signer_account_id()); //-- format!() returns a String which takes 24 bytes storage, usize * 3 (pointer, len, capacity) bytes (usize is 64 bits or 24 bytes on 64 bits arch)
        // let accounts_message_bytes = accounts_message.as_bytes(); //-- as_bytes() returns &[u8] 
        env::log_str(&accounts_message); //-- passing the message in form of a borrowed type even though as_bytes() returns &[u8]
        Self{ //-- the return type is of type Self or the contract itself with initialized fields - this function will default all the collections to be empty
            owner_id,
            sales: UnorderedMap::new(Storagekey::Sales.try_to_vec().unwrap()),  //-- UnorderedMap takes a unique vector of u8 bytes (to have unique encoding we've used an enum variant called Sales defined in utils::Storagekey) in it constructor argument as the prefix that must be append before the UnorderedMap sales keys to avoid data collision with other keys of other collections of the `MarketContract` fields since they might be same keys inside two different collection - the prefix can be also the utf8 encoded of a unique string like b"sales" which is the name of the collection field
            by_owner_id: LookupMap::new(Storagekey::ByOwnerId.try_to_vec().unwrap()),  //-- LookupMap takes a unique vector of u8 bytes (to have unique encoding we've used an enum variant called ByOwnerId defined in utils::Storagekey) in it constructor argument as the prefix that must be append before the LookupMap by_owner_id keys to avoid data collision with other keys of other collections of the `MarketContract` fields since they might be same keys inside two different collection - the prefix can be also the utf8 encoded of a unique string like b"by_owner_id" which is the name of the collection field
            sales_by_owner_id: LookupMap::new(Storagekey::SalesByOwnerId.try_to_vec().unwrap()),  //-- LookupMap takes a unique vector of u8 bytes (to have unique encoding we've used an enum variant called ByOwnerId defined in utils::Storagekey) in it constructor argument as the prefix that must be append before the LookupMap by_owner_id keys to avoid data collision with other keys of other collections of the `MarketContract` fields since they might be same keys inside two different collection - the prefix can be also the utf8 encoded of a unique string like b"by_owner_id" which is the name of the collection field
            by_nft_contract_id: LookupMap::new(Storagekey::ByNFTContractId.try_to_vec().unwrap()),  //-- UnorderedMap takes a unique vector of u8 bytes (to have unique encoding we've used an enum variant called ByNFTContractId defined in utils::Storagekey) in it constructor argument as the prefix that must be append before the LookupMap by_nft_contract_id keys to avoid data collision with other keys of other collections of the `MarketContract` fields since they might be same keys inside two different collection - the prefix can be also the utf8 encoded of a unique string like b"by_nft_contract_id" which is the name of the collection field
            storage_deposits: LookupMap::new(Storagekey::StorageDeposits.try_to_vec().unwrap()),  //-- UnorderedMap takes a unique vector of u8 bytes (to have unique encoding we've used an enum variant called StorageDeposits defined in utils::Storagekey) in it constructor argument as the prefix that must be append before the LookupMap storage_deposits keys to avoid data collision with other keys of other collections of the `MarketContract` fields since they might be same keys inside two different collection - the prefix can be also the utf8 encoded of a unique string like b"storage_deposits" which is the name of the collection field        
            offers: UnorderedMap::new(Storagekey::Offers.try_to_vec().unwrap()),
            market: UnorderedMap::new(Storagekey::MrketData.try_to_vec().unwrap()),
        }
    }


}