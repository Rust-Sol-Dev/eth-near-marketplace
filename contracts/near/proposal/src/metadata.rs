



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











#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate="near_sdk::serde")]
pub struct Proposal{
    pub collection_name: CollectionName,
    pub collection_creator_id: AccountId,
    pub title: String, 
    pub content: String,
    pub upvotes: Option<u32>,
    pub downvotes: Option<u32>,
    pub voters: Vec<Voter>,
    pub is_expired: bool,
    pub is_locked: bool,
    pub media: String,
    pub expire_at: u64, //-- nano sec
    pub created_at: u64, //-- nano sec
    
}




#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate="near_sdk::serde")]
pub struct Voter{
    pub nft_owner_id: AccountId,
    pub is_upvote: bool,
    pub score: u32, // NOTE - this is the number of event NFTs that this owner owns
}



#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate="near_sdk::serde")]
pub struct GetProposal{
    pub proposal_id: ProposalId,
    pub proposal: Proposal,
}


#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate="near_sdk::serde")]
pub struct MarketPorposal{
    pub proposal_id: ProposalId,
    pub propsal: Proposal,
}