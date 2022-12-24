







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


use crate::*;  // loading all defined crates, structs and functions from the root crate which is lib.rs in our case














// ------------------------------ internal functions 
// -------------------------------------------------------------------------------------------------
// -------------------------------------------------------------------------------------------------
// -------------------------------------------------------------------------------------------------
pub fn hash_account_id(account_id: &AccountId) -> CryptoHash{ //-- we'll pass the account_id as a borrowed type to this function - account_id in CryptoHash format is a 32 bytes or 256 bits which will be 64 chars in hex
    let mut hash = CryptoHash::default(); //-- getting the default hash which will be 32 elements of utf8 bytes (8 bits or 1 byte long for each)
    hash.copy_from_slice(&env::sha256(account_id.as_bytes())); //-- extending the defined hash with the borrowed type of the bytes of the hash of the account_id by converting its String into utf8 bytes first; the source or the length of the hash of the account_id bytes must be the same as the defined hash variable 
    hash
}











#[derive(Serialize, Deserialize)]
#[serde(crate="near_sdk::serde")] //-- must be added right down below of the serde derive proc macro attributes - loading serde crate instance from near_sdk crate using the #[serde()] proc macro attribute itself
pub struct SaleArgs {
    pub sale_conditions: SalePriceInYoctoNear, //-- the price of the NFT in u128 type in yocto Ⓝ (1e24)
}











// NOTE - we'll use this structure to deserialize the result of the executed nft_transfer_payout() promise or future object
#[derive(Serialize, Deserialize)]
#[serde(crate="near_sdk::serde")] //-- must be added right down below of the serde derive proc macro attributes - loading serde crate instance from near_sdk crate using the #[serde()] proc macro attribute itself
pub struct Payout{ //-- payout type for the royalty standards which specifies which account_id must get paid how much per each sell of a specific NFT
    pub payout: HashMap<AccountId, U128>, // NOTE - HashMap has loaded inside the lib.rs before and we imported using use crete::* syntax 
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








#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Bid{
    pub bidder_id: AccountId,
    pub price: U128,
}






#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate="near_sdk::serde")]
pub struct OfferData{
    pub buyer_id: AccountId,
    pub nft_contract_id: AccountId,
    pub token_id: TokenId,
    pub price: u128,
}




#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate="near_sdk::serde")]
pub struct JsonOfferData{
    pub buyer_id: AccountId,
    pub nft_contract_id: AccountId,
    pub token_id: TokenId,
    pub price: U128,
}




#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate="near_sdk::serde")]
pub struct MarketData{ //-- this struct will be instantiated on each NFT buying and start auction processes
    pub owner_id: AccountId,
    pub approval_id: u64,
    pub nft_contract_id: AccountId,
    pub token_id: TokenId,
    pub price: u128,
    pub bids: Option<Bids>,
    pub started_at: Option<u64>,
    pub ended_at: Option<u64>,
    pub end_price: Option<u128>, //-- highest price
    pub accept_nft_contract_id: Option<String>,
    pub accept_token_id: Option<String>,
    pub is_auction: Option<bool>, //-- whether this data on an auction or not
}


#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate="near_sdk::serde")]
pub struct JsonMarketData{ //-- this struct will be instantiated on each NFT buying and start auction processes
    pub owner_id: AccountId,
    pub approval_id: U64,
    pub nft_contract_id: AccountId,
    pub token_id: TokenId,
    pub price: U128,
    pub bids: Option<Bids>,
    pub started_at: Option<U64>,
    pub ended_at: Option<U64>,
    pub end_price: Option<U128>,
    pub is_auction: Option<bool>,
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
        NOTE - every instascne of ByOwnerIdInner, ByNFTContractIdInner and ByNFTTokenTypeInner will have a new memory location address thus we can use it as an storage key since the hash of this key will be different and unique each time due to different memory location address of each instacne which won't be the same even if we create a new instance with a same field each time
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
    Sales, ////////---------➔ converting this to vector (Storagekey::Sales.try_to_vec().unwrap()) gives us an array of [0] which is the utf8 bytes encoded version of the current variant (the offset in memory) that can be used as a unique storage key for the collection prefix key 
    ByOwnerId, ////////---------➔ converting this to vector (Storagekey::ByOwnerId.try_to_vec().unwrap()) gives us an array of [1] which is the utf8 bytes encoded version of the current variant (the offset in memory) that can be used as a unique storage key for the collection prefix key
    ByOwnerIdInner { account_id_hash: CryptoHash }, //-- 32 bytes or 256 bits (cause it's an array of 32 elements of type u8 which is 32 elements with 1 byte size) of the hash which will be 64 chars in hex which is the account_id length; use this to cover the prefix of the collection storage key based on a struct which contains the hash of the account_id
    ByNFTContractId, ////////---------➔ converting this to vector (Storagekey::ByNFTContractId.try_to_vec().unwrap()) gives us an array of [3] which is the utf8 bytes encoded version of the current variant (the offset in memory) that can be used as a unique storage key for the collection prefix key
    ByNFTContractIdInner { account_id_hash: CryptoHash }, //-- 32 bytes or 256 bits (cause it's an array of 32 elements of type u8 which is 32 elements with 1 byte size) of the hash which will be 64 chars in hex which is the account_id length; use this to cover the prefix of the collection storage key based on a struct which contains the hash of the account_id
    ByNFTTokenType, ////////---------➔ converting this to vector (Storagekey::ByNFTTokenType.try_to_vec().unwrap()) gives us an array of [5] which is the utf8 bytes encoded version of the current variant (the offset in memory) that can be used as a unique storage key for the collection prefix key
    ByNFTTokenTypeInner { token_type_hash: CryptoHash }, //-- 32 bytes or 256 bits (cause it's an array of 32 elements of type u8 which is 32 elements with 1 byte size) of the hash which will be 64 chars in hex which is the account_id length; use this to cover the prefix of the collection storage key based on a struct which contains the hash of the account_id
    FTTokenIds, ////////---------➔ converting this to vector (Storagekey::FTTokenIds.try_to_vec().unwrap()) gives us an array of [7] which is the utf8 bytes encoded version of the current variant (the offset in memory) that can be used as a unique storage key for the collection prefix key
    StorageDeposits, ////////---------➔ converting this to vector (Storagekey::StorageDeposits.try_to_vec().unwrap()) gives us an array of [8] which is the utf8 bytes encoded version of the current variant (the offset in memory) that can be used as a unique storage key for the collection prefix key
    Collection, ////////---------➔ converting this to vector (Storagekey::Collection.try_to_vec().unwrap()) gives us an array of [9] which is the utf8 bytes encoded version of the current variant (the offset in memory) that can be used as a unique storage key for the collection prefix key
    Offers,
    MrketData,
    OfferByOwnerId,
    SalesByOwnerId
}