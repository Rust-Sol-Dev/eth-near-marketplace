



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












#[near_bindgen] //-- implementing the #[near_bindgen] proc macro attribute on `MarketContract` struct to compile all its methods to wasm so we can call them in near cli
impl MarketContract{ //-- following methods will be compiled to wasm using #[near_bindgen] proc macro attribute 


  // --------------------------------------------------------------------------------- ADD BID

  #[payable]
  pub fn add_bid(&mut self, nft_contract_id: AccountId, token_id: TokenId, amount: U128) {
      let contract_and_token_id = format!("{}{}{}", &nft_contract_id, DELIMETER, token_id);
      let mut market_data = self.market.get(&contract_and_token_id).expect("SMARTIES: Token id does not exist");

      assert_eq!(market_data.is_auction.unwrap(), true, "SMARTIES: not auction");

      let bidder_id = env::predecessor_account_id();
      let current_time = env::block_timestamp();

      if market_data.started_at.is_some() {
          assert!(
              current_time >= market_data.started_at.unwrap(),
              "SMARTIES: Sale has not started yet"
          );
      }

      if market_data.ended_at.is_some() {
          assert!(
              current_time <= market_data.ended_at.unwrap(),
              "SMARTIES: Sale has ended"
          );
      }

      let remaining_time = market_data.ended_at.unwrap() - current_time;
      if remaining_time <= FIVEMINUTES {
        let extended_ended_at = market_data.ended_at.unwrap() + FIVEMINUTES;
        market_data.ended_at = Some(extended_ended_at);

        env::log_str(
          &json!({
              "type": "extend_auction",
              "params": {
                  "nft_contract_id": nft_contract_id,
                  "token_id": token_id,
                  "ended_at": extended_ended_at,
              }
          })
          .to_string(),
        );
      }

      assert_ne!(market_data.owner_id, bidder_id, "SMARTIES: Owner cannot bid their own token");

      assert!(
          env::attached_deposit() >= amount.into(),
          "SMARTIES: attached deposit is less than amount"
      );

      let new_bid = Bid {
          bidder_id: bidder_id.clone(),
          price: amount.into(),
      };

      let mut bids = market_data.bids.unwrap_or(Vec::new());

      if !bids.is_empty() {
          let current_bid = &bids[bids.len() - 1];

          assert!(
            amount.0 >= current_bid.price.0 + (current_bid.price.0 / 100 * 5),
            "SMARTIES: Can't pay less than or equal to current bid price + 5% : {:?}",
            current_bid.price.0 + (current_bid.price.0 / 100 * 5)
          );

          assert!(
              amount.0 >= market_data.price,
              "SMARTIES: Can't pay less than starting price: {:?}",
              U128(market_data.price)
          );

          // Retain all elements except account_id
          bids.retain(|bid| {
            if bid.bidder_id == bidder_id {
              // refund
              Promise::new(bid.bidder_id.clone()).transfer(bid.price.0);
            }

            bid.bidder_id != bidder_id
          });
      } else {
          assert!(
              amount.0 >= market_data.price,
              "SMARTIES: Can't pay less than starting price: {:?}",
              market_data.price
          );
      }

      bids.push(new_bid);
      market_data.bids = Some(bids);
      self.market.insert(&contract_and_token_id, &market_data);

      // Remove first element if bids.length >= 100
      let updated_bids = market_data.bids.unwrap_or(Vec::new());
      if updated_bids.len() >= 100 {
        self.internal_cancel_bid(nft_contract_id.clone(), token_id.clone(), updated_bids[0].bidder_id.clone())
      }

      env::log_str(
          &json!({
              "type": "add_bid",
              "params": {
                  "bidder_id": bidder_id,
                  "nft_contract_id": nft_contract_id,
                  "token_id": token_id,
                  "amount": amount,
              }
          })
          .to_string(),
      );
  }


  // --------------------------------------------------------------------------------- CANCEL BID

  #[payable]
  pub fn cancel_bid(&mut self, nft_contract_id: AccountId, token_id: TokenId, account_id: AccountId) {
    assert_one_yocto();
    let contract_and_token_id = format!("{}{}{}", &nft_contract_id, DELIMETER, token_id);
    let market_data = self
      .market
      .get(&contract_and_token_id)
      .expect("SMARTIES: Token id does not exist");

    let bids = market_data.bids.unwrap();

    assert!(
      !bids.is_empty(),
      "SMARTIES: Bids data does not exist"
    );

    for x in 0..bids.len() {
      if bids[x].bidder_id == account_id {
        assert!(
          [bids[x].bidder_id.clone(), self.owner_id.clone()]
            .contains(&env::predecessor_account_id()),
            "SMARTIES: Bidder or owner only"
        );
      }
    }

    self.internal_cancel_bid(nft_contract_id, token_id, account_id);
  }



  // --------------------------------------------------------------------------------- ACCEPT BID

  #[payable]
  pub fn accept_bid(&mut self, nft_contract_id: AccountId, token_id: TokenId) {
      let predecessor_account_id = env::predecessor_account_id();
      if predecessor_account_id != self.owner_id {
          assert_one_yocto();
      }
      let contract_and_token_id = format!("{}{}{}", &nft_contract_id, DELIMETER, token_id);
      let mut market_data = self
          .market
          .get(&contract_and_token_id)
          .expect("SMARTIES: Token id does not exist");

      assert_eq!(market_data.is_auction.unwrap(), true, "SMARTIES: not auction");
      let current_time: u64 = env::block_timestamp();

      assert!(
          [market_data.owner_id.clone(), self.owner_id.clone()]
          .contains(&predecessor_account_id),
          "SMARTIES: Seller or owner only"
      );

      if predecessor_account_id == self.owner_id && market_data.ended_at.is_some() {
        assert!(
          current_time >= market_data.ended_at.unwrap(),
          "SMARTIES: Auction has not ended yet"
        );
      }

      let mut bids = market_data.bids.unwrap();

      assert!(!bids.is_empty(), "SMARTIES: Cannot accept bid with empty bid");

      let selected_bid = bids.remove(bids.len() - 1);

      // refund all except selected bids
      for bid in &bids {
        // refund
        Promise::new(bid.bidder_id.clone()).transfer(bid.price.0);
      }
      bids.clear();

      market_data.bids = Some(bids);
      self.market.insert(&contract_and_token_id, &market_data);

      self.process_purchase(
          market_data.nft_contract_id,
          token_id,
          selected_bid.price.clone(),
          selected_bid.bidder_id.clone(),
          Some("bid".to_string())
      );



  }

  // --------------------------------------------------------------------------------- END AUCTION

  #[payable]
  pub fn end_auction(&mut self, nft_contract_id: AccountId, token_id: TokenId) {
    let predecessor_account_id = env::predecessor_account_id();
    if predecessor_account_id != self.owner_id {
        assert_one_yocto();
    }

    let current_time = env::block_timestamp();
    let contract_and_token_id = format!("{}{}{}", &nft_contract_id, DELIMETER, &token_id);
    let mut market_data = self
        .market
        .get(&contract_and_token_id)
        .expect("SMARTIES: Market data does not exist");

    assert_eq!(market_data.is_auction.unwrap(), true, "SMARTIES: not auction");
    assert!(
      [market_data.owner_id.clone(), self.owner_id.clone()]
        .contains(&predecessor_account_id),
      "SMARTIES: Seller or owner only"
    );

    if predecessor_account_id == self.owner_id && market_data.ended_at.is_some() {
      assert!(
        current_time >= market_data.ended_at.unwrap(),
        "SMARTIES: Auction has not ended yet (for owner)"
      );
    }

    let mut bids = market_data.bids.unwrap();

    if bids.is_empty() { //-- if there was no bids out there then we have to delete the market data by ending the auction
      self.internal_delete_market_data(&nft_contract_id, &token_id);

      env::log_str(
          &json!({
              "type": "delete_market_data",
              "params": {
                  "owner_id": market_data.owner_id,
                  "nft_contract_id": nft_contract_id,
                  "token_id": token_id,
              }
          })
          .to_string(),
      );
    } else { //-- if there was any bid out there we have to payout all the bidders
      let selected_bid = bids.remove(bids.len() - 1);

      // refund all except selected bids
      for bid in &bids {
        Promise::new(bid.bidder_id.clone()).transfer(bid.price.0);
      }

      bids.clear();

      market_data.bids = Some(bids);
      self.market.insert(&contract_and_token_id, &market_data);


        self.process_purchase(
          nft_contract_id,
          token_id,
          selected_bid.price.clone(),
          selected_bid.bidder_id.clone(),
          Some("bid".to_string())
      );
    }
  }








}