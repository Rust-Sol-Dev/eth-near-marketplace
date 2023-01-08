

## Notes

> Currently the NFT contract is deployed on `nft15.smarties.testnet`

> Currently the market contract is deployed on `market14.smarties.testnet`

> Currently the event contract is deployed on `event3.smarties.testnet`

> Currently the proposal contract is deployed on `proposal3.smarties.testnet`

## Setup 

> ```sudo chown -R root:root . && sudo chmod 777 -R .```

> ```sudo cp  /home/wildonion/.near-credentials/ /root/```

## References

* https://docs.near.org/tutorials/nfts/marketplace

## Smarties Marketplace Method Call Orders Example on Testnet

> See the `test.sh` of the NFT and market contracts for other methods (like views) and their params.

### Contract State Initialization

```console
near call market.smarties.testnet new '{"owner_id": "market.smarties.testnet"}' --accountId market.smarties.testnet
near call nft.smarties.testnet new_default_meta '{"owner_id": "nft.smarties.testnet"}' --accountId nft.smarties.testnet
near call event.smarties.testnet new '{"owner_id": "event.smarties.testnet"}' --accountId event.smarties.testnet
near call proposal.smarties.testnet new '{"owner_id": "proposal.smarties.testnet"}' --accountId proposal.smarties.testnet
```

### NFT Calls

> `init` methods can only be called once and contracts will panic on second call.

#### Mint For Everyone 

> Here the collection creator is `smarties.testnet` and the minter is `wome.testnet`

```console
near call nft.smarties.testnet nft_mint '{"nfts": [{"token_id": "7346783653746", "metadata": {"title": "gen minting", "description": "a minting from wome", "media": "https://bafybeiftczwrtyr3k7a2k4vutd3amkwsmaqyhrdzlhvpt33dyjivufqusq.ipfs.dweb.link/goteam-gif.gif"}, "receiver_id": "wome.testnet", "price": "1", "creator_id": "smarties.testnet", "perpetual_royalties": {"market.smarties.testnet": 2000, "smarties.testnet": 100}}, {"token_id": "7346783653746", "metadata": {"title": "gen minting", "description": "a minting from wome", "media": "https://bafybeiftczwrtyr3k7a2k4vutd3amkwsmaqyhrdzlhvpt33dyjivufqusq.ipfs.dweb.link/goteam-gif.gif"}, "receiver_id": "wome.testnet", "price": "1", "creator_id": "smarties.testnet", "perpetual_royalties": {"market.smarties.testnet": 2000, "smarties.testnet": 100}} ]}' --accountId wome.testnet --amount 2
```

#### Mint For Creator 

> Here the collection creator and the minter is `smarties.testnet`

```console
near call nft.smarties.testnet nft_creator_mint '{"nfts": [{"token_id": "7346783653746", "metadata": {"title": "gen minting", "description": "a minting from wome", "media": "https://bafybeiftczwrtyr3k7a2k4vutd3amkwsmaqyhrdzlhvpt33dyjivufqusq.ipfs.dweb.link/goteam-gif.gif"}, "receiver_id": "wome.testnet", "creator_id": "smarties.testnet", "perpetual_royalties": {"market.smarties.testnet": 2000, "smarties.testnet": 100} }}, {"token_id": "7346783653746", "metadata": {"title": "gen minting", "description": "a minting from wome", "media": "https://bafybeiftczwrtyr3k7a2k4vutd3amkwsmaqyhrdzlhvpt33dyjivufqusq.ipfs.dweb.link/goteam-gif.gif"}, "receiver_id": "wome.testnet", "creator_id": "smarties.testnet", "perpetual_royalties": {"market.smarties.testnet": 2000, "smarties.testnet": 100}} ]}' --accountId wome.testnet --amount 2
```

#### Reveal NFT

> Since this method must be called from a backend thus only the contract owner which is `nft.smarties.testnet` in our case can call this method to reveal a specific NFT and therefore we've bounded this method to `#[private]` attribute

```console
near call nft.smarties.testnet nft_reveal '{"nfts": [{"token_id": "7346783653746", "metadata": {"title": "gen minting reveal time", "description": "reveal this", "media": "https://bafybeiftczwrtyr3k7a2k4vutd3amkwsmaqyhrdzlhvpt33dyjivufqusq.ipfs.dweb.link/goteam-gif.gif"}}, {"token_id": "7346783653747", "metadata": {"title": "gen minting reveal time", "description": "reveal this", "media": "https://bafybeiftczwrtyr3k7a2k4vutd3amkwsmaqyhrdzlhvpt33dyjivufqusq.ipfs.dweb.link/goteam-gif.gif"}}], "collection_creator_id": "smarties.testnet"}' --accountId nft.smarties.testnet
```

#### Update NFT

```console
near call nft.smarties.testnet nft_update '{"token_id": "7346783653746", "metadata": {"title": "update nft info", "description": "a new update info", "media": "https://bafybeiftczwrtyr3k7a2k4vutd3amkwsmaqyhrdzlhvpt33dyjivufqusq.ipfs.dweb.link/goteam-gif.gif"}, "perpetual_royalties": {"market.smarties.testnet": 3000, "wildonion.testnet": 100} }' --accountId wome.testnet --depositYocto 1
```

### Marketplace Calls

> To pass a `U128` type we have to put the value inside "" which is in form of a string like `from_index` param

> To see all the events of a specific NFT inside a specific contract call ```near view nft.smarties.testnet nft_events '{"token_id": "7346783653746", "from_index": "0", "limit": 50}'```

> To see all the owners of a specific NFT inside a specific contract call ```near view nft.smarties.testnet nft_owners '{"token_id": "7346783653746", "from_index": "0", "limit": 50}'```

> To see all sale objects of a specific NFT owner call ```near view market.smarties.testnet get_sales_by_owner_id '{"account_id": "wome.testnet", "from_index": "0", "limit": "10"}'```

> To see the approval id of the marketplace call ```near view nft.smarties.testnet nft_token '{"token_id": "7346783653746"}'```

> To see all the NFT for a specific owner call ```near view nft.smarties.testnet nft_tokens_for_owner '{"account_id": "wome.testnet", "limit": 5}'```

> Signer of the following method is `wome.testnet` and the NFT contract owner is `nft.smarties.testnet`

> We can use `--depositYocto` flag with value 10000000000000000000000 instead of `--deposit` with value 0.01

> To see the sale object of the NFT call ```near view market.smarties.testnet get_sale '{"nft_contract_token_id": "nft.smarties.testnet.7346783653746"}'```

> To add offer on an NFT first the caller must deposit some NEAR 

> The deposit amount in `add_offer` method must be exactly equals to the price of the NFT offer

> Only the one who put the offer on NFT can call `delete_offer` method to cancel the offer

> `price` param in starting auction is the starting bid price

#### Storage Deposit For Listing, Revealing, Start Auction and Add NFTs Offer on the Marketplace 

> The market storage cost will be handled automatically using the attaching deposit in calling the `nft_approve` method 

```console
near call market.smarties.testnet storage_deposit '{"account_id": "wome.testnet"}' --accountId wome.testnet --deposit 2
```

#### Storage Withdraw from the Marketplace

> To see all the deposited storages for a given specific account call ```near view market.smarties.testnet storage_balance_of '{"account_id": "wome.testnet"}'```

```console
near call market.smarties.testnet storage_withdraw --accountId wome.testnet --deposit 2
```

#### Sell NFT

```console
near call nft.smarties.testnet nft_approve '{"token_id": "7346783653746", "account_id": "market.smarties.testnet", "msg": "{ \"market_type\": \"sale\", \"price\": \"5000000000000000000000000\" }" }' --accountId wome.testnet --deposit 0.01 --gas 200000000000000
```

#### Add NFT Offer

```console
near call market.smarties.testnet add_offer '{"nft_contract_id": "nft.smarties.testnet", "token_id": "7346783653746", "price": "5000000000000000000000000"}' --accountId new-mlk.testnet --depositYocto 5000000000000000000000000
```

#### Delete NFT Offer

```console
near call market.smarties.testnet delete_offer '{"nft_contract_id": "nft.smarties.testnet", "token_id": "7346783653746"}' --accountId new-mlk.testnet --depositYocto 1
```

#### Get NFT Offer

```console
near call market.smarties.testnet get_offer '{"nft_contract_id": "nft.smarties.testnet", "buyer_id": "new-mlk.testnet", "tokne_id": "7346783653746"}' --accountId wome.testnet
```

#### Accept NFT Offer

```console
near call nft.smarties.testnet nft_approve '{"token_id": "7346783653746", "account_id": "market.smarties.testnet", "msg": "{ \"market_type\": \"accept_offer\", \"buyer_id\": \"new-mlk.testnet\", \"price\": \"5000000000000000000000000\" }" }' --accountId wome.testnet --deposit 0.01 --gas 200000000000000
```

#### Start Auction

```console
near call nft.smarties.testnet nft_approve '{"token_id": "7346783653746", "account_id": "market.smarties.testnet", "msg": "{ \"market_type\": \"sale\", \"price\": \"5000000000000000000000000\", \"ended_at\": \"166307160000000000\", \"is_auction\": true }" }' --accountId wome.testnet --deposit 0.01 --gas 200000000000000
```

#### Add Auction Bid

```console
near call market.smarties.testnet add_bid '{"nft_contract_id": "nft.smarties.testnet", "token_id": "7346783653746", "amount": "6000000000000000000000000"}' --accountId wome.testnet --depositYocto 6000000000000000000000000
```

#### Cancel Auction Bid

```console
near call market.smarties.testnet cancel_bid '{"nft_contract_id": "nft.smarties.testnet", "token_id": "7346783653746", "account_id": "wome.testnet"}' --accountId wome.testnet --depositYocto 1
```

#### Accept Auction Bid

```console
near call market.smarties.testnet accept_bid '{"nft_contract_id": "nft.smarties.testnet", "token_id": "7346783653746"}' --accountId wome.testnet --depositYocto 1
```

#### End Auction

```console
near call market.smarties.testnet end_auction '{"nft_contract_id": "nft.smarties.testnet", "token_id": "7346783653746"}' --accountId wome.testnet --depositYocto 1 --gas 200000000000000
```

#### Purchasing NFT

> The `memo` param can be either "sale" or "bid" on market sell or buying on auction or offer process.   

```console
near call market.smarties.testnet buy '{"nft_contract_id": "nft.smarties.testnet", "token_id": "7346783653746", "memo": "sale"}' --accountId mlkk.testnet --deposit 3 --gas 200000000000000
```

## Test Accounts

> All testnet keys are inside `testnet` folder.

> `smarties.testnet` with private key: ```style buddy convince globe need mushroom used raw advice upon cram flight```

> `wome.testnet` with priavte key: ```clean nose diagram install paper round second item deal size marriage year```

> Market contract is on `market.smarties.testnet`

> Market NFT contract is on `nft.smarties.testnet` 

> Market proposal contract is on `proposal.smarties.testnet` 

> Market event contract is on `event.smarties.testnet` 


```console
near delete market.smarties.testnet smarties.testnet
near delete nft.smarties.testnet smarties.testnet
near delete event.smarties.testnet smarties.testnet
near delete proposal.smarties.testnet smarties.testnet
near create-account market.smarties.testnet --masterAccount smarties.testnet --initialBalance 25
near create-account nft.smarties.testnet --masterAccount smarties.testnet --initialBalance 25
near create-account proposal.smarties.testnet --masterAccount smarties.testnet --initialBalance 25
near create-account event.smarties.testnet --masterAccount smarties.testnet --initialBalance 25
NEAR_ENV=testnet near deploy --wasmFile market/out/market.wasm --accountId market.smarties.testnet
NEAR_ENV=testnet near deploy --wasmFile nft/out/nft.wasm --accountId nft.smarties.testnet
NEAR_ENV=testnet near deploy --wasmFile proposal/out/proposal.wasm --accountId proposal.smarties.testnet
NEAR_ENV=testnet near deploy --wasmFile event/out/event.wasm --accountId event.smarties.testnet
```

### Deploy on Master Accounts

> We can only have one smart contract per each account.

```console
NEAR_ENV=testnet near deploy --wasmFile nft/out/nft.wasm --accountId wome.testnet
NEAR_ENV=testnet near deploy --wasmFile market/out/market.wasm --accountId smarties.testnet
```

## Event Contract Methods Doc 

> `create`, `expire_event`, `vote` and `lock_event` are payable methods.

```rust
fn create(&mut self, event_id: EventId, event: Event) -> Option<Event>;
fn get_all_events_by_collection(&self, collection_name: String) -> Vec<Option<Event>>;
fn expire_event(&mut self, event_id: EventId) -> Option<Event>;
fn vote(&mut self, event_id: EventId, voter: Participant) -> Option<Event>;
fn lock_event(&mut self, collection_creator_id: AccountId, event_id: EventId) -> Option<Event>;


pub struct Event{
    pub collection_name: CollectionName,
    pub collection_creator_id: AccountId,
    pub title: String, 
    pub content: String,
    pub participants: Vec<Participant>,
    pub is_expired: bool,
    pub is_locked: bool,
    pub expire_at: u64, //-- nano sec
    pub created_at: u64, //-- nano sec
    
}
```

## Proposal Contract Methods Doc 

> `create`, `expire_proposal`, `vote` and `lock_proposal` are payable methods.

```rust
fn create(&mut self, proposal_id: ProposalId, proposal: Proposal) -> Option<Proposal>;
fn get_all_proposals_by_collection(&self, collection_name: String) -> Vec<Option<Proposal>>;
fn expire_proposal(&mut self, proposal_id: ProposalId) -> Option<Proposal>;
fn vote(&mut self, proposal_id: ProposalId, voter: Voter) -> Option<Proposal>;
fn lock_proposal(&mut self, collection_creator_id: AccountId, proposal_id: ProposalId) -> Option<Proposal>;



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
    pub expire_at: u64, //-- nano sec
    pub created_at: u64, //-- nano sec
    
}
```


