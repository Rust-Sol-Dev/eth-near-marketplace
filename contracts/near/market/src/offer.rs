



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


    #[payable]
    pub fn add_offer(&mut self, nft_contract_id: AccountId, token_id: AccountId, price: U128){
        
      assert_eq!(
          env::attached_deposit(),
          price.0,
          "SMARTIES: Attached deposit != price"
      );

      
      let buyer_id = env::predecessor_account_id();
      let offer_data = self.internal_delete_offer(nft_contract_id.clone(), token_id.to_string(), buyer_id.clone());
      if offer_data.is_some(){
          Promise::new(buyer_id.clone()).transfer(offer_data.unwrap().price); //-- transfer back the nft price to the buyer
      }


      //////// paying storage cost from the attached deposit
      let storage_deposit = env::attached_deposit();
      let depositor = Some(buyer_id.clone());
      let storage_account_id = depositor
                                    .map(|a| a.into()) //-- mapping the account_id inside the Option to convert it into a valid account_id using .into() method which will return the T
                                    .unwrap_or_else(env::predecessor_account_id);
    assert!(
        storage_deposit >= STORAGE_PER_SALE,
        "SMARTIES : Requires minimum deposit of {}",
        STORAGE_PER_SALE
    );
      let mut account_id_balance = self.storage_deposits.get(&storage_account_id).unwrap_or(0);
      account_id_balance += storage_deposit;
      self.storage_deposits.insert(&storage_account_id, &account_id_balance);
      /////////



      let storage_amount = self.storage_minimum_balance().0;
      let owner_paid_storage = self.storage_deposits.get(&buyer_id).unwrap_or(0);
      let signer_storage_required =
          (self.get_supply_by_owner_id(buyer_id.clone()).0 + 1) as u128 * storage_amount;

      assert!(
          owner_paid_storage >= signer_storage_required,
          "Insufficient storage paid: {}, for {} offer at {} rate of per offer",
          owner_paid_storage,
          signer_storage_required / storage_amount,
          storage_amount,
      );


        self.internal_add_offer(nft_contract_id.clone().into(), token_id.clone().to_string(), price, buyer_id.clone());

        env::log_str(
          &json!({
              "type": "add_offer",
              "params": {
                  "buyer_id": buyer_id,
                  "nft_contract_id": nft_contract_id,
                  "token_id": token_id,
                  "price": price,
              }
          })
          .to_string(),
      );

      
    }



    #[payable]
    pub fn delete_offer(&mut self, nft_contract_id: AccountId, token_id: TokenId) {

        assert_one_yocto();
        let buyer_id = env::predecessor_account_id();
        let contract_account_id_token_id = format!("{}{}{}{}{}", nft_contract_id, DELIMETER, buyer_id, DELIMETER, token_id);
        let offer_data = self.offers.get(&contract_account_id_token_id).expect("SMARTIES: Offer does not exist");

        assert_eq!(offer_data.token_id, token_id); //-- offer token_id must be equals to the passed in token_id

        assert_eq!(
            offer_data.buyer_id, buyer_id,
            "SMARTIES: Caller not offer's buyer"
        );

        self.internal_delete_offer(
            nft_contract_id.clone().into(),
            token_id.clone(),
            buyer_id.clone(),
        )
        .expect("SMARTIES: Offer not found");

        Promise::new(offer_data.buyer_id).transfer(offer_data.price);

        env::log_str(
            &json!({
                "type": "delete_offer",
                "params": {
                    "nft_contract_id": nft_contract_id,
                    "buyer_id": buyer_id,
                    "token_id": token_id,
                }
            })
            .to_string(),
        );
    }


    pub fn get_offer(&self, nft_contract_id: AccountId, buyer_id: AccountId, token_id: TokenId) -> JsonOfferData {

      let contract_account_id_token_id = format!("{}{}{}{}{}", nft_contract_id, DELIMETER, buyer_id, DELIMETER, token_id);
      let offer_data = self.offers.get(&contract_account_id_token_id).expect("SMARTIES: Offer does not exist");
      assert_eq!(offer_data.token_id, token_id); //-- offer token_id must be equals to the passed in token_id


      JsonOfferData {
          buyer_id: offer_data.buyer_id,
          nft_contract_id: offer_data.nft_contract_id,
          token_id: offer_data.token_id,
          price: U128(offer_data.price), //-- price in json must be string (U128 is of type String which has u128 inside of it) since it must be shown inside the js and browsers 
      }


    }



    #[private]
    pub fn resolve_offer(&mut self, seller_id: AccountId, offer_data: OfferData, token_id: TokenId) -> U128 {


        let payout_result = promise_result_as_success().and_then(|value|{ //-- promise_result_as_success() function uses env::promise_result() function under the hood - getting the result of the executed promise, the nft_transfer_payout() cross contract call; if it was successful we have the value in utf8 encoded form (since data between actors will be sent asyncly and serialized through the mpsc channel) which we have to deserialize it otherwise we'll get the None if the result of the promise wasn't successful
            serde_json::from_slice::<Payout>(&value) //-- deserializing the encoded payout object in form utf8 into the Payout struct
                .ok() //-- get the deserialized payout object only if the deserialization was ok
                .and_then(|payout_object|{ //-- and_then() returns an Option of either the parent method result or the result of the passed in closure which in our case we've passed in a closure with deserialized payout object as its arg
                    if payout_object.payout.len() > 10 || payout_object.payout.is_empty(){
                        ////
                        /////// ➔ codes after env::panic_str() are unreachable cause by panicking the main thread future codes will not be compiled
                        ////
                        env::log_str("SMARTIES : Can't Payout More Than 10 Royalties"); //-- &str allocates low cost storage than the String which will get usize (usize is 64 bits or 24 bytes on 64 bits arch) * 3 (pointer, len, capacity) bytes; cause it's just the size of the str itself which is the total length of its utf8 bytes array on either stack, heap or binary which is equals to its length of utf8 bytes and due to its unknown size at compile time we must borrow it by taking a pointer to its location
                        None //-- returning None since everything wasn't ok with the payout object
                    } else{ //-- if the total royalties are smaller or equals than 10 accounts we move forward and can payout them 
                        let mut reminder = offer_data.price; //-- price if of type U128 which we have to get its first element cause it's a tuple like struct - keeping track of how much the NFT contract wants us to payout, we'll start at the full price payed by the buyer and will subtract by the value of each royalty inside the loop
                        for &value in payout_object.payout.values(){ //-- iterating through the payout object which is a hashmap in form HashMap<AccountId, U128> - the values (are of type U128 means we must get their first element in order to have their actual value, cause they are tuple like struct) inside the hashmap are the payout values in yocto Ⓝ (1e24) that royaltie account_ids must get paid based on their royaly percentage calculated inside the nft_transfer_payout() method
                            reminder = reminder.checked_sub(value.0).unwrap(); //-- updating the reminder by subtracting the first element of each value (since they are tuple like struct) from the total price of the NFT which buyer has paid for - checked_sub() method will ompute self - passed in param and will return None if overflow occurred
                        }
                        ////
                        /////// ➔ if the reminder was 0 means that the NFT contract wanted us the total amount of the NFT price to be paid all royalties out since it might be too many royalties that forced us to spent all the price of the NFT for NEAR payout process to pay the royalty account_ids out
                        /////// ➔ if the reminder was 1 means that the NFT contract wanted us the 90 % of the total amount of the NFT price to be paid all royalties out since the sum of all royalty percentage value inside the token.perpetual_royalties hashmap might be 9999 which is equals to 10000 - 1 (the valud of 100 % is 10000) which means all royalties payout cost us 90 % of the total amount of the NFT price 
                        ////
                        if reminder == 0 || reminder == 1{ //-- if NFT contract wants us the 100 % or 90 % of the total price of the NFT we can return the payout obejct for NEAR payout process
                            Some(payout_object.payout) //-- returning the payout object of type Option for NEAR payout process
                        } else{
                            None //-- returning None if the reminder was anything but 1 or 0 since paying out all the royalties didn't go well and we have some yocto Ⓝ (1e24) which is greater than 1 since we're subtracting each value of every royalty account_id from the NFT price to keep track of the total amount from the NFT price that the NFT contract wants us to payout
                        }                
                    }
                })
        });
        

        let payout = if let Some(payout_option) = payout_result{ //-- getting the payout object out of the payout_result for NEAR payout process 
            payout_option //-- return the payout object for NEAR payouts process
        } else{ //-- if we're here means that the payout_result is None since everything didn't go well with the payout object deserialized from the incoming data from the executed promise
            Promise::new(offer_data.buyer_id.clone()).transfer(u128::from(offer_data.price)); //-- transferring the price of the NFT back to the buyer contract actor account since the payout object is None 
            env::log_str(
                &json!({
                    "type": "resolve_offer_fail",
                    "params": {
                        "owner_id": seller_id,
                        "nft_contract_id": offer_data.nft_contract_id,
                        "token_id": token_id,
                        "price": offer_data.price.to_string(),
                        "buyer_id": offer_data.buyer_id,
                        "is_offer": true,
                    }
                }).to_string(),
             );
            return offer_data.price.into(); //-- returning the price of the NFT that the buyer has paid for
        };


        for (receiver_id, amount) in payout{ //-- iterating through the payout object to transferr the royalty amount to royalty account_ids - payout is of type HashMap<AccountId, U128> in which all values is of type u128 in yocto Ⓝ (1e24) and is the amount that a specific account_id must get paid based on his/her royalty percentage calculated inside the nft_transfer_payout() method
            Promise::new(receiver_id).transfer(amount.0); //-- transferring the amount in yocto Ⓝ (1e24) related to each royalty account_id from the sold out NFT; if we're here means that everything went well and the payout object wasn't None 
        }

        env::log_str(
            &json!({
                "type": "resolve_offer",
                "params": {
                    "owner_id": seller_id,
                    "nft_contract_id": &offer_data.nft_contract_id,
                    "token_id": &token_id,
                    "price": offer_data.price.to_string(),
                    "buyer_id": offer_data.buyer_id,
                    "is_offer": true,
                }
            })
            .to_string(),
        );

        return offer_data.price.into();

    }



}