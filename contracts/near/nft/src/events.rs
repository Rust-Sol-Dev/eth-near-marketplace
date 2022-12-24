





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








// NOTE - all price fields are in Ⓝ
// NOTE - we can use #[serde(flatten)] attribute on a field of a struct or enum in those cases that we don't know about the exact keys or values inside the flattened field thus we can use this attribute to hold additional data that is not captured by any other fields of the struct or enum
// NOTE - since we don't know what's exactly inside the data of an event (cause it's an array of json) we have to flatten the event field inside the EventLog struct to only have the content of the current variant of EventLogVariant enum since this enum is tagged 
// NOTE - #[serde(flatten)] proc macro attribute can be used for factoring common keys into a shared structure, or for capturing remaining fields into a map with arbitrary string keys















#[derive(Serialize, Deserialize, Debug, BorshDeserialize, BorshSerialize)]
#[serde(tag="event", content="data")] // NOTE - the deserialized data of the following enum  will be : {"event": "nft_mint", "data": [{...NftMintLog_instance...}, {...NftMintLog_instance...}]} or {"event": "nft_transfer", "data": [{...NftTransferLog_instance...}, {...NftTransferLog_instance...}]}
#[serde(rename_all="snake_case")] //-- converting all fields' name to snake_case format like nft_ming_log
#[serde(crate="near_sdk::serde")] //-- must be added right down below of the serde derive proc macro attributes - loading serde crate instance from near_sdk crate using the #[serde()] proc macro attribute itself
#[non_exhaustive] // NOTE - this attribute indicates that more variants/fields may be added to an enum/struct in the future and prevents downstream crates from exhaustively listing out all variants/fields
pub enum EventLogVariant{ //-- event log enum which can be either NFT mint or NFT transfer log 
    NftMint(Vec<NftMintLog>), //-- vector of all minting NFT events
    NftTransfer(Vec<NftTransferLog>), //-- vector of all transferring NFT events; on firing nft_transfer() method which is useful for airdrops and giveaways
    NftList(Vec<NftListLog>),
    NftUpdate(Vec<NftUpdateLog>),
    NftReveal(Vec<NftRevealLog>),
}




#[derive(Serialize, Deserialize, Debug, BorshDeserialize, BorshSerialize)]
#[serde(crate="near_sdk::serde")] //-- must be added right down below of the serde derive proc macro attributes - loading serde crate instance from near_sdk crate using the #[serde()] proc macro attribute itself
pub struct EventLog{ //-- an interface to capture the data about and event - this is the EVENT_JSON 
    pub standard: String,
    pub version: String,
    #[serde(flatten)] //-- flatten to not have "event": {<EventLogVariant>} in the JSON, just have the contents of {<EventLogVariant>} which is the value of the data key itself - we can use #[serde(flatten)] attribute on a field of a struct or enum in those cases that we don't know about the number of exact fields inside the struct or enum or what's exactly inside the body of an api comming from the client to decode or map it into the struct or enum thus we can use this attribute to hold additional data that is not captured by any other fields of the struct or enum
    pub event: EventLogVariant, //-- the data which is a vector of all either NftMint or NftTransfer variant events - we'll have {"standard": "1", "version": "1", "event": "event name", "data": [{...NftMintLog_instance...}] or [{...NftTransferLog_instance...}]}
}



impl fmt::Display for EventLog{ //-- implementing the Display trait for the EventLog struct to show its instances' fields like EVENT_JSON:{"time": 167836438974, "event": "event name, "data": [{...RuntimeLog_instance...}] or [{...ServerlessLog_instance...}]} when we're calling logging functions like println!() which is a formatted stream of strings - any value or type that implements the Display trait can be passed to format_args!() macro, as can any Debug implementation be passed to a {:?} within the formatting string; Debug must be implemented for the type
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        f.write_fmt( //-- writing some formatted information using format_args!() macro into the formatter instance which is `f`
            format_args!( //-- format_args!(), unlike its derived macros, avoids heap allocations
                "EVENT_JSON:{}", //-- it'll start with EVENT_JSON:{}
                &serde_json::to_string(self).map_err(|_| fmt::Error).unwrap() //-- formatting every field of the self which is the instance of the EventLog struct into the json string to writ into the `f` and catch the fmt::error of each message or field if there was any when we're creating the stream by formatting the struct 
            ) 
        ) // NOTE - we can print the string instance of the EventLog like so: println!("{:?}", event_log_instance.to_string()); since the Display trait is implemented for EventLog struct
    }
}



#[derive(Serialize, Deserialize, Debug, BorshDeserialize, BorshSerialize)] 
#[serde(crate="near_sdk::serde")] //-- must be added right down below of the serde derive proc macro attributes - loading serde crate instance from near_sdk crate using the #[serde()] proc macro attribute itself
pub struct NftMintLog{ //-- event log to capture token minting
    pub owner_id: AccountId, //-- NFT owner_id might be collection creator or other account as the minter
    pub token_ids: Vec<TokenId>, //-- it might be a collection minting process!
    pub price: u128,
    pub done_at: u64, //-- the current block timestamp
    #[serde(skip_serializing_if="Option::is_none")] //-- skip serializing this field if it was None
    pub memo: Option<String>,
}



#[derive(Serialize, Deserialize, Debug, BorshDeserialize, BorshSerialize)] 
#[serde(crate="near_sdk::serde")] //-- must be added right down below of the serde derive proc macro attributes - loading serde crate instance from near_sdk crate using the #[serde()] proc macro attribute itself
pub struct NftRevealLog{ //-- event log to capture token minting
    pub collection_creator_id: AccountId, //-- NFT collection creator_id
    pub owner_id: AccountId, //-- NFT owner_id might be collection creator or other account as the minter
    pub token_ids: Vec<TokenId>, //-- it might be a collection minting process!
    pub done_at: u64, //-- the current block timestamp
    #[serde(skip_serializing_if="Option::is_none")] //-- skip serializing this field if it was None
    pub memo: Option<String>,
}



#[derive(Serialize, Deserialize, Debug, BorshDeserialize, BorshSerialize)] 
#[serde(crate="near_sdk::serde")] //-- must be added right down below of the serde derive proc macro attributes - loading serde crate instance from near_sdk crate using the #[serde()] proc macro attribute itself
pub struct NftUpdateLog{ //-- event log to capture token minting
    pub owner_id: AccountId, //-- NFT owner_id might be collection creator or other account as the minter
    pub token_ids: Vec<TokenId>, //-- it might be a collection minting process!
    pub done_at: u64, //-- the current block timestamp
    #[serde(skip_serializing_if="Option::is_none")] //-- skip serializing this field if it was None
    pub memo: Option<String>,
}



#[derive(Serialize, Deserialize, Debug, BorshDeserialize, BorshSerialize)]
#[serde(crate="near_sdk::serde")] //-- must be added right down below of the serde derive proc macro attributes - loading serde crate instance from near_sdk crate using the #[serde()] proc macro attribute itself
pub struct NftTransferLog{ //-- event log to capture token transfer
    #[serde(skip_serializing_if="Option::is_none")] //-- skip serializing this field if it was None
    pub authorized_id: Option<AccountId>, //-- if there was any approved account_id to transfer the NFT on behalf of the owner like the sender itself when we're calling nft_transfer() for selling the NFT method from the market contract using the cross contract call
    pub old_owner_id: AccountId,
    pub new_owner_id: AccountId,
    pub token_ids: Vec<TokenId>, //-- it might be a collection transferring process!
    #[serde(skip_serializing_if="Option::is_none")] //-- skip serializing this field if it was None
    pub price: Option<u128>, //-- if there was a price means that this NFT has been given away otherwise it was a sale
    pub done_at: u64, //-- the current block timestamp 
    #[serde(skip_serializing_if="Option::is_none")] //-- skip serializing this field if it was None
    pub memo: Option<String>,
}



#[derive(Serialize, Deserialize, Debug, BorshDeserialize, BorshSerialize)] 
#[serde(crate="near_sdk::serde")] //-- must be added right down below of the serde derive proc macro attributes - loading serde crate instance from near_sdk crate using the #[serde()] proc macro attribute itself
pub struct NftListLog{ //-- event log to capture token listing on the market
    pub authorized_id: AccountId, //-- the account_id that has been approved to list the NFT like the marketplace account_id
    pub owner_id: AccountId, //-- the NFT owner
    pub token_ids: Vec<TokenId>, //-- it might be a collection minting process!
    pub price: u128, //-- NFT price when the owner is listing on the market
    pub done_at: u64, //-- the current block timestamp
    #[serde(skip_serializing_if="Option::is_none")] //-- skip serializing this field if it was None
    pub msg: Option<String>, //-- the sale condition json stringify
}