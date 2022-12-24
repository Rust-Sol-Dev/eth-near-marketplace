


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



use crate::*; // loading all defined crates, structs and functions from the root crate which is lib.rs in our case
use crate::nft_core::NoneFungibleTokenCore; //-- based on the orphan rule we MUST import this trait to use the nft_token() method on the instance of the `NFTContract` struct - we have to sepecify which trait has this method cause it might be another trait with this method exists inside this crate






#[near_bindgen] //-- implementing the #[near_bindgen] proc macro attribute on `NFTContract` struct to compile all its methods to wasm so we can call them in near cli
impl NFTContract{ //-- following methods are view methods (none payable methods) and will be compiled to wasm using #[near_bindgen] proc macro attribute
    
    
        
    // NOTE - we can bound the return type of a method using impl Trait<T> like the return type of keys() method of the UnorderedMap data structure which is bounded to impl Iterator<Item = K> + 'a
    // NOTE - following are some nft queries that must be perform on our current contract cause we want to like list all tokens for an owner and we have them in self.tokens_per_owner field
    // NOTE -  we've borrowed the self in the following methods cause we don't want to lose the lifetime of the created instance from the contract struct after calling each method 
    //         by borrowing the self we'll prevent the instance from moving and have it inside the upcoming scope even after calling these methods

    


    pub fn nft_total_supply(&self) -> U128{ //-- query for the total supply or total amounts of nfts on this contract
        U128(self.token_metadata_by_id.len() as u128) //-- converting the total length of all nfts inside this contract to u128 cause it might be lots of minted nfts in this contract
    }

    pub fn nft_tokens(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<JsonToken>{ //-- query for the tokens info on this contract regardless of their owners using pagination - we put from_index and limit params inside Option in order to have a default value for them on None match 
        let start = u128::from(from_index.unwrap_or(U128(0)));
        self.token_metadata_by_id.keys() //-- keys() method will return an iterator which is bounded to Iterator trait and contains all the keys inside an UnorderedMap or self.token_metadata_by_id field; the type of all elements inside the iterator are of type K or the key of self.token_metadata_by_id field which is of type TokenId which is String 
            .skip(start as usize) //-- skipping `start` elements until `start` elements are skipped; usize can be either 32 bits or 64 bits long - it'll return an iterator so we can map over it
            .take(limit.unwrap_or(50) as usize) //-- yielding `limit` elements until `limit` elements are yeilded; usize can be either 32 bits or 64 bits long - it'll return an iterator so we can map over it
            .map(|token_id| self.nft_token(token_id.clone()).unwrap()) //-- returning the token info json for this token_id using self.nft_token() method - we have to clone the token_id in each iteration when passing it to the self.nft_token() method cause we don't want to lose its ownership after passing
            .collect() //-- collecting all the token infos regardless of their owners
    }

    pub fn nft_supply_for_owner(&self, account_id: AccountId) -> U128{ //-- query for total supply or total amounts of nfts for a given owner - the total nfts owned by the goteam.testnet account is 0 on this contract
        let tokens_for_owner_set = self.tokens_per_owner.get(&account_id); //-- getting the set of all token_id(s) for this account_id by call the get() method on self.tokens_per_owner which will return Option<unordered<String>> contains the token_id(s) built from a persistent storage (created from a struct contains the account_id as its field) as the prefix key; means that token_id inside the unordered set belongs to a unique storage inside the memory which is owned by the hash of the account_id
        if let Some(owner_tokens_set) = tokens_for_owner_set{ //-- getting the UnorderedSet of all token_id(s) out of the Option
            U128(owner_tokens_set.len() as u128) //-- converting the total amounts of all nfts belongs to this account_id to u128
        } else{
            U128(0) //-- this account_id has no nft on this contract 
        }
    }
    
    pub fn nft_tokens_for_owner(&self, account_id: AccountId, from_index: Option<U128>, limit: Option<u64>) -> Vec<JsonToken>{ //-- query for all the token for an owner using pagination - we put from_index and limit params inside Option in order to have a default value for them on None match 
        let tokens_for_owner_set = self.tokens_per_owner.get(&account_id); //-- getting the set of all token_id(s) for this account_id by call the get() method on self.tokens_per_owner which will return Option<unordered<String>> contains the token_id(s) built from a persistent storage (created from a struct contains the account_id as its field) as the prefix key; means that token_id inside the unordered set belongs to a unique storage inside the memory which is owned by the hash of the account_id
        let tokens = if let Some(owner_tokens_set) = tokens_for_owner_set{ //-- can't use match cause the return type must be equal in each match arm and we have either an empty vector or an UnorderedSet of Strings - getting the UnorderedSet of all token_id(s) out of the Option
            owner_tokens_set
        } else{
            return vec![]; //-- means we didn't find any token set for the current account_id and we must return an empty vector
        };
        
        let start = u128::from(from_index.unwrap_or(U128(0))); //-- start pagination from `from_index` var or start from 0 of type u128
        tokens.iter() //-- iterating through the set of all tokens which is belongs to a unique prefix key inside the memory which is the hash of the account_id  
              .skip(start as usize) //-- skipping `start` elements until `start` elements are skipped; usize can be either 32 bits or 64 bits long - it'll return an iterator so we can map over it
              .take(limit.unwrap_or(50) as usize) //-- yielding `limit` elements until `limit` elements are yeilded; usize can be either 32 bits or 64 bits long - it'll return an iterator so we can map over it
              .map(|token_id| self.nft_token(token_id.clone()).unwrap()) //-- returning the token info json for this token_id using self.nft_token() method - we have to clone the token_id in each iteration when passing it to the self.nft_token() method cause we don't want to lose its ownership after passing
              .collect() //-- collecting all the token infos related to the current account_id
    }

    pub fn nft_events(&self, token_id: TokenId, from_index: Option<U128>, limit: Option<u64>) -> Vec<&EventLog>{ //-- the lifetime of the &Eventlog is depends on the lifetime of the &self to avoid dangling pointer since we're returning a reference from this method
        let token_events = self.token_events.get(&token_id);
        if let Some(nft_events) = token_events{
            let start = u128::from(from_index.unwrap_or(U128(0))); //-- start pagination from `from_index` var or start from 0 of type u128
            nft_events.iter()
                .skip(start as usize)
                .take(limit.unwrap_or(50) as usize)
                .collect()
        } else{
            vec![]
        }
    }

    pub fn nft_owners(&self, token_id: TokenId, from_index: Option<U128>, limit: Option<u64>) -> Vec<NFTOwnerInfo>{
        match self.owners_per_token.get(&token_id){
            Some(owners) => { //-- UnorderedSet is iterable so we can iterate over it
                let start = u128::from(from_index.unwrap_or(U128(0))); //-- start pagination from `from_index` var or start from 0 of type u128
                owners.iter()
                    .skip(start as usize)
                    .take(limit.unwrap_or(50) as usize)
                    .map(|owner_info| owner_info)
                    .collect()

            },
            None => vec![]
        }
    }

}