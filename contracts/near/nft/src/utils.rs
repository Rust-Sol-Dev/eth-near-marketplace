







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


 
    

use std::{mem::size_of, fmt::format};
use crate::*;  // loading all defined crates, structs and functions from the root crate which is lib.rs in our case













// ------------------------------ example of near actor design pattern
// -------------------------------------------------------------------------------------------------
// -------------------------------------------------------------------------------------------------
// https://docs.near.org/docs/tutorials/contracts/xcc-rust
// 
// 
// NOTE - since every transaction is a promise or a future object thus they must communicate 
//        with each other using the id of the contract actor account to pass async messages.
// -------------------------------------------------------------------------------------------------
pub fn actor_ds_ex(){
    
    

    // creating a new promise (future object) ActionReceipt from accountA.testnet account_id which will 
    // create a new empty promise (future object) ActionReceipt (async message) to pass it between 
    // contract actor through mpsc channel using actor address which is the 
    // hash of the account_id (accountA.testnet) in here
    let scheduling_promise_in_account = "accountA.testnet".parse().unwrap(); //-- building the account_id from the &str
    let promise_id = env::promise_batch_create(&scheduling_promise_in_account); //-- a u64 bits or 8 bytes id which could be a pointer to the address of the promise
    
    


    env::promise_batch_action_function_call( //-- filling the created promise (future object) ActionReceipt with a transaction like calling the ft_balance_of() method of the current contract actor which is accountA.testnet account
        promise_id, //-- this is the id of the created promise which contains an empty promise (future object) ActionReceipt 
        "ft_balance_of", //-- calling ft_balance_of() method of the current contract actor which is accountA.testnet
        &json!({"account_id": "accountB.testnet".to_string()}).to_string().into_bytes(), //-- data to be passed to the ft_balance_of() method in form of utf8 bytes
        0, //-- amount of yocto Ⓝ (1e24) to attach for this transaction which in our case is calling the ft_balance_of() method of the accountA.testnet contract actor
        Gas(5_000_000_000_000) //-- gas fee to attach
    );
    


    
    // the following is a callback promise (future object) ActionReceipt to receive the DataReceipt of the promise_id (the first promise) 
    // the ActionReceipt of this promise is dependent on the previous promise (future object) ActionReceipt whenever it gets solved we'll 
    // have the DataReceipt inside the following created promise (future object) ActionReceipt
    let current_account_id = env::current_account_id().as_str().parse().unwrap();
    let callback_promise_id = env::promise_batch_then( //-- creating the second promise which also will create an empty ActionReceipt to fulfill the callback promise with the incoming message or receipt which contains the data from the first promise (future object) ActionReceipt
        promise_id, //-- this is the id of the first promise (future object) ActionReceipt which contains the DataReceipt either pending, postponed or solved coming from the first promise (future object) ActionReceipt
        &current_account_id, //-- the current_account_id() which is the one who owns this contract (accountA.testnet) is the receiver of this created promise (future object) ActionReceipt    
    );
    



    // attacing a callback function to the callback promise (future object) ActionReceipt
    env::promise_batch_action_function_call(
        callback_promise_id, //-- this is the id of the second promise (future object) ActionReceipt which contains the DataReceipt from the first promise (future object) ActionReceipt
        "my_callback", //-- the callback function which must be call after fulfilling the promise with the DataReceipt coming from the first promise (future object) ActionReceipt
        b"{}", //-- data to be passed to the my_callback() method in form of utf8 bytes
        0, //-- amount of yocto Ⓝ (1e24) to attach for this transaction which in our case is calling the ft_balance_of() method of the accountA.testnet contract actor
        Gas(5_000_000_000_000) //-- gas fee to attach
    );



    env::promise_return(callback_promise_id) //-- returning the solved DataReceipt of the callback promise 
    
    
}





#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct MarketArgs {
    pub market_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<U128>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_id: Option<AccountId>, // offer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_price: Option<U128>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<U64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ended_at: Option<U64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_auction: Option<bool>,
}





// ------------------------------ internal functions 
// -------------------------------------------------------------------------------------------------
// -------------------------------------------------------------------------------------------------
// -------------------------------------------------------------------------------------------------
pub fn panic_not_self(){
    if env::current_account_id() != env::predecessor_account_id(){ //-- current_account_id() is the one who owns this contract - the owner (or the signer of the contract if it's not a cross contract call) is not the previous contract actor account or the last (current) caller of this method
        env::panic_str("last caller is not the owner of this contract"); //-- &str allocates low cost storage than the String which will get usize (usize is 64 bits or 24 bytes on 64 bits arch) * 3 (pointer, len, capacity) bytes; cause it's just the size of the str itself which is the total length of its utf8 bytes array on either stack, heap or binary which is equals to its length of utf8 bytes and due to its unkown size at compile time we must borrow it by taking a pointer to its location
    }
}


pub fn panic_one_yocto(){
    if env::attached_deposit() != 1{
        env::panic_str("Requires attached deposit of exactly 1 yocto Ⓝ (1e24)"); //-- &str allocates low cost storage than the String which will get usize (usize is 64 bits or 24 bytes on 64 bits arch) * 3 (pointer, len, capacity) bytes; cause it's just the size of the str itself which is the total length of its utf8 bytes array on either stack, heap or binary which is equals to its length of utf8 bytes and due to its unkown size at compile time we must borrow it by taking a pointer to its location
    }
}


pub fn panic_atleast_one_yocto(){
    if env::attached_deposit() < 1{
        env::panic_str("Requires attached deposit of at least 1 yocto Ⓝ (1e24)"); //-- &str allocates low cost storage than the String which will get usize (usize is 64 bits or 24 bytes on 64 bits arch) * 3 (pointer, len, capacity) bytes; cause it's just the size of the str itself which is the total length of its utf8 bytes array on either stack, heap or binary which is equals to its length of utf8 bytes and due to its unkown size at compile time we must borrow it by taking a pointer to its location
    }
}


pub fn hash_account_id(account_id: &AccountId) -> CryptoHash{ //-- we'll pass the account_id as a borrowed type to this function - account_id in CryptoHash format is a 32 bytes or 256 bits which will be 64 chars in hex
    let mut hash = CryptoHash::default(); //-- getting the default hash which will be 32 elements of utf8 bytes (8 bits or 1 byte long for each)
    hash.copy_from_slice(&env::sha256(account_id.as_bytes())); //-- extending the defined hash with the borrowed type of the bytes of the hash of the account_id by converting its String into utf8 bytes first; the source or the length of the hash of the account_id bytes must be the same as the defined hash variable 
    hash
}


pub fn refund_deposit(storage_used: u64){ //-- refunding the initial deposit based on the amount of storage that was used up - all balances are of type u128 to cover the yocto Ⓝ (1e24) amounts
    let required_cost = env::storage_byte_cost() * Balance::from(storage_used); //-- getting the required cost of mutating the contract state on the chain based on specified balance which is of type u128 from the used or released storage - storage_byte_cost() is the balance needed to store one byte on chain    
    let attached_deposit = env::attached_deposit(); //-- getting the attached deposit - attached_deposit() method will get the balance that was attached to the call that will be immediately deposited before the contract execution starts; this is the minimum balance required to call the nft_mint() method 0.1 $NEAR is attached and the caller will get refunded any excess that is unused at the end 
    if required_cost > attached_deposit{ //-- 1 yocto is 10^-24
        let panic_message = format!("Need {} yocto Ⓝ (1e24) for the storage cost", required_cost); //-- format!() returns a String which takes 24 bytes storage, usize * 3 (pointer, len, capacity) bytes (usize is 64 bits or 24 bytes on 64 bits arch)
        env::panic_str(panic_message.as_str()); //-- &str allocates low cost storage than the String which will get usize (usize is 64 bits or 24 bytes on 64 bits arch) * 3 (pointer, len, capacity) bytes; cause it's just the size of the str itself which is the total length of its utf8 bytes array on either stack, heap or binary which is equals to its length of utf8 bytes and due to its unknown size at compile time we must borrow it by taking a pointer to its location
    }
    let refund = attached_deposit - required_cost; //-- refunding the owner account by subtracting the required_cost from his/her attached_deposit in yocto Ⓝ (1e24)
    if refund > 1{ //-- if the refund was greater than 1 yocto Ⓝ (1e24), means we have to get pay back the remaining deposit as a refund to the predecessor_account_id - refund is of type u128 or 16 bytes
        Promise::new(env::predecessor_account_id()).transfer(refund); //-- transfer the refund (using system account_id) to the predecessor_account_id which is the previous contract actor account and the last (current) caller of a method - we've scheduled a promise object here to create a transaction for transferring some $NEARs asyncly to the predecessor account which is the last caller as the receiver_id's contract actor and where the scheduled promise must be executed in
    }
}


pub fn refund_deposit_minting_payout(storage_used: u64, creator_fee: u128, creator_id: AccountId){ //-- refunding the initial deposit based on the amount of storage that was used up - all balances are of type u128 to cover the yocto Ⓝ (1e24) amounts
    let attached_deposit = env::attached_deposit(); //-- getting the attached deposit - attached_deposit() method will get the balance that was attached to the call that will be immediately deposited before the contract execution starts; this is the minimum balance required to call the nft_mint() method 0.1 $NEAR is attached and the caller will get refunded any excess that is unused at the end 
    let required_cost = env::storage_byte_cost() * Balance::from(storage_used); //-- getting the required cost of mutating the contract state on the chain based on specified balance which is of type u128 from the used or released storage - storage_byte_cost() is the balance needed to store one byte on chain    
    if required_cost > attached_deposit{
        let panic_message = format!("Need {} yocto Ⓝ (1e24) for the storage cost", required_cost); //-- format!() returns a String which takes 24 bytes storage, usize * 3 (pointer, len, capacity) bytes (usize is 64 bits or 24 bytes on 64 bits arch)
        env::panic_str(panic_message.as_str()); //-- &str allocates low cost storage than the String which will get usize (usize is 64 bits or 24 bytes on 64 bits arch) * 3 (pointer, len, capacity) bytes; cause it's just the size of the str itself which is the total length of its utf8 bytes array on either stack, heap or binary which is equals to its length of utf8 bytes and due to its unknown size at compile time we must borrow it by taking a pointer to its location
    }
    let excess_to_pay_creator = attached_deposit - required_cost;
    let refund = u128::saturating_sub(creator_fee, excess_to_pay_creator);
    Promise::new(creator_id).transfer(creator_fee); //-- paying out the collection creator from the contract itself - scheduling a promise object which will call the built-in method of the near protocol the transfer() method which will be executed later asyncly to transfer Ⓝ to the creator contract acctor account
    if refund > 1{
        Promise::new(env::predecessor_account_id()).transfer(refund); //-- transfer the refund (using system account_id) to the predecessor_account_id which is the previous contract actor account and the last (current) caller of a method - we've scheduled a promise object here to create a transaction for transferring some $NEARs asyncly to the predecessor account which is the last caller as the receiver_id's contract actor and where the scheduled promise must be executed in
    }
}


pub fn bytes_for_approved_account_id(account_id: &AccountId) -> u64{ //-- calculating the exact amount of bytes of the passed in account_id in u64 bits or 8 bytes maximum (usize on 64 bits arch system)
    (
        account_id.as_str().len() as u64 //-- getting the length (in bytes) of the &str of the account_id String (as u64 cause the default is usize) which is equals to its length of utf8 bytes array (&[u8])
        + 4 //-- adding 4 bytes to the length of the &str - this 4 bytes or 32 bits is added by Borsh serialization to store the length of the string when deserializing it into String from its utf8 bytes
        + size_of::<u64>() as u64 //-- adding the total size of the u64 bits in bytes as u64 bits or 8 bytes maximum
    ) as u64 //-- casting all calculated bytes into u64 bits which will be something between 0 and 18446744073709551615 (2^64 − 1) cause the type of storage in near protocol is u64 bits or 8 bytes maximum (usize on 64 bits arch system)
}


pub fn refund_approved_account_ids_iter<'a, I>(account_id: AccountId, approved_account_ids: I) -> Promise where I: Iterator<Item = &'a AccountId>{ //-- returning a promise or future object containing the refunding result of the the passed in account_id which is a cross contract call promise method the pre defined transfer() method - we've bounded the I (the approved_account_ids generic type) which can be either of type Keys<String, u64> or an array to Iterator trait over AccountIds items means the passed in type must be an iterator so we can iterate over its elements which are of type &AccountId and obviously we need a lifetime also for each borrowed type or pointer to account_id
    let storage_released: u64 = approved_account_ids.map(bytes_for_approved_account_id).sum(); //-- calculating the sum of all storage bytes taken up by each approved account_id inside the approved_account_ids field because by transferring an NFT to someone else we're adding a new token object to self.tokens_by_id field in which the approved_account_ids field will be a new empty hashmap on chain thus we have to refund the one who tranferred the NFT (the caller or the NFT owner) cause he/she have paied for this (approved account_ids) inside the nft_mint() method 
    let required_cost = env::storage_byte_cost() * Balance::from(storage_released); //-- getting the required cost of mutating the contract state on the chain based on specified balance which is of type u128 from the used or released storage - storage_byte_cost() is the balance needed to store one byte on chain
    Promise::new(account_id).transfer(required_cost) //-- creating a new promise or future object for cross contract call process which will transfer the amount of released storage in $NEAR back to the passed in account_id (the caller or the NFT owner) or the one who have paid for storing approved accounts on the contract when an NFT is minted inside the nft_mint() method - we've scheduled a promise object here to create a transaction for transferring some $NEARs asyncly to the predecessor account which is the last caller as the receiver_id's contract actor and where the scheduled promise must be executed in
}


pub fn refund_approve_account_ids(account_id: AccountId, approved_account_ids: &HashMap<AccountId, u64>) -> Promise{ //-- the approved_account_ids hashmap must be in its borrowed type to avoid ownership moving
    refund_approved_account_ids_iter(account_id, approved_account_ids.keys()) //-- we're refunding the owner for releasing the storage used up by the approved_account_ids when transferring the NFT - we've passed the total keys of the approved_account_ids hashmap to iterate over them so we can map them 
}


pub fn royalty_to_payout(royalty_percentage: u32, amount_to_pay_for_nft: Balance) -> U128{ //-- calculating the total payout by convert the royalty percentage and amount to pay into a payout of type U128
    
    /*

        ----------------------------------------------------------------------------------------------------------
        [?] we can't receive floating numbers from the front-end due to the fact that different cpu architectures 
            handle floating numbers differently thus we have to multiply the royalty percentage in front-ent by
            a specific value and that value would be 10_000.
        [?] we gave 100 % a value of 10000 to keep track of all perpetual royalties in a u32 type 
        [?] amount_to_pay_for_nft is the amount that the buyer has paid for the NFT or the seller must get for his/her NFT.
            here calculate the total payout that the owner must get paid using the amount of the 
            sold nft and his/her royalty percentage and in order to allow for percentage less 
            than 1 % we must give 100 % a value of 10_000 this means that the percentage for 
            the minimum value of 1 would be 0.01 % cause : 
                        100  %              =        10_000
                        0.01 %              =        X ------> is the value of 0.01 %
                        X = 10_000 * 0.01 / 100 = 1
        [?] payout = royalty percentage value * amount to pay / value of 100 %
        ----------------------------------------------------------------------------------------------------------

    */
    
    U128(royalty_percentage as u128 * amount_to_pay_for_nft / 10_000u128) //-- converting the percentage to the actual amount as u128 that should be paid by multiplying the percentage by the result of dividing the given amount_to_pay_for_nft or balance by 10_000 (10_000 is the value of the 100 % ) that should be paid
}


pub fn asNEAR(amount: u128) -> String{
    format!("{}", amount / ONE_NEAR)
}


pub fn toYocto<D: Into<u128>>(amount: D) -> u128{
    ONE_NEAR * amount.into()
}


#[derive(Serialize, Deserialize)]
#[serde(crate="near_sdk::serde")] //-- must be added right down below of the serde derive proc macro attributes - loading serde crate instance from near_sdk crate using the #[serde()] proc macro attribute itself
pub struct SaleArgs {
    pub sale_conditions: U128, //-- the price of the NFT in u128 type in yocto Ⓝ (1e24)
}


#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate="near_sdk::serde")] //-- must be added right down below of the serde derive proc macro attributes - loading serde crate instance from near_sdk crate using the #[serde()] proc macro attribute itself
pub struct NFTOwnerInfo{
    pub account_id: AccountId,
    pub owned_type: OwnedType,
}


#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate="near_sdk::serde")] //-- must be added right down below of the serde derive proc macro attributes - loading serde crate instance from near_sdk crate using the #[serde()] proc macro attribute itself
pub enum OwnedType{
    Mint, //-- the owner owned this NFT on minting it
    MarketSale, //-- the owner owned this NFT by saling it 
    Bid, //-- the owner owned this NFT on auction or offer bids 
    AcceptOffer,
    GiveAway, //-- the owner owned this NFT on give away
}




#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate="near_sdk::serde")] //-- must be added right down below of the serde derive proc macro attributes - loading serde crate instance from near_sdk crate using the #[serde()] proc macro attribute itself
pub struct NftRevealInfo{
    pub token_id: TokenId,
    pub metadata: TokenMetadata,
}




#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate="near_sdk::serde")] //-- must be added right down below of the serde derive proc macro attributes - loading serde crate instance from near_sdk crate using the #[serde()] proc macro attribute itself
pub struct NftMintInfo{
    pub token_id: TokenId,
    pub metadata: TokenMetadata,
    pub receiver_id: AccountId,
    pub price: Option<U128>,
    pub creator_id: AccountId,
    pub perpetual_royalties: Option<HashMap<AccountId, u32>>
}





// ------------------------------ data collision prevention structures 
// -------------------------------------------------------------------------------------------------
// -------------------------------------------------------------------------------------------------
// -------------------------------------------------------------------------------------------------
/*
    ---------------------------------------------------------------------------------------------------------------------------------------------------------------------


         ===========================
        | Data Collision Explanation 
         ===========================
        | when initializing a data structure make sure to give it a unique id, otherwise, it could point to other structure's key-value references;
        | so we need a unique indentifire key for each object from near collections, if two near collections share the same key, they share the 
        | same data irregardless of whether it'll work or fail if you share memory between two different objects, like Vector and LookupMap.
        | If we use the same storage key, it will lead to error, complaining that: you already have an entry with the <KEY_NAME> stored in the collection, 
        | and we are repeatingly storage another value to that key, and this is not possible, we also want them to be independent of each other; 
        | and same storage key used by 2 different HashMap means they share the same values, using same key for a new entry that has already in the 
        | map face us data collision issue means two entries with same key share a smae memory location for their entry data, to solve this issue we 
        | have to use near collections cause they'll use the memory location address of an enum variant (the current one) as the storage key 
        | to avoid collection collision, since the hash of the address of the current variant in enum always is unique since we'll use their utf8 encoded version
        | thus we can have two keys with the same name but different values and different hashes in two different collections at the same time. 
        |
        |
        |
        |
        |
        |
        |


        

        
        collection 1 keys : {1: "value64", 2: "value53", 3: "value24"}
        collection 2 keys : {1: "anether", 2: "anither", 3: "another"}
        
        above collections will be collided with each other inside the memory since they share the same storage for their keys and have same keys
        to fix this we can allocate a unique storage key for each collection like using that binding that key for each entry that comes into the collection
        and that unique storage key must be built from a utf8 bytes encoded unique indentifire like an enum variant:
        
        #[derive(BorshSerialize, BorshDeserialize)]
        pub enum CollectStorageKey{
            CollectionOne,
            CollectionTwo,
        }

        collection 1 storage key : 0 ---- built from the utf8 bytes encoded CollectionOne enum variant (CollectStorageKey::CollectionOne.try_to_vec().unwrap())
        collection 2 storage key : 1 ---- built from the utf8 bytes encoded CollectionTwo enum variant (CollectStorageKey::CollectionTwo.try_to_vec().unwrap())
        
        collection 1 keys : {1: "value64", 2: "value53", 3: "value24"} -> put all the keys inside the created storage key for the first collection like: {0: [1, 2, 3]} or as a unique prefix for the keys: {01: "value64", 02: "value53", 03: "value24"}
        collection 2 keys : {1: "anether", 2: "anither", 3: "another"} -> put all the keys inside the created storage key for the second collection like: {1: [1, 2, 3]} or as a unique prefix for the keys: {11: "anether", 12: "anither", 13: "another"}





        NOTE - by setting a unique storage key for each collection actually we're putting all the keys and entries of that collection inside a unique storage in memory which has a unique key or flag to avoid data collision for each collection's keys
        NOTE - since two different collections might have same key we'll set a prefix key for each collection using enum variant serialized to utf8 to avoid collection collision with same key in their entries, by doing this every collection will have a unique identifier and will be separated from other collection in which a same version of a key exists
        NOTE - every instascne of TokenPerOwnerInner and TokensPerTypeInner will have a new memory location address thus we can use it as an storage key since the hash of this key will be different and unique each time due to different memory location address of each instacne which won't be the same even if we create a new instance with a same field each time
        NOTE - enum has an extra size like 8 bytes, a 64 bits pointer which is big enough to store the current vairant address for its tag which tells use which variant we have right now, but rust uses null pointer optimization instead of allocating 8 bytes tag  
        NOTE - null pointer optimization means a reference can never be null such as Option<&T> which is a pinter with 8 bytes length thus rust uses that reference or pointer as the tag with 8 bytes length for the current variant  
        NOTE - none struct variants in Storagekey enum allocates zero byte for the current persistent storage once the tag point to their address at a time
        NOTE - the enum size with zero byte for each variants would be the largest size of its variant + 8 bytes tag which would be 8 bytes in overall
        NOTE - an enum is the size of the maximum of its variants plus a discriminant value to know which variant it is, rounded up to be efficiently aligned, the alignment depends on the platform
        NOTE - an enum size is equals to a variant with largest size + 8 bytes tag
        NOTE - enum size with a single f64 type variant would be 8 bytes and with four f64 variants would be 16 bytes cause one 8 bytes (the tag) wouldn't be enough because there would be no room for the tag
        NOTE - the size of the following enum is 24 (is equals to its largest variant size which belongs to the Text variant) + 8 (the tag size) bytes 
        
        pub enum UserID {
            Number(u64),
            Text(String),
        }
        

    ---------------------------------------------------------------------------------------------------------------------------------------------------------------------
*/
#[derive(BorshSerialize)] // NOTE - since UnorderedMap, LookupMap and UnorderedSet each one takes a vector of u8 as their key_prefix argument we have to bound the Storagekey enum to BorshSerialize trait to convert each variant into a vector of u8 using try_to_vec() method of the BorshSerialize trait - all collections (i.e. Vector, Map, Tree, etc) have an unique id which is called the storage key and can be either an encoded enum variant or an encoded string 
// -> we've used an enum based storage key for better memory efficiency and avoiding data collision to keeps track of the persistent storage taken by the current collection (one of the following variant). 
// -> data collision could happen by UnorderedMap, LookupMap or UnorderedSet since these hashmap based structure generate a hash from their keys. 
// -> in order not to have a duplicate key entry inside hashmap based structures we can use enum to avoid having some hash collision with two distinct keys.
// -> with enum we can be sure that there will be only one collection (one of the following variant) at a time inside the storage that has been pointed by the enum tag.
// -> hash of the account_id inside the TokensPer* structs is the unique key to use it as the prefix for creating the UnorderedSet to avoid data collision cause every account_id has a unique hash with 256 bits long
pub enum Storagekey{ //-- defining an enum based unique storage key for every near collection to avoid collection collision which might be happened when two different collections share a same storage for their keys on the chain which will face us data collision at runtime
    TokensPerOwner, ////////---------➔ converting this to vector (Storagekey::TokensPerOwner.try_to_vec().unwrap()) gives us an array of [0] which is the utf8 bytes encoded version of the current variant (the offset in memory) that can be used as a unique storage key for the collection prefix key 
    OwnersPerToken, ////////---------➔ converting this to vector (Storagekey::TokensPerOwner.try_to_vec().unwrap()) gives us an array of [1] which is the utf8 bytes encoded version of the current variant (the offset in memory) that can be used as a unique storage key for the collection prefix key
    TokenPerOwnerInner{ account_id_hash: CryptoHash }, //-- 32 bytes or 256 bits (cause it's an array of 32 elements of type u8 which is 32 elements with 1 byte size) of the hash which will be 64 chars in hex which is the account_id length
    TokensById, ////////---------➔ converting this to vector (Storagekey::TokensById.try_to_vec().unwrap()) gives us an array of [3] which is the utf8 bytes encoded version of the current variant (the offset in memory) that can be used as a unique storage key for the collection prefix key
    TokenMetadataById, ////////---------➔ converting this to vector (Storagekey::TokenMetadataById.try_to_vec().unwrap()) gives us an array of [4] which is the utf8 bytes encoded version of the current variant (the offset in memory) that can be used as a unique storage key for the collection prefix key
    NFTContractMetadata, ////////---------➔ converting this to vector (Storagekey::NFTContractMetadata.try_to_vec().unwrap()) gives us an array of [5] which is the utf8 bytes encoded version of the current variant (the offset in memory) that can be used as a unique storage key for the collection prefix key
    TokensPerType, ////////---------➔ converting this to vector (Storagekey::TokensPerType.try_to_vec().unwrap()) gives us an array of [6] which is the utf8 bytes encoded version of the current variant (the offset in memory) that can be used as a unique storage key for the collection prefix key
    TokensPerTypeInner{ token_type_hash: CryptoHash }, //-- 32 bytes or 256 bits (cause it's an array of 32 elements of type u8 which is 32 elements with 1 byte size) of the hash which will be 64 chars in hex which is the account_id length
    TokenTypesLocked, ////////---------➔ converting this to vector (Storagekey::TokenTypesLocked.try_to_vec().unwrap()) gives us an array of [8] which is the utf8 bytes encoded version of the current variant (the offset in memory) that can be used as a unique storage key for the collection prefix key
}