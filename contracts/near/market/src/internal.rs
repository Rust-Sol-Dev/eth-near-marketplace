



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






// NOTE - sale.owner_id is the one who wants to sell his/her NFT and lists on the market
// NOTE - nft_contract_id is the account_id that the NFT contract is deployed on








// ----------------------------------------
//     CROSS CONTRACT CALLS' INTERFACES
// ----------------------------------------
#[ext_contract(nft_contract)] //-- nft_contract name that we passed in #[ext_contract()] proc macro is the name of the contract (a hypothetical contract name of course) that we're extending its interface for cross contract call and creating transaction which is a promise (future object) ActionReceipt object and means we want to call the following methods inside that contract which contains a transaction which is a promise (future object) ActionReceipt object that must be executed later
trait NftContractReceiver{ //-- this trait which contains the cross conract call methods will extend the interface of the receiver_id's contract actor with a name inside the #[ext_contract()] proc macro which specifies the extended interface contract name on this contract 

    /////
    /////// ➔ following method must be called and executed inside the receiver_id's contract actor account (thus it must be existed and defined on receiver contract actor account) from this contract actor account therefore it'll take a param called account_id which is the one who should call this method on his/her contract actor account and must be the owner of his/her contract
    /////// ➔ receiver_id: purchaser (person to transfer the NFT to) | token_id: the id of the NFT to transfer | approval_id: market contract's approval_id in order to transfer the token on behalf of the owner | memo: memo (to include some context) | balance: the price that the token was purchased for, this will be used in conjunction with the royalty percentages for the token& in order to determine how much money should go to which account | max_len_payout: the maximum amount of accounts the market can payout at once (this is limited by gas fee) 
    ///// 
    fn nft_transfer_payout(&mut self, receiver_id: AccountId, token_id: TokenId, approval_id: u64, memo: String, balance: U128, max_len_payout: u64); //-- this method will be used for cross contract call on the receiver_id's contract actor (which must be implemented on the receiver_id's contract actor) once the nft_transfer_call() method is called and will return true if the token should be returned back to the sender

}
















impl MarketContract{ //-- we've defined the following methods of the `MarketContract` struct in this crate cause this crate is related to all internal calculation functions and methods - we don't need to add #[near_bindgen] proc macro attribute on this impl cause these are none exporting methods and won't compile to wasm to call them from cli 

    pub fn internal_remove_sale(&mut self, nft_contract_id: AccountId, token_id: TokenId) -> Option<Sale>{
        let contract_and_token_id = format!("{}{}{}", &nft_contract_id, DELIMETER, token_id); //-- creating the unique id for the sale object from nft_contract_id and the token_id
        match self.sales.remove(&contract_and_token_id){ //-- removing the sale object related to a unique sale id
            Some(sale) => {
                let mut sale_ids = match self.sales_by_owner_id.get(&sale.owner_id){ //-- getting the set of all sale ids related to an owner_id of the NFT which is the seller
                     Some(sale_ids) => {
                         sale_ids //-- returning the set of all sale ids related to a specific owner
                     },
                     None => {
                         env::panic_str("Found No Sale Ids"); //-- &str allocates low cost storage than the String which will get usize (usize is 64 bits or 24 bytes on 64 bits arch) * 3 (pointer, len, capacity) bytes; cause it's just the size of the str itself which is the total length of its utf8 bytes array on either stack, heap or binary which is equals to its length of utf8 bytes and due to its unknown size at compile time we must borrow it by taking a pointer to its location
                     },
                 };
                 let mut nft_ids = match self.by_nft_contract_id.get(&nft_contract_id){ //-- getting the set of all nft ids related to the passed in nft_contract_id which is the account_id and who owns the NFT contract
                     Some(nft_ids) => {
                         nft_ids //-- returning the set of all nft ids related to a specific owner
                     },
                     None => {
                         env::panic_str("Found No Nft Ids"); //-- &str allocates low cost storage than the String which will get usize (usize is 64 bits or 24 bytes on 64 bits arch) * 3 (pointer, len, capacity) bytes; cause it's just the size of the str itself which is the total length of its utf8 bytes array on either stack, heap or binary which is equals to its length of utf8 bytes and due to its unknown size at compile time we must borrow it by taking a pointer to its location
                     }
                 };
                 sale_ids.remove(&contract_and_token_id); //-- removing the created unique sale id from the set of all sale ids related to its owner
                 nft_ids.remove(&token_id); //-- removing the NFT from the found nft_ids set for a specific owner related to the passed in token_id  
         
         
                 
                 if sale_ids.is_empty(){ //-- if the found set of all sale ids was empty we have to remove the sale.owner_id entry from the self.sales_by_owner_id collection 
                     self.sales_by_owner_id.remove(&sale.owner_id);
                 } else{ //-- if the found set of all sale ids was'nt empty we have to insert the updated sale ids set (since we've removed the a sale id from it) back to the self.sales_by_owner_id collection 
                     self.sales_by_owner_id.insert(&sale.owner_id, &sale_ids); //-- inserting the updated sale_ids set back into the self.sales_by_owner_id collection
                 }
                 if nft_ids.is_empty(){ //-- if the found set of all nft ids was empty we have to remove the passed in nft_contract_id entry from the self.by_nft_contract_id collection
                     self.by_nft_contract_id.remove(&nft_contract_id);
                 } else{ //-- if the found set of all nft ids was'nt empty we have to insert the updated nft ids set (since we've removed the a nft id from it) back to the self.by_nft_contract_id collection 
                     self.by_nft_contract_id.insert(&nft_contract_id, &nft_ids); //-- inserting the updated nft_ids set back into the self.by_nft_contract_id collection
                 }
         
                 // transfer the depositted storage cost back to the seller
                 Promise::new(sale.owner_id.clone()).transfer(STORAGE_PER_SALE);

                 Some(sale)

            },
            None => { //-- means there is no set related to this unique sale id
                env::log_str("Found No Sale"); //-- &str allocates low cost storage than the String which will get usize (usize is 64 bits or 24 bytes on 64 bits arch) * 3 (pointer, len, capacity) bytes; cause it's just the size of the str itself which is the total length of its utf8 bytes array on either stack, heap or binary which is equals to its length of utf8 bytes and due to its unknown size at compile time we must borrow it by taking a pointer to its location
				None
            }
        }

    }


    pub fn internal_add_offer(&mut self, nft_contract_id: AccountId, token_id: TokenId, price: U128, buyer_id: AccountId){
        
        let contract_account_id_token_id = format!("{}{}{}{}{}", nft_contract_id, DELIMETER, buyer_id, DELIMETER, token_id);
        self.offers.insert(
            &contract_account_id_token_id,
            &OfferData { 
                buyer_id: buyer_id.clone(),
                nft_contract_id: nft_contract_id.clone(),
                token_id: token_id.clone(),
                price: price.0,
            } 
        );


        let mut by_owner_id = self.by_owner_id.get(&buyer_id).unwrap_or_else(|| {
            UnorderedSet::new(
                utils::Storagekey::ByOwnerIdInner {
                    //// we get a new unique prefix for the collection by hashing the owner
                    account_id_hash: hash_account_id(&buyer_id),
                }
                .try_to_vec()
                .unwrap(),
            )
        });
        by_owner_id.insert(&contract_account_id_token_id);
        self.by_owner_id.insert(&buyer_id, &by_owner_id);

        
    }


    pub fn internal_delete_offer(&mut self, nft_contract_id: AccountId, token_id: TokenId, buyer_id: AccountId) -> Option<OfferData>{
        
        let contract_account_id_token_id = format!("{}{}{}{}{}", nft_contract_id, DELIMETER, buyer_id, DELIMETER, token_id);
        let offer_data = self.offers.remove(&contract_account_id_token_id);

        match offer_data {
            Some(offer) => {
                let by_owner_id = self.by_owner_id.get(&offer.buyer_id);
                if let Some(mut by_owner_id) = by_owner_id {
                    by_owner_id.remove(&contract_account_id_token_id);
                    if by_owner_id.is_empty() {
                        self.by_owner_id.remove(&offer.buyer_id);
                    } else {
                        self.by_owner_id.insert(&offer.buyer_id, &by_owner_id);
                    }
                }
                return Some(offer);
            }
            None => return None,
        };


        
    }



    pub fn internal_update_approval_id(&mut self, approval_id: &u64, nft_contract_id: &AccountId, account_id: &AccountId, token_id: &TokenId){
    
        let contract_and_token_id = format!("{}{}{}", nft_contract_id, DELIMETER, token_id);
        if let Some(mut market_data) = self.market.get(&contract_and_token_id){
            market_data.approval_id = approval_id.clone();
            self.market.insert(&contract_and_token_id, &market_data);
        }

    }


    pub fn internal_accept_offer(&mut self, nft_contract_id: AccountId, buyer_id: AccountId, token_id: TokenId, seller_id: AccountId, approval_id: u64, price: u128) -> PromiseOrValue<bool>{
        
        let contract_account_id_token_id = format!("{}{}{}{}{}", nft_contract_id, DELIMETER, buyer_id, DELIMETER, token_id);
        let offer_data_raw = self.offers.get(&contract_account_id_token_id); 

        if offer_data_raw.is_none() {
            self.internal_update_approval_id(&approval_id, &nft_contract_id, &seller_id, &token_id);
            env::log_str("SMARTIES: Offer does not exist");
            return PromiseOrValue::Value(false);
        }

        self.internal_delete_market_data(&nft_contract_id, &token_id);

        let offer_data = offer_data_raw.unwrap();

        assert_eq!(offer_data.token_id, token_id.clone());
        assert_eq!(offer_data.price, price);

        let offer_data = self
            .internal_delete_offer(
                nft_contract_id.clone().into(),
                token_id.clone(),
                buyer_id.clone(),
            )
            .expect("SMARTIES: Offer does not exist");

        let memo = "accept offer".to_string();
        PromiseOrValue::Promise(
            nft_contract::ext(nft_contract_id) //-- the account_id that this method must be called and executed inside since the account_id param is the one who is responsible for executing this call which is the NFT owner which is the seller contract actor account
            .with_attached_deposit(1) //-- we must attach 1 yocto Ⓝ (1e24) in the following cross contract call since inside the nft_transfer_payout() method we've enforced the caller to attach 1 yocto Ⓝ (1e24) for security reasons like prevent the contract call from DDOSing 
            .with_static_gas(GAS_FOR_NFT_TRANSFER) //-- the total gas fee which will be deposited in yocto Ⓝ (1e24) from the caller wallet for this transaction cross contract call
            .nft_transfer_payout( //-- initiating a corss contract call by creating a transaction which is a promise (future object) ActionReceipt object which must be executed on receiver_id's contract actor account (NFT owner which is the seller) to transfer the NFT to the buyer contract actor account and fulfill the pending DataReceipt future object (which is an async message) with the cross contract call result inside the resolve_purchase() callback method using .then() since the fulfilled DataReceipt future object contains a payout object used for the market to distribute funds to the appropriate accounts - - calling the nft_transfer_payout() cross contract call promise method on the receiver side (NFT owner which is the seller) from the extended receiver_id's contract actor interface which is `nft_contract`
                offer_data.buyer_id.clone(),
                token_id.clone(),
                approval_id,
                memo,
                U128::from(offer_data.price),
                10u64, // max length payout
            ).then( //-- wait for the scheduled transaction which is a promise (future object) ActionReceipt object on the receiver_id's contract actor (NFT owner which is the seller) to finish executing to resolve it using .then() method inside resolve_purchase() method
                Self::ext(env::current_account_id()) //-- the account_id that this method must be called and executed inside which is the current_account_id() and is the one who owns this contract which is the market itself - account_id param is the one who is responsible for executing this call which is the market itself
                    .with_attached_deposit(NO_DEPOSIT) //-- no deposit is required from the caller for calling the resolve_purchase() callback method since this method doesn't require any deposit amount
                    .with_static_gas(GAS_FOR_ROYALTIES) //-- total gas required for calling the callback method which has taken from the attached deposited (contract budget) when the caller called the nft_transfer_call() method
                    .resolve_offer( //-- calling resolve_purchase() method from the extended interface of the current contract actor (market contract) which is the `market_contract` contract; since this is a private method only the owner of the this contract can call it means the caller must be the signer or the one who initiated, owned and signed the contract or the account of the contract itself which is the market itself; since callback methods are private thus the caller of them must be the owner of the contract
                        seller_id,
                        offer_data,
                        token_id,
                    ) //-- resolve_purchase() method will return a U128 price which is in yocto Ⓝ (1e24)
            ) //-- returning the promise from this method 
        )
    }



    pub fn internal_add_market_data(&mut self, owner_id: AccountId, approval_id: u64, nft_contract_id: AccountId, token_id: TokenId, price: U128, mut started_at: Option<U64>, ended_at: Option<U64>, end_price: Option<U128>, is_auction: Option<bool>) {
        
        let contract_and_token_id = format!("{}{}{}", nft_contract_id, DELIMETER, token_id);
        let bids: Option<Bids> = match is_auction {
            Some(u) => {
                if u {
                    Some(Vec::new())
                } else {
                    None
                }
            }
            None => None,
        };

        let current_time: u64 = env::block_timestamp();

        if started_at.is_some() {
            assert!(started_at.unwrap().0 >= current_time);

            if ended_at.is_some() {
                assert!(started_at.unwrap().0 < ended_at.unwrap().0);
            }
        }

        if let Some(is_auction) = is_auction {
            if is_auction == true {
                if started_at.is_none() {
                    started_at = Some(U64(current_time));
                }
            }

            assert!(ended_at.is_some(), "SMARTIES: Ended at is none")
        }

        if ended_at.is_some() {
            assert!(ended_at.unwrap().0 >= current_time);
        }

        assert!(
            price.0 < MAX_PRICE,
            "SMARTIES: price higher than {}",
            MAX_PRICE
        );

        self.market.insert(
            &contract_and_token_id,
            &MarketData {
                owner_id: owner_id.clone().into(),
                approval_id,
                nft_contract_id: nft_contract_id.clone().into(),
                token_id: token_id.clone(),
                price: price.into(),
                bids,
                started_at: match started_at {
                    Some(x) => Some(x.0),
                    None => None,
                },
                ended_at: match ended_at {
                    Some(x) => Some(x.0),
                    None => None,
                },
                end_price: match end_price {
                    Some(x) => Some(x.0),
                    None => None,
                },
                accept_nft_contract_id: None,
                accept_token_id: None,
                is_auction,
            },
        );

        let mut token_ids = self.by_owner_id.get(&owner_id).unwrap_or_else(|| {
            UnorderedSet::new(
                Storagekey::ByOwnerIdInner {
                    account_id_hash: hash_account_id(&owner_id),
                }
                .try_to_vec()
                .unwrap(),
            )
        });

        token_ids.insert(&contract_and_token_id);
        self.by_owner_id.insert(&owner_id, &token_ids);

        env::log_str(
            &json!({
                "type": "add_market_data",
                "params": {
                    "owner_id": owner_id,
                    "approval_id": approval_id,
                    "nft_contract_id": nft_contract_id,
                    "token_id": token_id,
                    "price": price,
                    "started_at": started_at,
                    "ended_at": ended_at,
                    "end_price": end_price,
                    "is_auction": is_auction,
                }
            })
            .to_string(),
        );
    }




    pub fn internal_delete_market_data(&mut self, nft_contract_id: &AccountId, token_id: &TokenId) -> Option<MarketData> {
        
        let contract_and_token_id = format!("{}{}{}", &nft_contract_id, DELIMETER, token_id);
        env::log_str(&contract_and_token_id);
        let market_data: Option<MarketData> =
            if let Some(market_data) = self.market.get(&contract_and_token_id) {
                self.market.remove(&contract_and_token_id);

                if let Some(ref bids) = market_data.bids {
                    for bid in bids {
                        Promise::new(bid.bidder_id.clone()).transfer(bid.price.0);
                    }
                };

                Some(market_data)
            } else {
                None
            };


        let _nft_contract_id: AccountId = nft_contract_id.as_str().parse().unwrap(); //-- convert the nft_contract_id to AccountId
        self.internal_remove_sale(_nft_contract_id, token_id.to_string()); //-- also remove the sale object f


        market_data.map(|market_data| {
            let by_owner_id = self.by_owner_id.get(&market_data.owner_id);
            if let Some(mut by_owner_id) = by_owner_id {
                by_owner_id.remove(&contract_and_token_id); //-- you might get the index overflow for unorderedset bug
                if by_owner_id.is_empty() {
                    self.by_owner_id.remove(&market_data.owner_id);
                } else {
                    self.by_owner_id.insert(&market_data.owner_id, &by_owner_id);
                }
            }
            market_data
        })

    }


    pub fn internal_cancel_bid(&mut self, nft_contract_id: AccountId, token_id: TokenId, account_id: AccountId) {
        let contract_and_token_id = format!("{}{}{}", &nft_contract_id, DELIMETER, token_id);
        let mut market_data = self
          .market
          .get(&contract_and_token_id)
          .expect("SMARTIES: Token id does not exist");
    
        let mut bids = market_data.bids.unwrap();
    
        assert!(
          !bids.is_empty(),
          "SMARTIES: Bids data does not exist"
        );
    
        // Retain all elements except account_id
        bids.retain(|bid| {
          if bid.bidder_id == account_id {
            // refund
            Promise::new(bid.bidder_id.clone()).transfer(bid.price.0);
          }
    
          bid.bidder_id != account_id
        });
    
        market_data.bids = Some(bids);
        self.market.insert(&contract_and_token_id, &market_data);
    
        env::log_str(
          &json!({
            "type": "cancel_bid",
            "params": {
              "bidder_id": account_id, "nft_contract_id": nft_contract_id, "token_id": token_id
            }
          })
          .to_string(),
        );
      }


}