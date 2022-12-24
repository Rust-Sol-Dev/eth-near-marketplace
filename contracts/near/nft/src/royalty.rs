









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





// --------------------------------
// --- payable function process ---
//      1 - ensure that the user has attached at least on yoctoNAER for the storage cost and security reasons like avoiding the DDOS attack on the contract by making sure that the caller has enough amount to call this and is not an intruder
//      2 - then calculate the storage used in u64 bits or 8 bytes maximum (usize on 64 bits arch system) of mutating the state of the contract like mutating any collection inside the contract struct like the total size of a new entry added inside the collection or the total size of the removed entries
//      3 - finally call refund_deposit() method to calculate the total costs for that bytes and refund to the caller any execess if there was an attached which was larger than the total storage cost or any removal entry process which will free up some storage which we must refund the caller based on the freed up storage bytes  



// NOTE - define the trait and struct methods as &mut self if you want to mutate their fields
// NOTE - &self means all fields in struct are in their borrowed form thus we have a shared reference of them inside the lifetime of the struct object also we'll have a valid lifetime for object after calling its methods in current scope which won't be moved by first call
// NOTE - inside all smart contracts are methods which are transactions and in order to call them from other contracts we have to use cross contract call promise feature by defining a trait and bound it to #[ext_contract()] proc macro attribute inside the caller contract; by doing this we're extending the receiver_id's contract actor (a hypothetical contract name of course since the method that we've defined inside the trait is already defined on the receiver_id's contract actor) interface which contains a method that must be scheduled to call on the receiver_id's contract actor inside the caller contract actor account and get the result of it inside the callback method of the caller contract
// NOTE - gas fee is the computational fee paied as raward to validators by attaching them (in gas units) in scheduling function calls in which they mutate the state of the contract which face us cpu usage costs; and also the remaining deposit will get pay back as a refund to the caller by the near protocol
// NOTE - deposit or amount must be attached (in yocto Ⓝ (1e24) or near) for scheduling payable function calls based on storages they've used by mutating the state of the contract on chain like updating a collection field inside the contract struct and we have to get pay back the remaining deposit as a refund to the caller and that's what the refund_deposit() function does
// NOTE - if a contract method mutate the state like adding a data into a collection field inside the contract struct; the method must be a payable method (we need to tell the caller attach deposit to cover the cost) and we have to calculate the storage used for updating the contract state inside the function to tell the caller deposit based on the used storage in bytes (like the total size of the new entry inside a collection) then refund the caller with the extra tokens he/she attached
// NOTE - every cross contract calls for communicating between contract actor accounts in cross sharding pattern takes up cpu usage and network laoding costs which forces us to attach gas units in the contract method call in which the cross contract call method is calling to pass it through the calling of the cross contract call method
// NOTE - a payable method has &mut self as its first param and all calculated storage must of type u64 bits or 8 bytes maximum length (64 bits arch system usize) 
// NOTE - caller in payable methods must deposit one yocto Ⓝ (1e24) for security purposes like always make sure that the user has some $NEAR in order to call this means only those one who have $NEARS can call this method to avoid DDOS attack on this method
// NOTE - we'll give the old owner of the token whatever is left from the total royalties at the end and he/she will get paid more than the other owners
// NOTE - royalty field is the hashmap of account_ids and their royalty percentage value to calculate their total payout
// NOTE - Payout instance has the hashmap field which contains account_ids and their payout balance in u128 in $NEAR 
// NOTE - on first sell of the NFT the onwer (minter) royalty payout won't be what is specified inside the royalty object and will be whatever is left after paying out collaborator (like the profit of market contract actor account itself) or charity account_ids cause we're checking that if the owner was inside the royalty object just pass! and he/she sould get paid on second sell
// NOTE - on second sell the owner (minter) royalty payout will be exactly what is specified inside the royalty object since the owner of the transferred token will not be the minter on second sell thus inside the iteration we'll calculate the minter payout and the old owner will get whatever is left outside the iteration after payingout other owners or collaborator (like the profit of market contract actor account itself) or charity account_ids  
// NOTE - if a method of the contract is going to mutate the state of the contract the first param of that method must be &mut self and it can be a none payable method like private method
















// ---------------------------------------
//   NFT CORE STANDARD INTERFACE METHODS
// ---------------------------------------
pub trait NoneFungibleTokenCore{ //-- defining an object safe trait for NFT core queries, we'll implement this for any contract that wants to interact with NFT core queries - object safe traits are not bounded to trait Sized thus they won't return Self or have generic params in its methods if so then some space should have been allocated inside the memory for Self or that generic param and it will no longer an abstract type

    fn nft_payout(&self, token_id: TokenId, balance: U128, max_len_payout: u32) -> Payout; //-- since we want to do some calculation without mutating the state of the contract and its fields thus there is no need to define the first param as &mut self and bind the method to #[payabele] proc macro attribute
    fn nft_transfer_payout(&mut self, receiver_id: AccountId, token_id: TokenId, approval_id: u64, memo: Option<String>, balance: U128, max_len_payout: u32) -> Payout; //-- transferring an NFT from the current owner to a receiver_id which usually will be done by selling the NFT by the owner, if an approval_id was passed means that an account_id except the owner can also transfer this NFT on behalf of the owner like a marketplace account; this method is useful for royalty since we'll pay the owner based on the royalty percentage amount and returns the payout object to the marketplace that should be paid the previous owner based on the passed in balance (the amount of the NFT which has sold on marketplace) 

}


#[near_bindgen] //-- implementing the #[near_bindgen] proc macro attribute on `NFTContract` struct to compile all its methods to wasm so we can call them in near cli
impl NoneFungibleTokenCore for NFTContract{ //-- implementing the NoneFungibleTokenCore trait for our main `NFTContract` struct to extend its interface; bounding the mentioned trait to the `NFTContract` struct to query NFT core (nft_* methods standards) infos


    ////////////////////////////////////////////
    ///// CALCULATING THE ROYALTY PAYOUTS //////
    ////////////////////////////////////////////

    fn nft_payout(&self, token_id: TokenId, balance: U128, max_len_payout: u32) -> Payout{ //-- balance is the amount that the buyer has paid for the NFT or the seller must get for his/her NFT at the end since all royalties must get paid first - this method doesn't transfer the NFT but only calculates the total payout in $NEAR for an NFT based on the passed in balance (the amount of the NFT which has been sold on marketplace) which must be paid by the marketplace to the account_ids (all the NFT owners or charity account_ids must get paid per each sell or transfer, also the old owner which can be the main owner or the minter or creator on second sell must get paid at the end which will have the more payout than the other owners) each time a buyer pays for that NFT
        
        match self.tokens_by_id.get(&token_id){ //-- getting the token object related to the token_id (passed by reference to borrow it) if there is some token object from the self.tokens_by_id LookupMap
            Some(token) => { //-- if there was some token found with this token_id
                let token_current_owner = token.owner_id; //-- getting the token current owner_id
                let mut total_perpetual = 0; //-- keeping track of the total perpetual royalties
                let balacnce_u128 = u128::from(balance); //-- getting the u128 type of the balance or the token amount
                let mut payout_object = Payout{ //-- creating the payout object
                    payout: HashMap::new() //-- an empty hashmap to keep track of the payout for each account_id or owner_id
                };
                let royalty = token.royalty; //-- getting the royalty hashmap of the token to calculate the payout for each owner_id based on their royalty percentage value
                if royalty.len() as u32 > max_len_payout{ //-- we're making sure that are not paying out to too many account_ids at the same time - if there was too many choosed for payout, gas fee will limit this condition since we may run out of gas fee by transferring $NEARs with a single attached gas to resolve_purchase() callback method inside the market contract actor account 
                    env::panic_str("Marketplace Can't Payout to That Many Receivers; 100 Receivers Max"); //-- &str allocates low cost storage than the String which will get usize (usize is 64 bits or 24 bytes on 64 bits arch) * 3 (pointer, len, capacity) bytes; cause it's just the size of the str itself which is the total length of its utf8 bytes array on either stack, heap or binary which is equals to its length of utf8 bytes and due to its unknown size at compile time we must borrow it by taking a pointer to its location
                }
                for (owner_id, royalty_percentage_value) in royalty.iter(){
                    let royalty_object_owner_id = owner_id.clone(); //-- we're cloning the owner_id each time since it'll move in each iteration by passing it through the insert() method of payout object
                    if royalty_object_owner_id != token_current_owner{ //-- if the key isn't the owner we'll add it into the payout object hashmap with its calculated payout using royalty_to_payout() method cause we'll add the current owner (the minter or creator at second sell) at the end
                        let calculated_payout_from_royalty_value_and_nft_amount = royalty_to_payout(*royalty_percentage_value, balacnce_u128); //-- calculating the payout for an owner_id based on his/her royalty percentage value and the amount of the NFT
                        payout_object.payout.insert(royalty_object_owner_id, calculated_payout_from_royalty_value_and_nft_amount); //-- inserting the calculated payout related to an owner_id into the payout hashmap object - first param of this method is &mut self since we want to mutate the hashmap and have a valid lifetime of the payout object also after calling this method so we can call other method of the payout hashmap; actually by doing this we're borrowing the instance and its fields to have it in later scopes if we want to call other methods of the instance 
                        total_perpetual += *royalty_percentage_value; //-- we have to dereference the royalty_percentage_value cause is of type &u32
                    }
                }
                let token_current_owner_royalty_percentage_value = 10000 - total_perpetual; //-- the royalty percentage value is equals to subtracting the total_perpetual percentage values from the 10000 since we gave 100 % a value of 10000 - we'll give the owner of the token whatever is left from the total_perpetual royalties 
                let token_current_owner_payout = royalty_to_payout(token_current_owner_royalty_percentage_value, balacnce_u128); //-- calculating the total payout for the current owner of the token or the minter at second sell 
                payout_object.payout.insert(token_current_owner, token_current_owner_payout); //-- inserting the payout of the token_current_owner into the payout hashmap object
                payout_object
            },
            None => { //-- means we found no token with the passed in token_id
                env::panic_str("Found No Token"); //-- &str allocates low cost storage than the String which will get usize (usize is 64 bits or 24 bytes on 64 bits arch) * 3 (pointer, len, capacity) bytes; cause it's just the size of the str itself which is the total length of its utf8 bytes array on either stack, heap or binary which is equals to its length of utf8 bytes and due to its unknown size at compile time we must borrow it by taking a pointer to its location
            },
        }
    
    }
    

    //////////////////////////////////////////////////////////////////////
    ///// TRANSFERRING THE NFT THEN CALCULATING THE ROYALTY PAYOUTS //////
    //////////////////////////////////////////////////////////////////////

    #[payable] //-- means the following would be a payable method and the caller must pay for that and must get pay back the remaining deposit or any excess that is unused at the end by refunding the caller account by our contract (something like refund_deposit() method) or the NEAR protocol - we should bind the #[near_bindgen] proc macro attribute to the contract struct in order to use this proc macro attribute
    fn nft_transfer_payout(&mut self, receiver_id: AccountId, token_id: TokenId, approval_id: u64, memo: Option<String>, balance: U128, max_len_payout: u32) -> Payout{ //-- balance is the amount that the buyer has paid for the NFT or the seller must get for his/her NFT at the end since all royalties must get paid first - this method transfers the NFT exactly like nft_transfer() method but calculates the total payout in $NEAR for an NFT based on the passed in balance (the amount of the NFT which has been sold on marketplace) which must be paid by the marketplace to the account_ids (all the NFT owners or charity account_ids must get paid per each sell or transfer, also the old owner which can be the main owner or the minter or creator on second sell must get paid at the end which will have the more payout than the other owners) each time a buyer pays for that NFT
        
        // -------------------------------------------------------------------------------
        // -------------------------------------------------------------------------------
        // NOTE - every old owner which is not inside the perpetual royalty object of the token can only be paid only once since the old owner will get paid at the end
        // NOTE - per each NFT sell all collaborator (like the profit of market contract actor account itself)s will be paid based on their royalty percentage 
        // NOTE - we've cloned the owner_id of the transferred_token in refund_approve_account_ids() 
        //        method to prevent it from moving cause we'll use it to get the old owner for payout ops
        // NOTE - we have to first transfer the token and refund the owner for releasing 
        //        the storage for approval account_ids then calculate the payout for all 
        //        NFT owners or charity account_ids as the pervious method
        // utils::panic_not_self(); //-- the caller of this method must be the owner of the contract means that only the owner of the contract can transfer NFT
        utils::panic_one_yocto(); //-- ensuring that the user has attached exactly one yocto Ⓝ (1e24) to the call to pay for the storage and security reasons (only those caller that have at least 1 yocto Ⓝ (1e24) can call this method; by doing this we're avoiding DDOS attack on the contract) on the contract by forcing the users to sign the transaction with his/her full access key which will redirect them to the NEAR wallet; we'll refund any excess amount from the storage later after calculating the required storage cost
        let sender_id = env::predecessor_account_id(); //-- getting the predecessor_account_id which is the previous contract actor account and the last (current) caller of this method
        let transferred_token = self.internal_transfer(&sender_id, &receiver_id, &token_id, Some(approval_id), Some(balance), memo); //-- transferring the NFT from either the sender_id's or an approved account_id's contract actor to the receiver_id's contract actor and return the transferred token info object - we might have no approval_id and make this call a simple transfer from the owner only
        let deposited_amount = env::attached_deposit(); //-- getting the attached deposited from the caller - here we can do some chunky shity job to tranfer the deposit to another contract actor account :)
        utils::refund_approve_account_ids(transferred_token.owner_id.clone(), &transferred_token.approved_account_ids); //-- refunding the owner for releasing the storage used up by the approved account ids (cause he/she doesn't own any allocated storage for approval_account_ids for the token cause the token has been transferred to another one) based on the transferred token object since the owner paid for it when he/she was minting the NFT inside the nft_mint() method - passing the approved_account_ids in its borrowed from by taking a reference to its location using & to prevent from moving and losing its ownership so we can have it in later scopes
        // -------------------------------------------------------------------------------
        // -------------------------------------------------------------------------------

        let token_old_owner = transferred_token.owner_id; //-- getting the transferred token old owner_id
        let mut total_perpetual = 0; //-- keeping track of the total perpetual royalties
        let balacnce_u128 = u128::from(balance); //-- getting the u128 type of the balance or the transferred token amount
        let mut payout_object = Payout{ //-- creating the payout object
            payout: HashMap::new() //-- an empty hashmap to keep track of the payout for each account_id or owner_id
        };
        let royalty = transferred_token.royalty; //-- getting the royalty hashmap of the transferred token to calculate the payout for each owner_id based on their royalty percentage value
        if royalty.len() as u32 > max_len_payout{ //-- we're making sure that are not paying out to too many account_ids at the same time - if there was too many choosed for payout, gas fee will limit this condition since we may run out of gas fee by transferring $NEARs with a single attached gas to resolve_purchase() callback method inside the market contract actor account 
            env::panic_str("Marketplace Can't Payout to That Many Receivers; 100 Receivers Max"); //-- &str allocates low cost storage than the String which will get usize (usize is 64 bits or 24 bytes on 64 bits arch) * 3 (pointer, len, capacity) bytes; cause it's just the size of the str itself which is the total length of its utf8 bytes array on either stack, heap or binary which is equals to its length of utf8 bytes and due to its unknown size at compile time we must borrow it by taking a pointer to its location
        }
        for (owner_id, royalty_percentage_value) in royalty.iter(){ // NOTE - if we use iter() method then we must dereference the key and the value to get their value cause iter() method element are borrowed type of keys and values or pointers to the key and value location (&'a key, &'a value) with a valid lifetime 
            let royalty_object_owner_id = owner_id.clone(); //-- we're cloning the owner_id each time since it'll move in each iteration by passing it through the insert() method of payout object since the insert() method get the type itself and the type will be moved by passing it into this method
            if royalty_object_owner_id != token_old_owner{ //-- if the key isn't the owner we'll add it into the payout object hashmap with its calculated payout using royalty_to_payout() method cause we'll add the old owner (the minter or creator at second sell) at the end
                let calculated_payout_from_royalty_value_and_nft_amount = royalty_to_payout(*royalty_percentage_value, balacnce_u128); //-- calculating the payout for an owner_id based on his/her royalty percentage value and the amount of the transferred NFT
                payout_object.payout.insert(royalty_object_owner_id, calculated_payout_from_royalty_value_and_nft_amount); //-- inserting the calculated payout related to an owner_id into the payout hashmap object - first param of this method is &mut self since we want to mutate the hashmap and have a valid lifetime of the payout object also after calling this method so we can call other method of the payout hashmap; actually by doing this we're borrowing the instance and its fields to have it in later scopes if we want to call other methods of the instance 
                total_perpetual += *royalty_percentage_value; //-- we have to dereference the royalty_percentage_value cause is of type &u32
            }
        }
        let token_old_owner_royalty_percentage_value = 10000 - total_perpetual; //-- the royalty percentage value is equals to subtracting the total_perpetual percentage values from the 10000 since we gave 100 % a value of 10000 - we'll give the owner of the token whatever is left from the total_perpetual royalties after paying charity or collaborator (like the profit of market contract actor account itself) account_ids
        let token_old_owner_payout = royalty_to_payout(token_old_owner_royalty_percentage_value, balacnce_u128); //-- calculating the total payout for the old owner of the transferred token or the minter at second selll
        payout_object.payout.insert(token_old_owner, token_old_owner_payout); //-- inserting the payout of the token_old_owner into the payout hashmap object
        payout_object

    }


    
}
