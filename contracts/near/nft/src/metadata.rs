




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











#[derive(Serialize, Deserialize)]
#[serde(crate="near_sdk::serde")] //-- must be added right down below of the serde derive proc macro attributes - loading serde crate instance from near_sdk crate using the #[serde()] proc macro attribute itself
pub struct Payout{ //-- payout type for the royalty standards which specifies which account_id must get paid how much per each sell of a specific NFT
    pub payout: HashMap<AccountId, U128>, // NOTE - HashMap has loaded inside the lib.rs before and we imported using use crete::* syntax 
}


#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate="near_sdk::serde")] //-- must be added right down below of the serde derive proc macro attributes - loading serde crate instance from near_sdk crate using the #[serde()] proc macro attribute itself
pub struct NFTContractMetadata{ //-- token metadata info at contract level
    pub spec: String, //-- required, nft contract metadata version
    pub name: String, //-- required, nft contract metadata name
    pub symbol: String, //-- required, nft contract metadata symbol
    pub icon: Option<String>, //-- optional, nft contract metadata icon (cost storage)
    pub base_uri: Option<String>, //-- optional, nft contract metadata url to decentralized storage of the assets referenced by `reference` or `media` url fields
    pub reference: Option<String>, //-- optional, nft contract metadata url to a json file which contains more info about the asset
    pub reference_hash: Option<Base64VecU8>, //-- optional, a base64 string encoded to utf8 version of sha256 hash of the json from the reference field which is in form Vec<u8>; is more like: base64(Vec<u8>(reference_field)) - required if the reference field is required
}


#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate="near_sdk::serde")] //-- must be added right down below of the serde derive proc macro attributes - loading serde crate instance from near_sdk crate using the #[serde()] proc macro attribute itself
pub struct TokenMetadata{ //-- token metadata info at token level itself
    pub title: Option<String>, //-- optional, token metadata title
    pub description: Option<String>, //-- optional, token metadata description
    pub media: Option<String>, //-- optional, token metadata url to decentralized storage of the media content
    pub media_hash: Option<Base64VecU8>, //-- optional, a base64 string encoded to utf8 version of sha256 hash of the media content from the media field which is in form Vec<u8>; is more like: base64(Vec<u8>(media_field)) - required if the media field is required
    pub copies: Option<u64>, //-- optional, number of copies of this set of metadata in existence when token was minted
    pub issued_at: Option<u64>, //-- optional, token metadata unix timestamp of the minted token
    pub expires_at: Option<u64>, //-- optional, token metadata unix timestamp of the minted token
    pub updated_at: Option<u64>, //-- optional, token metadata unix timestamp of the updated time of this token
    pub extra: Option<String>, //-- optional, extra on chain info about the nft and it can be stringified json like an NFT attribute
    pub reference: Option<String>, //-- optional, token metadata url to a json file which contains more info about the asset like the NFT attributes
    pub reference_hash: Option<Base64VecU8>, //-- optional, a base64 string encoded to utf8 version of sha256 hash of the json from the reference field which is in form Vec<u8>; is more like: base64(Vec<u8>(reference_field)) - required if the reference field is required
}


#[derive(BorshDeserialize, BorshSerialize)]
pub struct Token{ //-- contains the owner_id, approved accounts info for selling the token on behalf of the owner and the royalty hashmap for royalty payout
    pub creator_id: AccountId, 
    pub owner_id: AccountId, //-- owner of the token
    pub approved_account_ids: HashMap<AccountId, u64>, //-- a map between all approved account_ids and their approval id to transfer the token on behalf and their unique approval id - we've used hashmap instead of near collection cause we must have only one key or account_id per approval_id
    pub next_approval_id: u64, //-- the next approval id to transfer the token on behalf
    pub royalty: HashMap<AccountId, u32> //-- since perpetual royalties will be on a per-token basis we added this field here - the percentage value in u32 bits or 4 bytes that must be used to calculate the total payout in $NEAR which must be paid by the marketplace to the account_ids (all the NFT owners or charity account_ids must get paid per each sell or transfer, also the old owner which can be the main owner or the minter or creator on second sell must get paid at the end which will have the more payout than the other owners) each time a buyer gets paid for that NFT by calling the nft_transfer_payout() method
}


#[derive(BorshDeserialize, BorshSerialize)]
pub struct CreateToken{ //-- contains the creator_id, approved accounts info for selling the token on behalf of the owner and the royalty hashmap for royalty payout
    pub creator_id: AccountId, //-- creator of the token
    pub approved_account_ids: HashMap<AccountId, u64>, //-- a map between all approved account_ids and their approval id to transfer the token on behalf and their unique approval id - we've used hashmap instead of near collection cause we must have only one key or account_id per approval_id
    pub next_approval_id: u64, //-- the next approval id to transfer the token on behalf
    pub royalty: HashMap<AccountId, u32> //-- since perpetual royalties will be on a per-token basis we added this field here - the percentage value in u32 bits or 4 bytes that must be used to calculate the total payout in $NEAR which must be paid by the marketplace to the account_ids (all the NFT owners or charity account_ids must get paid per each sell or transfer, also the old owner which can be the main owner or the minter or creator on second sell must get paid at the end which will have the more payout than the other owners) each time a buyer gets paid for that NFT by calling the nft_transfer_payout() method
}


#[derive(Serialize, Deserialize)]
#[serde(crate="near_sdk::serde")]
pub struct JsonToken{ //-- the token json info which will be returned from view calls
    pub owner_id: AccountId, //-- the owner of the token
    pub creator_id: AccountId, //-- the creator of the token
    pub token_id: TokenId, //-- the id of the token which is of type String
    pub metadata: TokenMetadata, //-- the metadata of the token - metadata instance of the TokenMetadata struct will be serialized into the utf8 bytes using the serde Serializer 
    pub approved_account_ids: HashMap<AccountId, u64>, //-- a map between all approved account_ids to transfer the token on behalf and their unique approval id - we've used hashmap instead of near collection cause we must have only one key or account_id per approval_id  
    pub royalty: HashMap<AccountId, u32> //-- since perpetual royalties will be on a per-token basis we added this field here - the percentage value in u32 bits or 4 bytes that must be used to calculate the total payout in $NEAR which must be paid by the marketplace to the account_ids (all the NFT owners or charity account_ids must get paid per each sell or transfer, also the old owner which can be the main owner or the minter or creator on second sell must get paid at the end which will have the more payout than the other owners) each time a buyer gets paid for that NFT by calling the nft_transfer_payout() method
}


pub trait NoneFungibleTokenMetadata{ //-- defining an object safe trait for NFT metadata queries, we'll implement this for any contract that wants to interact with NFT metadata queries - object safe traits are not bounded to trait Sized thus they won't return Self or have generic params in its methods if so then some space should have been allocated inside the memory for Self or that generic param and it will no longer an abstract type  
    fn nft_metadata(&self) -> NFTContractMetadata; //-- the return type is of type NFTContractMetadata struct - we should borrow the self (&self) as far as we can
}


#[near_bindgen] //-- implementing the #[near_bindgen] proc macro attribute on the trait implementation for the extended interface (NoneFungibleTokenMetadata trait) of `NFTContract` struct interface in order to have a compiled wasm trait methods for this contract struct so we can call it from the near cli 
impl NoneFungibleTokenMetadata for NFTContract{ //-- implementing the NoneFungibleTokenMetadata trait for our main `NFTContract` struct to extend its interface; bounding the mentioned trait to the `NFTContract` struct to query NFT metadata infos
    fn nft_metadata(&self) -> NFTContractMetadata{ //-- overriding the nft_metadata() method of the NFTContractMetadata trait
        self.metadata.get().unwrap() //-- since metadata field is inside the LazyOption we must get the actual data itself using get() method which will return the type (NFTContractMetadata in our case) inside an Option
    }
}