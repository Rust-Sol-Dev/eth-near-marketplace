



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





// --------------------------------
// --- payable function process ---
//      1 - ensure that the user has attached at least on yoctoNAER for the storage cost and security reasons like avoiding the DDOS attack on the contract by making sure that the caller has enough amount to call this and is not an intruder
//      2 - then calculate the storage used in u64 bits or 8 bytes maximum (usize on 64 bits arch system) of mutating the state of the contract like mutating any collection inside the contract struct like the total size of a new entry added inside the collection or the total size of the removed entries
//      3 - finally call refund_deposit() method to calculate the total costs for that bytes and refund to the caller any execess if there was an attached which was larger than the total storage cost or any removal entry process which will free up some storage which we must refund the caller based on the freed up storage bytes  



// NOTE - define the trait and struct methods as &mut self if you want to mutate their fields
// NOTE - &self means all fields in struct are in their borrowed form thus we have a shared reference of them inside the lifetime of the struct object also we'll have a valid lifetime for object after calling its methods in current scope which won't be moved by first call
// NOTE - inside all smart contracts are methods which are transactions and in order to call them from other contracts we have to use cross contract call promise feature by defining a trait and bound it to #[ext_contract()] proc macro attribute inside the caller contract; by doing this we're extending the receiver_id's contract actor (a hypothetical contract name of course since the method that we've defined inside the trait is already defined on the receiver_id's contract actor) interface which contains a method that must be scheduled to call on the receiver_id's contract actor inside the caller contract actor account and get the result of it inside the callback method of the caller contract
// NOTE - gas fee is the computational fee paied as raward to validators by attaching them (in gas units) in scheduling function calls in which they mutate the state of the contract which face us cpu usage costs; and also the remaining deposit will get pay back as a refund to the caller by the near protocol
// NOTE - deposit or amount is the cost of the method and must be attached (in yocto Ⓝ (1e24) or near) for scheduling payable function calls based on storages they've used by mutating the state of the contract on chain like updating a collection field inside the contract struct and we have to get pay back the remaining deposit as a refund to the caller and that's what the refund_deposit() function does
// NOTE - if a contract method mutate the state like adding a data into a collection field inside the contract struct; the method must be a payable method (we need to tell the caller attach deposit to cover the cost) and we have to calculate the storage used for updating the contract state inside the function to tell the caller deposit based on the used storage in bytes (like the total size of the new entry inside a collection) then refund the caller with the extra tokens he/she attached
// NOTE - every cross contract calls for communicating between contract actor accounts in cross sharding pattern takes up cpu usage and network laoding costs which forces us to attach gas units in the contract method call in which the cross contract call method is calling to pass it through the calling of the cross contract call method
// NOTE - in nft_approve() method we add a new entry into approved_account_ids field of the token object means we mutate the state of the contract by allocating extra storage on chain to insert a new approved account_id into the mentioned hashmap thus we have to pay for it from caller's deposit and refund the caller if there was any execess storage cost 
// NOTE - a payable method has &mut self as its first param and all calculated storage must of type u64 bits or 8 bytes maximum length (64 bits arch system usize) 
// NOTE - caller in payable methods must deposit one yocto Ⓝ (1e24) for security purposes like always make sure that the user has some $NEAR in order to call this means only those one who have $NEARS can call this method to avoid DDOS attack on this method
// NOTE - in near version 3 `ext_contract` proc macro attribute takes a Rust Trait and converts it to a module with static methods and each of these static methods takes positional arguments defined by the Trait then the last three arguments the receiver_id, the attached deposit and the amount of gas are used behind the scenes and returns a new Promise
// NOTE - a payable method can be used to pay the storage cost, the escrow price or the gas fee and the excess will be refunded by the contract method or the NEAR protocol
// NOTE - gas fee is the computational cost which must be paid if we’re doing cross contract call or moving between shards and actor cause this action will cost some cpu usage performance and must be attached separately in its related call from the cli 
// NOTE - amount or deposit is the cost of the payable function which can be either the cost of the storage usage for mutating contract or the cost of some donation or escrow ops
// NOTE - in order to call and schedule a promise or future object method from other contract actor account we have to define a trait and bound it to #[ext_contract()] proc macro which contains the method signature of the second contract actor account finally we can call in here and catch the the result of the scheduled promise of future object using the NEAR cross contract call syntax





















// ---------------------------------------
//   NFT CORE STANDARD INTERFACE METHODS
// ---------------------------------------
pub trait NoneFungibleTokenCore{ //-- defining an object safe trait for NFT core queries, we'll implement this for any contract that wants to interact with NFT core queries - object safe traits are not bounded to trait Sized thus they won't return Self or have generic params in its methods if so then some space should have been allocated inside the memory for Self or that generic param and it will no longer an abstract type

    fn nft_approve(&mut self, token_id: TokenId, account_id: AccountId, msg: Option<String>); //-- approving an account_id to transfer a token on behalf of the owner - since we want to update the token object fields we must define the first param as &mut self &mut self (a shared mutable pointer or reference) and bind the method to #[payable] proc macro attribute
    fn nft_is_approved(&self, token_id: TokenId, approved_account_id: AccountId, approval_id: Option<u64>) -> bool; //-- checking whether that the passed in account_id plus its approval_id is approved for tranfering the token on behalf of the owner or not - since we want to iterate through the approved_account_ids field inside the token object to check that current account has the correct approval_id or not thus there is no need to define the first param &mut self and bind the method to #[payabele] proc macro attribute
    fn nft_revoke(&mut self, token_id: TokenId, account_id: AccountId); //-- revoking a specific account from transferring the token on behalf of the owner - since we want to update the token object fields we must define the first param as &mut self (a shared mutable pointer or reference) and bind the method to #[payable] proc macro attribute
    fn nft_revoke_all(&mut self, token_id: TokenId); //-- revoking all accounts from transferring the token on behalf of the owner

}


#[near_bindgen] //-- implementing the #[near_bindgen] proc macro attribute on `NFTContract` struct to compile all its methods to wasm so we can call them in near cli
impl NoneFungibleTokenCore for NFTContract{ //-- implementing the NoneFungibleTokenCore trait for our main `NFTContract` struct to extend its interface; bounding the mentioned trait to the `NFTContract` struct to query NFT core (nft_* methods standards) infos

    
    #[payable] //-- means the following would be a payable method and the caller must pay for that and must get pay back the remaining deposit or any excess that is unused at the end by refunding the caller account by our contract (something like refund_deposit() method) or the NEAR protocol - we should bind the #[near_bindgen] proc macro attribute to the contract struct in order to use this proc macro attribute
    fn nft_approve(&mut self, token_id: TokenId, account_id: AccountId, msg: Option<String>){ //-- allow a specific account_id like a marketplace contract actor account to approve a token_id on behalf of the owner and list it on the marketplace

        let storage_deposit = env::attached_deposit();
        assert!(
            storage_deposit >= STORAGE_PER_SALE,
            "SMARTIES : Requires deposit of {} for market calls",
            STORAGE_PER_SALE
        );

        utils::panic_atleast_one_yocto(); //-- ensuring that the user has attached at least one yocto Ⓝ (1e24) to the call to pay for the storage and security reasons (only those caller that have at least 1 yocto Ⓝ (1e24) can call this method; by doing this we're avoiding DDOS attack on the contract) on the contract by forcing the users to sign the transaction with his/her full access key which will redirect them to the NEAR wallet; we'll refund any excess amount from the storage later after calculating the required storage cost for adding new approved account_id 
        let mut token = if let Some(token) = self.tokens_by_id.get(&token_id){ //-- getting the Token object value related to this token_id inside the self.tokens_by_id LookupMap data structure
            token
        } else{
            env::panic_str("Found No Token"); //-- &str allocates low cost storage than the String which will get usize (usize is 64 bits or 24 bytes on 64 bits arch) * 3 (pointer, len, capacity) bytes; cause it's just the size of the str itself which is the total length of its utf8 bytes array on either stack, heap or binary which is equals to its length of utf8 bytes and due to its unknown size at compile time we must borrow it by taking a pointer to its location
        };

        if &env::predecessor_account_id() != &token.owner_id{ //-- since we can borrow the token.owner_id String thus both side of the condition must be equal together means we have to borrow the env::predecessor_account_id() String too - the last (current) caller of this method must be the owner of the token cause only the token owner must call this method to approve an account to transfer the token on his/her behalf
            env::panic_str("The Caller Is Not The Owner of The Token! Cause Only The Token Owner Can Approve An Account"); //-- &str allocates low cost storage than the String which will get usize (usize is 64 bits or 24 bytes on 64 bits arch) * 3 (pointer, len, capacity) bytes; cause it's just the size of the str itself which is the total length of its utf8 bytes array on either stack, heap or binary which is equals to its length of utf8 bytes and due to its unknown size at compile time we must borrow it by taking a pointer to its location
        }
        

        let approval_id = token.next_approval_id; //-- getting the next_approval_id for setting a new approval if we need one
        let is_new_approval = token.approved_account_ids.insert(account_id.clone(), approval_id).is_none(); //-- inserting a new entry with this approval_id if there was no one, insert returns None if the key was not present means it'll create the new entry with this key since it didn't return the entry on this key and if that so means we're creating a new approval
        let storage_used = if is_new_approval{
            bytes_for_approved_account_id(&account_id) //-- calculating how much storage is being used to add the account to approved_account_ids hashmap cause the state of the contract has changed and we must calculate the requirement deposit based on used bytes - passing the account_id by reference in its borrowed type
        } else{
            0 //-- 0 storage is used if it wasn't a new approval and only attaching 1 yocto Ⓝ (1e24) would be enough
        };
 

        token.next_approval_id += 1; //-- incrementing the token's next_approval_id field by 1 to specify the approval account id which must be used to approve an account_id in the next call by doing that we can only allow one approved account_id to transfer the NFT on behalf of the owner at a time and avoid multi transferring at the same time
        self.tokens_by_id.insert(&token_id, &token); //-- insert the updated token back into the self.tokens_by_id collection, insert() method will update the value on second call if there was any entry with that key exists cause hashmap based data structures use the hash of the key to validate the uniquness of their values and we must use enum based storage key if we want to add same key twice but with different values in two different collections to avoid data collision
        refund_deposit(storage_used); //-- depositing some $NEARs based on used bytes in the contract and get pay back the remaining deposit or any excess that is unused at the end by refunding the caller account by our contract (something like refund_deposit() method) or the NEAR protocol; if the caller didn't attach enough it'll panic


        ////
        /////// ➔ this cross contract call from the NFT contract signed by the NFT owner to the account that the user wants to give the access to, for listing and transferring his/her NFT in there (like the marketplace contract actor account) will be scheduled only if the msg param wasn't empty which is a sale condition or the price of the NFT that must be sold out on the marketpalce
        /////// ➔ the msg param is of type String which will be deserialized on the market contract actor account insdie the nft_on_approve() method to get the price of the NFT in yocto Ⓝ (1e24) of type u128
        ////
        if let Some(msg) = msg{ //-- if msg wan't None then we initiate and schedule a cross contract call and call promise method the nft_on_approve() method to be executed on the account that we're giving access to which means someone with an NFT can list his/her own NFT on an approved account like the marketplace account by passing in a message that could be properly decoded to allow users purchases his/her NFT on the marketplace
        
        let MarketArgs {
            market_type,
            price,
            buyer_id, //-- used for offer
            started_at,
            ended_at,
            end_price,
            is_auction,
        } = near_sdk::serde_json::from_str(&msg).expect("Not valid Smarties Markte Args");
        
        //////////////////////////////////////////////////////////////////////////////////////////
        ////////////////// CONSTRUCTING THE LIST LOG AS PER THE EVENTS STANDARD //////////////////
        //////////////////////////////////////////////////////////////////////////////////////////
        let nft_list_log = EventLog{ //-- emitting the minting event
            standard: NFT_STANDARD_NAME.to_string(), //-- the current standard
            version: NFT_METADATA_SPEC.to_string(), //-- the version of the standard based on near announcement
            event: EventLogVariant::NftList(vec![NftListLog{ //-- the data related with the minting event stored in a vector 
                authorized_id: account_id.clone(),
                owner_id: token.owner_id.clone(),
                token_ids: vec![token_id.clone()],
                price: price.unwrap().0,
                done_at: env::block_timestamp(), //-- the timestamp that this method or transaction is done with executing 
                msg: Some(msg.clone()), //-- json stringify sale condition
            }]),
        }; // NOTE - since we've implemented the Display trait for the EventLog struct thus we can convert the nft_list_log instance to string to log the nft minting event info at runtime
        env::log_str(&nft_list_log.to_string()); //-- format!() returns a String which takes 24 bytes storage, usize * 3 (pointer, len, capacity) bytes (usize is 64 bits or 24 bytes on 64 bits arch)
        let event_vector = self.token_events.entry(token_id.clone()).or_insert(vec![]);
        event_vector.push(nft_list_log);
        //////////////////////////////////////////////////////////////////////////////////////////
        
        let log_msg = format!(":::: SMARTIES: the msg param in nft contract is {:?}", msg);
        env::log_str(&log_msg);
        
        let log_depoist = format!(":::: SMARTIES: the attached deposit in nft contract is {:?}", storage_deposit);
        env::log_str(&log_depoist);
        
        ext_non_fungible_approval_receiver::ext(account_id) //-- the account_id that this method must be called and executed inside which is the one who is approved for transferring the token on behalf of the owner and is the one who has access to do this - account_id param is the one who is responsible for executing this call like the market contract actor account - no need to clone the current_account_id cause we're passing it by reference or as a borrowed type
            .with_attached_deposit(storage_deposit) //-- no deposit is required from the caller for calling the nft_on_approve() cross contract call promise method since this method doesn't require any deposit amount
            .with_static_gas(env::prepaid_gas() - GAS_FOR_NFT_APPROVE) //-- prepaid_gas() method returns the amount of gas attached to the call via near cli that can be used to pay the gas fees | attached gas - required gas for calling nft_approve() method is the total gas fee which will be deposited in yocto Ⓝ (1e24) from the caller wallet for this transaction call
            .nft_on_approve( //-- initiating the receiver's corss contract call by creating a transaction which is a promise (future object) ActionReceipt object which returns obviously a promise or a future object which contains an async message including the data coming from the receiver_id's contract actor once it gets executed - calling the nft_on_approve() cross contract call promise method on the receiver side (like a market contract actor account) from the extended receiver_id's contract actor interface which is `ext_non_fungible_approval_receiver`; by calling this method we can do whatever we want inside the receiver_id's contract actor which is the account that we're giving access and approving to transfer the token on behalf of the owner like the market contract actor account - calling the nft_on_approve() cross contract call promise method to schedule an ActionReceipt object which contains async message or a future object which must be solved by executing the mentioned method on the token.owner_id's contract actor
                token_id,
                token.owner_id, //-- this is the signer_id or the caller of the nft_approve() method which is the one who started the chain of cross contract call transaction - passing the NFT owner_id which must not be the NFT contract owner or the one who owns this contract
                approval_id, //-- this is the approval_id of the account_id who has access to transfer the token on behalf of the owner like the market contract actor account
                msg,
            )
            .as_return(); //-- it'll return a promise without getting its result using a callback which might be solved or failed; it depends on the result of cross contract call inside the nft_on_approve() method which is on the market contract actor account
        }
 
    } 



    fn nft_is_approved(&self, token_id: TokenId, approved_account_id: AccountId, approval_id: Option<u64>) -> bool{ //-- checking that a passed in account_id is approved to transfer the token on behalf of the owner and has the same approval_id as the one provided or not
        match self.tokens_by_id.get(&token_id){ //-- getting the token object related to the token_id (passed by reference to borrow it) if there is some token object from the self.tokens_by_id LookupMap
            Some(token) => { //-- if there was some token found with this token_id
                let approval_id_for_this_account_id = token.approved_account_ids.get(&approved_account_id); //-- getting the approval id of the passed in account_id or approved_account_id from the approved_account_ids field of the found token object
                match approval_id_for_this_account_id{
                    Some(found_approval) => { //-- if there was some approval_id found for passed in account_id
                        if let Some(passed_in_approval_id) = approval_id{ //-- if a specific approval_id was passed into the method
                            passed_in_approval_id == *found_approval //-- return true if the passed in approval_id matches the actual found approval_id for the passed in account_id; if true means the account_id is the one who is approved to transfer the NFT on behalf of the owner - since both side of the condition must be the same type we can dereference the found_approval to get it out of its borrowed form 
                        } else{
                            true //-- returning true since there was no approval_id passed into the function and we found the approval_id related to the passed in account_id inside the approved_account_ids hashmap thus it's ok to grant the access for the passed in account_id to transfer the token on behalf of the owner without the passed in approval_id 
                        }
                    },
                    None => {
                        false //-- returning false since we found no approval_id for the passed in account_id means the passed in account_id is not approved for transferring the token on behalf of the owner and has no access to do that
                    },
                }
            },
            None => { //-- means we found no token with the passed in token_id
                env::panic_str("Found No Token"); //-- &str allocates low cost storage than the String which will get usize (usize is 64 bits or 24 bytes on 64 bits arch) * 3 (pointer, len, capacity) bytes; cause it's just the size of the str itself which is the total length of its utf8 bytes array on either stack, heap or binary which is equals to its length of utf8 bytes and due to its unknown size at compile time we must borrow it by taking a pointer to its location
            },
        }
    }



    #[payable] //-- means the following would be a payable method and the caller must pay for that and must get pay back the remaining deposit or any excess that is unused at the end by refunding the caller account by our contract (something like refund_deposit() method) or the NEAR protocol - we should bind the #[near_bindgen] proc macro attribute to the contract struct in order to use this proc macro attribute
    fn nft_revoke(&mut self, token_id: TokenId, account_id: AccountId){ //-- removing the passed in account_id from approved_account_ids hashmap of a found token object related to a specific token_id and refund the owner for the storage being releas
        utils::panic_one_yocto(); //-- ensuring that the user has attached exactly one yocto Ⓝ (1e24) to the call to pay for the storage and security reasons (only those caller that have at least 1 yocto Ⓝ (1e24) can call this method; by doing this we're avoiding DDOS attack on the contract) on the contract by forcing the users to sign the transaction with his/her full access key which will redirect them to the NEAR wallet; we'll refund any excess amount from the storage later after calculating the required storage cost 
        match self.tokens_by_id.get(&token_id){ //-- getting the token object related to the token_id (passed by reference to borrow it) if there is some token object from the self.tokens_by_id LookupMap
            Some(mut token) => { //-- if there was some token found with this token_id - we've defined the token to be mutable cause we want to remove an entry from its approved_account_ids hashmap if there was any
                if &env::predecessor_account_id() != &token.owner_id{ //-- since we can borrow the token.owner_id String thus both side of the condition must be equal together means we have to borrow the env::predecessor_account_id() String too - the last (current) caller of this method must be the owner of the token cause only the token owner must call this method to remove an approved account from the approved_account_ids hashmap
                    env::panic_str("The Caller Is Not The Owner of The Token!"); //-- &str allocates low cost storage than the String which will get usize (usize is 64 bits or 24 bytes on 64 bits arch) * 3 (pointer, len, capacity) bytes; cause it's just the size of the str itself which is the total length of its utf8 bytes array on either stack, heap or binary which is equals to its length of utf8 bytes and due to its unknown size at compile time we must borrow it by taking a pointer to its location
                }
                let token_owner = env::predecessor_account_id(); //-- getting the caller of this method which is the owner of the token also
                if token.approved_account_ids.remove(&account_id).is_some(){ //-- if the account_id was inside approved_account_ids hashmap we remove it and true will be returned thus the body of the if condition will be executed
                    utils::refund_approved_account_ids_iter(token_owner, [account_id].iter()); //-- refunding the fund released to the caller which is the owner of the token by removing the account_id from approved_account_ids hashmap - since the approved_account_ids argument must be an iterator we've converted the array of account_ids which has only one element to an iterator so we can map it inside the refund_approved_account_ids_iter() function; we don't need to borrow the [account_id] by taking a reference to it since array in rust has no fixed size at compile time, the iter() method will handle the lifetime for us when we call the iter() method on a type that we want to make it an iterator, the iter() method will create a new iterator from the type by passing the &self to its new() method 
                    self.tokens_by_id.insert(&token_id, &token); //-- insert the updated token back into the self.tokens_by_id collection, insert() method will update the value on second call if there was any entry with that key exists cause hashmap based data structures use the hash of the key to validate the uniquness of their values and we must use enum based storage key if we want to add same key twice but with different values in two different collections to avoid data collision
                }
            },
            None => { //-- means we found no token with the passed in token_id
                env::panic_str("Found No Token"); //-- &str allocates low cost storage than the String which will get usize (usize is 64 bits or 24 bytes on 64 bits arch) * 3 (pointer, len, capacity) bytes; cause it's just the size of the str itself which is the total length of its utf8 bytes array on either stack, heap or binary which is equals to its length of utf8 bytes and due to its unknown size at compile time we must borrow it by taking a pointer to its location
            },
        }
    }


    
    #[payable] //-- means the following would be a payable method and the caller must pay for that and must get pay back the remaining deposit or any excess that is unused at the end by refunding the caller account by our contract (something like refund_deposit() method) or the NEAR protocol - we should bind the #[near_bindgen] proc macro attribute to the contract struct in order to use this proc macro attribute
    fn nft_revoke_all(&mut self, token_id: TokenId){ //-- removing all account_ids inside the approved_account_ids hashmap related to a specifi token_id from transferring the token on behalf of the owner
        utils::panic_one_yocto(); //-- ensuring that the user has attached exactly one yocto Ⓝ (1e24) to the call to pay for the storage and security reasons (only those caller that have at least 1 yocto Ⓝ (1e24) can call this method; by doing this we're avoiding DDOS attack on the contract) on the contract by forcing the users to sign the transaction with his/her full access key which will redirect them to the NEAR wallet; we'll refund any excess amount from the storage later after calculating the required storage cost 
        match self.tokens_by_id.get(&token_id){ //-- getting the token object related to the token_id (passed by reference to borrow it) if there is some token object from the self.tokens_by_id LookupMap
            Some(mut token) => { //-- if there was some token found with this token_id - we've defined the token to be mutable cause we want to remove all entries from its approved_account_ids hashmap
                if &env::predecessor_account_id() != &token.owner_id{ //-- since we can borrow the token.owner_id String thus both side of the condition must be equal together means we have to borrow the env::predecessor_account_id() String too - the last (current) caller of this method must be the owner of the token cause only the token owner must call this method to remove all approved accounts from the approved_account_ids hashmap
                    env::panic_str("The Caller Is Not The Owner of The Token!"); //-- &str allocates low cost storage than the String which will get usize (usize is 64 bits or 24 bytes on 64 bits arch) * 3 (pointer, len, capacity) bytes; cause it's just the size of the str itself which is the total length of its utf8 bytes array on either stack, heap or binary which is equals to its length of utf8 bytes and due to its unknown size at compile time we must borrow it by taking a pointer to its location
                }
                let token_owner = env::predecessor_account_id(); //-- getting the caller of this method which is the owner of the token also
                if !token.approved_account_ids.is_empty(){ //-- make sure that the approved_account_ids hashmap is not empty so we can revoke all account_ids  
                    utils::refund_approve_account_ids(token_owner, &token.approved_account_ids); //-- we must first refund to the caller which is the owner of the token then clear the approved_account_ids hashmap cause we can't pass an empty hashmap to the function obviously - refunding the owner for releasing the storage used up by the approved account ids (cause he/she doesn't own any allocated storage for approval_account_ids for the token cause the token has been transferred to another one) based on the transferred token object since the owner paid for it when he/she was minting the NFT inside the nft_mint() method - passing the approved_account_ids in its borrowed from by taking a reference to its location using & to prevent from moving and losing its ownership so we can have it in later scopes
                    token.approved_account_ids.clear(); //-- clearing the map and removing all key-value pairs; it'll keep the allocated memory for later usage
                    self.tokens_by_id.insert(&token_id, &token); //-- insert the updated token back into the self.tokens_by_id collection, insert() method will update the value on second call if there was any entry with that key exists cause hashmap based data structures use the hash of the key to validate the uniquness of their values and we must use enum based storage key if we want to add same key twice but with different values in two different collections to avoid data collision
                }
            },
            None => { //-- means we found no token with the passed in token_id
                env::panic_str("Found No Token"); //-- &str allocates low cost storage than the String which will get usize (usize is 64 bits or 24 bytes on 64 bits arch) * 3 (pointer, len, capacity) bytes; cause it's just the size of the str itself which is the total length of its utf8 bytes array on either stack, heap or binary which is equals to its length of utf8 bytes and due to its unknown size at compile time we must borrow it by taking a pointer to its location
            },
        }

    }


}












// ----------------------------------------
//     CROSS CONTRACT CALLS' INTERFACES
// ----------------------------------------
#[ext_contract(ext_non_fungible_approval_receiver)] //-- ext_non_fungible_approval_receiver name that we passed in #[ext_contract()] proc macro is the name of the contract (a hypothetical contract name of course) that we're extending its interface for cross contract call and creating transaction which is a promise (future object) ActionReceipt object and means we want to call the following methods inside that contract which contains a transaction which is a promise (future object) ActionReceipt object that must be executed later
trait NoneFungibleTokenApprovalsReceiver{ //-- this trait which contains the cross conract call methods will extend the interface of the receiver_id's contract actor with a name inside the #[ext_contract()] proc macro which specifies the extended interface contract name on this contract  

    fn nft_on_approve(&mut self, token_id: TokenId, owner_id: AccountId, approval_id: u64, msg: String); // NOTE - to get the result of this method it must be existed inside the receiver contract actor account like the market account in order to schedule it in this contract to be executed in market contract actor account 

}