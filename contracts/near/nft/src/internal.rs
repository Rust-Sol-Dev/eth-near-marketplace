



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





// NOTE - methods and function in here don't need to be compiled to wasm cause they're internal functions
// NOTE - can't use #[payable] and #[private] attributes on a none #[near_bindgen] proc macro attribute contract struct 
// NOTE - CryptoHash are objects with 256 bits of information which is 32 bytes of data or 64 chars in hex
//        pub struct Digest(pub [u8; 32]); //-- we've specified the size of the array thus we don't need to take a reference to the [u8]
//        pub struct CryptoHash(pub Digest);











impl NFTContract{ //-- we've defined the following methods of the `NFTContract` struct in this crate cause this crate is related to all internal calculation functions and methods - we don't need to add #[near_bindgen] proc macro attribute on this impl cause these are none exporting methods and won't compile to wasm to call them from cli 

    pub fn internal_add_token_to_owner(&mut self, account_id: &AccountId, token_id: &TokenId){ //-- we've defined the self to be mutable and borrowed cause we want to add the account_id and minted token info to tokens_per_owner field and have the isntance with a valid lifetime after calling this method on it - add the minted token to the set of token an owner has first
        
        let mut tokens_set = self.tokens_per_owner.get(account_id).unwrap_or_else(|| { //-- getting the set of token_id(s) for the given account out of the LookupMap or create a new set for the given account inside the closure
            UnorderedSet::new( //-- if the NFT contract account doesn't have any tokens, we create a new unordered set to save the minted token_id for the current account_id as his/her first NFT 
            Storagekey::TokenPerOwnerInner{ //-- choosin  g a new unique prefix or key from the enum for the storage of the current collection which is the TokenPerOwnerInner variant struct - UnorderedSet will create a new set based on the selected storage key which is an utf8 bytes encoded enum variant in our case and takes the size of the TokenPerOwnerInner struct in memory in order to avoid data collision with other hashmap based data structures' keys which share same storage for their keys
                        account_id_hash: hash_account_id(&account_id), //-- getting the hash of the current account_id
                } //-- our current storage key in memory (also current variant) is the TokenPerOwnerInner struct which contains the hash of the account_id we can be sure that there is one account_id which has this token_id in other words there are no two different account_id(s) that has the same token_id at the same time 
                .try_to_vec() //-- converting the selected storage key into a vector of u8 to build the UnorderedSet hash key based on this vector - we can call the try_to_vec() method cause BorshSerializer trait is implemented for Storagekey enum
                .unwrap(),
            ) //-- to create a new UnorderedSet we have to pass a prefix which is a unique key to avoid data collision and in our case this unique key is the hash of the account_id since there is one account_id which has this token_id in other words there are no two different account_id(s) that have the same token_id at the same time 
        }); //-- the type of the tokens_set must be UnorderedSet<String> cause TokenId is of type String
        tokens_set.insert(token_id); //-- inserting the token_id into the created set for the current account_id
        self.tokens_per_owner.insert(account_id, &tokens_set); //-- inserting the created set for the given account_id
    
    }



    pub fn internal_transfer(&mut self, sender_id: &AccountId, receiver_id: &AccountId, token_id: &TokenId, approval_id: Option<u64>, balance: Option<U128>, memo: Option<String>) -> Token{ //-- transferring a specific NFT to a specific receiver_id - always take a reference to String (token_id) to borrow it to prevent from moving and cloning
        
        let check_memo = memo.unwrap().clone();
        let owned_type: utils::OwnedType;
        let nft_price_in_near = if let Some(price) = balance{
            let ot = if check_memo == "bid".to_string(){
                utils::OwnedType::Bid    
            } else if check_memo == "accept offer".to_string(){
                utils::OwnedType::AcceptOffer
            } else{
                utils::OwnedType::MarketSale    
            };
            owned_type = ot;
            Some(price.0)
        } else{
            owned_type = utils::OwnedType::GiveAway;
            None
        };
        let token = if let Some(token) = self.tokens_by_id.get(&token_id){ //-- getting the Token object value related to this token_id inside the self.tokens_by_id LookupMap data structure
            token
        } else{
            env::panic_str("Found No Token"); //-- &str allocates low cost storage than the String which will get usize (usize is 64 bits or 24 bytes on 64 bits arch) * 3 (pointer, len, capacity) bytes; cause it's just the size of the str itself which is the total length of its utf8 bytes array on either stack, heap or binary which is equals to its length of utf8 bytes and due to its unknown size at compile time we must borrow it by taking a pointer to its location 
        };

        if sender_id != &token.owner_id{ //-- the owner_id inside the found token object must be the one who is transferring this NFT if not we then check if the sender is inside the approved_account_ids field otherwise we panic - the caller of the transferring method must be the owner of the NFT
            if !token.approved_account_ids.contains_key(sender_id){ //-- the token approved_account_ids hashmap must contains the sender_id key otherwise we panic
                env::panic_str("Unauthorized! Token is Not for You & You Are Not An Approved Account To Transfer This NFT"); //-- &str allocates low cost storage than the String which will get usize (usize is 64 bits or 24 bytes on 64 bits arch) * 3 (pointer, len, capacity) bytes; cause it's just the size of the str itself which is the total length of its utf8 bytes array on either stack, heap or binary which is equals to its length of utf8 bytes and due to its unknown size at compile time we must borrow it by taking a pointer to its location
            }
            
            /////
            /////// ➔ people that have been approved can also transfer NFTs on behalf of the owner means if the sender_id wasn't equal to the token owner_id doesn't mean that we can't transfer NFT at all cause the sender_id might be inside the approved_account_ids hashmap
            /////// ➔ check if both the account trying to transfer is in the approved list and they correspond to the correct approval id
            /////// ➔ approved_account_ids is a mapping based collection between all approved accounts and their approval_id corresponding to their index inside the hashmap which is the next_approval_id field and it started from 0 
            /////
            
            if let Some(enforced_approval_id) = approval_id{ //-- if there was an approval_id we must check that the sender's approval_id is the same as the once inside the approved_account_ids field
                if token.approved_account_ids.contains_key(sender_id){ //-- checking that approved_account_ids field contains the approval_id related to the sender_id
                    let actual_approved_id = token.approved_account_ids.get(sender_id).unwrap(); //-- unwrapping will never panic cause once we've arrived here means we have the approval_id of the sender_id - getting the approval_id related to the sender_id
                    if actual_approved_id != &enforced_approval_id{ //-- we must borrow the enforced_approval_id using & cause the left hand side of the condition is a borrowed type
                        env::panic_str("The Actual Approved Id Is Different From The Given Approval Id"); //-- &str allocates low cost storage than the String which will get usize (usize is 64 bits or 24 bytes on 64 bits arch) * 3 (pointer, len, capacity) bytes; cause it's just the size of the str itself which is the total length of its utf8 bytes array on either stack, heap or binary which is equals to its length of utf8 bytes and due to its unknown size at compile time we must borrow it by taking a pointer to its location
                    }
                } else{ //-- we didn't found any approval_id related to the sender_id
                    env::panic_str("This Sender Is Not An Approved Account"); //-- &str allocates low cost storage than the String which will get usize (usize is 64 bits or 24 bytes on 64 bits arch) * 3 (pointer, len, capacity) bytes; cause it's just the size of the str itself which is the total length of its utf8 bytes array on either stack, heap or binary which is equals to its length of utf8 bytes and due to its unknown size at compile time we must borrow it by taking a pointer to its location
                }
            }
        }
        
        if &token.owner_id == receiver_id{ //-- we must borrow the token.owner_id using & cause the right hand side of the condition is a borrowed type  
            env::panic_str("The Toke Owner and the Receiver Should'nt be the Same :/"); //-- at this state on the chain we must make sure that the sender_id is not sending the token to themselves - &str allocates low cost storage than the String which will get usize (usize is 64 bits or 24 bytes on 64 bits arch) * 3 (pointer, len, capacity) bytes; cause it's just the size of the str itself which is the total length of its utf8 bytes array on either stack, heap or binary which is equals to its length of utf8 bytes and due to its unknown size at compile time we must borrow it by taking a pointer to its location
        }

        self.internal_remove_token_from_owner(&token.owner_id, token_id); //-- we've defined the self to be mutable and borrowed cause we want to insert a new token_id into the self.tokens_by_id LookupMap and replace the old entry or key - removing the token from it's current owner's set (the UnorderedSet of all token_id(s) for a specific owner) 
        self.internal_add_token_to_owner(receiver_id, token_id); //-- adding the token_id to the receiver_id's set (the UnorderedSet of all token_id(s) for a specific owner); creating a new record in the self.tokens_per_owner if we didn't found any token_id set related to the receiver_id inside the self.tokens_per_owner LookupMap
        let new_token = Token{
            creator_id: token.creator_id.clone(),
            owner_id: receiver_id.clone(), //-- creating a new token object which contains the receiver_id as it's owner_id field - cloning the receiver_id to prevent from moving
            approved_account_ids: Default::default(), //-- resetting the approval account ids by creating an empty hashmap or {} for all approved account ids cause we're inserting a new token into the self.tokens_by_id field with a new owner_id
            next_approval_id: token.next_approval_id, //-- next approval id will be the old one; it might be 0 or the updated one inside the nft_approve() method
            royalty: token.royalty.clone(), //-- royaly hashmap would be the old one - cloning the royalty to prevent from moving
        };

        self.tokens_by_id.insert(token_id, &new_token); //-- inserting the new created token object into the self.tokens_by_id LookupMap, it'll replace the old entry cause hashmap based data structures use the hash of the key to validate the uniquness of their values and we must use enum based storage key if we want to add same key twice but with different values 



        //////////////////////////////////////////////////////////////////////////////////////////////
        ////////////////// CONSTRUCTING THE TRANSFER LOG AS PER THE EVENTS STANDARD //////////////////
        ////////////////////////////////////////////////////////////////////////////////////////////// 
        //// ➔ shared reference can't dereference between threads and can't move out of it cause by 
        ////     moving or dereferencing it it'll lose its ownership and lifetime while some methods and 
        ////     threads are using it; we can sovle this using as_ref() method wich converts a &wrapped 
        ////     type into &T or by cloning the type.
        //// ➔ if the approval_id wasn't None means that sender_id can be aslo an approved account
        ////    otherwise he/she is the owner of the NFT! 
        //////////////////////////////////////////////////////////////////////////////////////////////
        let auth_sender_id = sender_id.clone(); //-- cloning the sender_id to prevent from moving since we can't dereference a shared reference that doesn't implement the Copy trait
        let event_receiver_id = receiver_id.clone(); //-- cloning the receiver_id to prevent from moving since we can't dereference a shared reference that doesn't implement the Copy trait
        let old_owner_id = token.owner_id.clone(); //-- cloning the owner_id to prevent the token from moving since we can't dereference a shared reference that doesn't implement the Copy trait
        let authorized_id = if approval_id.is_some(){ //-- if the approval_id was provided, we set the authorized_id equal to the sender_id since the one who is transferring the NFT (sender_id) must be an approved account if he/she wasn't the owner of the NFT means if we have an approved account_id we can consider that the sender_id is an approved account_id thus we can set the authorized_id to the current sender_id  
            Some(auth_sender_id) 
        } else{
            None //-- the authorized_id must be None since we have no approved account which means that sender is the owner of the transferred token since no approval_id is passed  
        };
        let nft_transfer_log = EventLog{ //-- emitting the transferring event
            standard: NFT_STANDARD_NAME.to_string(), //-- the current standard
            version: NFT_METADATA_SPEC.to_string(), //-- the version of the standard based on near announcement
            event: EventLogVariant::NftTransfer(vec![NftTransferLog{ //-- the data related with the transferring event stored in a vector 
                authorized_id, //-- the authorized_id that might be None or equals to the sender_id if the approval_id wasn't None
                old_owner_id,
                new_owner_id: event_receiver_id,
                token_ids: vec![token_id.to_string()], //-- list of all minted token ids; since it might be an airdrop or giveaway batch
                price: nft_price_in_near,
                done_at: env::block_timestamp(), //-- the timestamp that this method or transaction is done with executing 
                memo: Some(check_memo), //-- the memo message which might be sale or transfer based on the passed in balance param
            }]),
        }; // NOTE - since we've implemented the Display trait for the EventLog struct thus we can convert the nft_transfer_log instance to string to log the nft transferring event info at runtime
        env::log_str(&nft_transfer_log.to_string()); //-- format!() returns a String which takes 24 bytes storage, usize * 3 (pointer, len, capacity) bytes (usize is 64 bits or 24 bytes on 64 bits arch)
        let event_vector = self.token_events.entry(token_id.to_string()).or_insert(vec![]);
        event_vector.push(nft_transfer_log);
        let mut owners_set = self.owners_per_token.get(&token_id).unwrap_or_else(|| {
            UnorderedSet::new(utils::Storagekey::OwnersPerToken.try_to_vec().unwrap())
        });
        let owner_info = utils::NFTOwnerInfo{
            account_id: token.owner_id.clone(), //-- cloning the token owner_id in order to have the token object itself to return it from this method
            owned_type,
        };
        owners_set.insert(&owner_info);
        self.owners_per_token.insert(&token_id, &owners_set);
        //////////////////////////////////////////////////////////////////////////////////////////////

        token //-- returning the transferred token
    
    }



    pub fn internal_remove_token_from_owner(&mut self, account_id: &AccountId, token_id: &TokenId){ //-- we've defined the self to be mutable and borrowed cause we want to remove a record from the self.tokens_per_owner LookupMap - removing the token from the owner's set of token_id(s)

        let mut tokens_set = if let Some(tokens_set) = self.tokens_per_owner.get(&account_id){ //-- getting the set of all token_id(s) of a specific owner inside the self.tokens_per_owner LookupMap
            tokens_set
        } else{
            env::panic_str("Token Should be Owned by the Sender :/"); //-- &str allocates low cost storage than the String which will get usize (usize is 64 bits or 24 bytes on 64 bits arch) * 3 (pointer, len, capacity) bytes; cause it's just the size of the str itself which is the total length of its utf8 bytes array on either stack, heap or binary which is equals to its length of utf8 bytes and due to its unknown size at compile time we must borrow it by taking a pointer to its location 
        };
        tokens_set.remove(token_id); //-- remving the token_id from the set of token_id(s) which are belong to the passed in account_id
        if tokens_set.is_empty(){ //-- if the set of all token_id(s) was empty after removing 
            self.tokens_per_owner.remove(account_id); //-- remove the account_id from the self.tokens_per_owner LookupMap cause this account_id has an empty list of token_id(s) or no tokens at all thus we don't want hi/her on chain
        } else{ //-- if the set of all token_id(s) wasn't empty; insert a new record into the self.tokens_per_owner LookupMap
            self.tokens_per_owner.insert(account_id, &tokens_set); //-- since the key of hashmap based data structures are unique we can't have multiple data with a same key thus we can update the set of all token_id(s) related to a specific account_id by inserting the updated token_set as the value of the self.tokens_per_owner LookupMap for that account_id
        }
    
    }

}