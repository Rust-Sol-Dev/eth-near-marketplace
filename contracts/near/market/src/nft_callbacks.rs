




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












// NOTE - we'll handle all cross& contract call processes coming from each NFT contract in here like calling nft_on_approve() method from the NFT contract
// NOTE - we have to define a trait with a name NonFungibleToken*Receiver in which the * is the name of the process that is triggered from the NFT contract like NonFungibleTokenApprovalsReceiver which is a trait contains all approval methods
















// -----------------------------------------------------------------------------------
//      METHODS THAT WILL BE CALLED FROM NFT CONTRACT USING CROSS CONTRACT CALLS 
// -----------------------------------------------------------------------------------
trait NonFungibleTokenApprovalsReceiver{ //-- this trait will be used to define the approval methods which might be called from the NFT contracts like nft_on_approve() method - when nft_approve() method is called on the NFT contract if the msg param of the nft_approve() method wan't None it'll fire a cross contract call to our marketplace and the following is the method that is invoked     
    fn nft_on_approve(&mut self, token_id: TokenId, owner_id: AccountId, approval_id: u64, msg: String); //-- since we want to create a sale object in this method and mutate the state of the contract we must define the first param as &mut self
}














// -------------------------------------------------------------
//     TRAITS IMPLEMENTATION OF CROSS CONTRACT CALL METHODS 
// -------------------------------------------------------------
#[near_bindgen] //-- implementing the #[near_bindgen] proc macro attribute on the trait implementation for the extended interface (NonFungibleTokenApprovalsReceiver trait) of `MarketContract` struct interface in order to have a compiled wasm trait methods for this contract struct so we can call it from the near cli 
impl NonFungibleTokenApprovalsReceiver for MarketContract{ //-- implementing the NonFungibleTokenApprovalsReceiver trait for our main `MarketContract` struct to extend its interface; bounding the mentioned trait to the `NFTContract` struct to query NFT approval infos
    
    #[payable]
    fn nft_on_approve(&mut self, token_id: TokenId, owner_id: AccountId, approval_id: u64, msg: String){ //-- overriding the nft_on_approve() method of the NonFungibleTokenApprovalsReceiver trait - if the seller or the NFT owner wants to sell his/her NFT he/she must call nft_approve() method on his/her NFT contract and pass a none empty message to that method since this method (which is in our marketplace obviously) will fire only if the msg param inside the nft_approve() method wasn't empty (Some(msg)); and if so the NFT owner which is the seller can list his/her NFT on the market by scheduling this call on his/her NFT contract and give the access to the market to transfer and list the NFT on behalf of his/her   
        
        /*

            -------------------------------------------------------------------------------------------------

                https://www.near-sdk.io/zero-to-hero/beginner/actions#predecessor-signer-and-current-account
            
                cross contract call chain ➔ NFT owner --call--> nft_approve(..., msg) inside NFT contract --call--> nft_on_approve()
                signer                    ➔ the one who made this chain which is the caller of nft_approve() method which must be the NFT owner except minter
                caller                    ➔ the one who has called the nft_on_approve() method which is the NFT contract owner
                current owner             ➔ the market contract actor account is the owner of this contract

                NOTE - the caller of cross contract call must not be the signer since it might be an evil account 
                       that only checks the signer account to determine ownership of the NFT thus the caller can't be the signer
                       due to the fact that the NFT contract might be an evil contract which might do a cross-contract call 
                       to vulnerable-market.near, instructing it to transfer an NFT.


                NOTE - this method should only be called once the marketplace has gotten approval to transfer the token on behalf of all the owner
                       this is why there's an assertion that the predecessor cannot be the signer cause the minter first must approve the marketplace 
                       to transfer the token on behalf of all that NFT owners then on second sell the new owner can call nft_on_approve() 
                       method to list his/her NFT on the market to create the sale object. 

            -------------------------------------------------------------------------------------------------

        */

        ////
        /////// ➔ the signer is the starter of the cross contract call chain which must be the NFT owner who wants put his/her NFT on the marketplace create sell object and it can't be NFT contract owner
        /////// ➔ in several cross-contract calls, the signer is the account that pushed over the first domino in that chain reaction means the first one who called the nft_approve() method inside the NFT contract to call the nft_on_approve() method (led to this execution) inside the market contract which must be the NFT owner which must not be the NFT contract owner
        /////// ➔ in a none cross contract call transaction the caller and the signer are the same and is equals to the one who has called the method
        /////// ➔ in cross contract it seems that the signer (who signed the transaction with his/her keys) of it must not be equal to the account where the call was happened; in our case means the signer of the cross contract call is the NFT owner which must not be equal to the NFT contract owner or the minter means the minter can't directly call this method from its account thus we can say that the incoming call was for the second sell not the first seller which is the minter 
        /////// ➔ signer id is the NFT owner (except minter or the NFT contract owner) and the one who signed and initiated the cross contract call transaction; the signer of this method must be the NFT owner since the only person that can approve an account_id to transfer NFT on behalf of him/her is the NFT owner which must be the second seller cause it can't be the minter which is the first seller 
        /////// ➔ owner_id is the current NFT owner which is not the NFT contract owner since the current NFT might be transferred to another owner or has beed sold out on the market thus the current NFT owner might not be the minter or the one who owns this contract therefore owner_id might not equal to nft_contract_id
        //// 
        let nft_contract_id = env::predecessor_account_id(); //-- getting the caller id (last caller) of this method since this method will be called from the NFT contract thus the caller is the owner of the NFT contract
        let signer_id = env::signer_account_id(); //-- getting the signer id (who initiated and signed the transaction) in this call which is the NFT owner which has been passed to this method inside the nft_approve() method on the NFT contract
        if nft_contract_id == signer_id.clone(){ //-- if the signer or the owner of the NFT is the one who owns the NFT contract which is the minter we have to panic since 
            env::panic_str("Signer (The NFT Owner) Shouldn't Be The NFT Contract Owner (Caller Can't Be The Signer Since Caller Might Be A Vulnerable Account!)"); //-- &str allocates low cost storage than the String which will get usize (usize is 64 bits or 24 bytes on 64 bits arch) * 3 (pointer, len, capacity) bytes; cause it's just the size of the str itself which is the total length of its utf8 bytes array on either stack, heap or binary which is equals to its length of utf8 bytes and due to its unknown size at compile time we must borrow it by taking a pointer to its location
        }
        if owner_id != signer_id{ //-- means that only the NFT owner (except minter or the NFT contract owner) of this method must be the signer this transaction since only the NFT owner (except minter or the NFT contract owner) can approve an account_id (the market contract actor account) for transferring his/her NFT on behalf of hi/her 
            env::panic_str("The Passed In `owner_id` Param To This Method Which Is The NFT Owner Must Be Equal To The Signer"); //-- &str allocates low cost storage than the String which will get usize (usize is 64 bits or 24 bytes on 64 bits arch) * 3 (pointer, len, capacity) bytes; cause it's just the size of the str itself which is the total length of its utf8 bytes array on either stack, heap or binary which is equals to its length of utf8 bytes and due to its unknown size at compile time we must borrow it by taking a pointer to its location
        }


        let log_msg = format!(":::: SMARTIES: the msg param in market contract is {:?}", msg);
        env::log_str(&log_msg);



        let MarketArgs {
            market_type,
            price,
            buyer_id, //-- used for offer
            end_price,
            started_at,
            ended_at,
            is_auction,
        } = near_sdk::serde_json::from_str(&msg).expect("Not valid Smarties Markte Args");



        // replace old approval on market
        let contract_and_token_id = format!("{}{}{}", nft_contract_id, DELIMETER, token_id);
        if let Some(mut old_market) = self.market.get(&contract_and_token_id){
            old_market.approval_id = approval_id;
            self.market.insert(&contract_and_token_id,&old_market);
        }


        // ------------------------------------------------------------------------------------- SALE        
        if market_type == "sale"{
            


            //////// paying storage cost from the attached deposit
            let storage_deposit = env::attached_deposit();
            let depositor = Some(signer_id.clone());
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


            let log_deposit = format!(":::: SMARTIES: the attached deposit coming from nft contract is {:?}", storage_deposit);
            env::log_str(&log_deposit);
            

            //// we need to enforce that the user has enough storage for 1 EXTRA sale.  
            //// get the storage for a sale. dot 0 gets the first element from U128 tuple struct
            let storage_amount = self.storage_minimum_balance().0;
            //// get the total storage paid by the owner
            let owner_paid_storage = self.storage_deposits.get(&signer_id).unwrap_or(0);
            //// get the storage required which is simply the storage for the number of sales they have + 1 
            let signer_storage_required = (self.get_supply_by_owner_id(signer_id).0 + 1) as u128 * storage_amount; //-- adding one is required if the signer_id has 0 supply 
            //// make sure that the total paid is >= the required storage
            if owner_paid_storage < signer_storage_required{
                let panic_message = format!("Insufficient storage paid: {}, for {} sales at {} rate of per sale", owner_paid_storage, signer_storage_required / STORAGE_PER_SALE, STORAGE_PER_SALE);
                env::panic_str(panic_message.as_str()); //-- &str allocates low cost storage than the String which will get usize (usize is 64 bits or 24 bytes on 64 bits arch) * 3 (pointer, len, capacity) bytes; cause it's just the size of the str itself which is the total length of its utf8 bytes array on either stack, heap or binary which is equals to its length of utf8 bytes and due to its unknown size at compile time we must borrow it by taking a pointer to its location
            }
            self.internal_delete_market_data(&nft_contract_id, &token_id);
            



            /////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
                let contract_and_token_id = format!("{}{}{}", nft_contract_id, DELIMETER, token_id);
                self.sales.insert(
                    &contract_and_token_id,
                    &Sale {
                        owner_id: owner_id.clone(), //// owner of the sale / token
                        approval_id, //// approval ID for that token that was given to the market
                        nft_contract_id: nft_contract_id.to_string(), //// NFT contract the token was minted on
                        token_id: token_id.clone(), //// the actual token ID
                        sale_conditions: price.unwrap(), //// the sale conditions 
                },
                );
    
                //// Extra functionality that populates collections necessary for the view calls 
                //// get the sales by owner ID for the given owner. If there are none, we create a new empty set
                let mut sales_by_owner_id = self.sales_by_owner_id.get(&owner_id).unwrap_or_else(|| {
                    UnorderedSet::new(
                        utils::Storagekey::ByOwnerIdInner {
                            //// we get a new unique prefix for the collection by hashing the owner
                            account_id_hash: hash_account_id(&owner_id),
                        }
                        .try_to_vec()
                        .unwrap(),
                    )
                });
                //// insert the unique sale ID into the set
                sales_by_owner_id.insert(&contract_and_token_id);
                //// insert that set back into the collection for the owner
                self.sales_by_owner_id.insert(&owner_id, &sales_by_owner_id);
        
                //// get the token IDs for the given nft contract ID. If there are none, we create a new empty set
                let mut by_nft_contract_id = self
                    .by_nft_contract_id
                    .get(&nft_contract_id)
                    .unwrap_or_else(|| {
                        UnorderedSet::new(
                            utils::Storagekey::ByNFTContractIdInner {
                                //// we get a new unique prefix for the collection by hashing the owner
                                account_id_hash: hash_account_id(&nft_contract_id),
                            }
                            .try_to_vec()
                            .unwrap(),
                        )
                    });
                //// insert the token ID into the set
                by_nft_contract_id.insert(&token_id);
                //// insert the set back into the collection for the given nft contract ID
                self.by_nft_contract_id
                    .insert(&nft_contract_id, &by_nft_contract_id);  
                /////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
                
                
                

                self.internal_add_market_data(
                    owner_id,
                    approval_id,
                    nft_contract_id,
                    token_id,
                    price.unwrap(),
                    started_at,
                    ended_at,
                    end_price,
                    is_auction,
                );
        } 
        // ------------------------------------------------------------------------------------- OFFER
        else if market_type == "accept_offer" {
            assert!(buyer_id.is_some(), "SMARTIES: Account id is not specified");
            assert!(price.is_some(), "SMARTIES: Price is not specified (for check)");

            self.internal_accept_offer(
                nft_contract_id,
                buyer_id.unwrap(),
                token_id,
                owner_id,
                approval_id,
                price.unwrap().0,
            );
        }

    
    }

    //////////////////////////////////////////////////////////////////////////////////////////////////
    //////////////////////////////////////////////////////////////////////////////////////////////////










}