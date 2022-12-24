






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




// NOTE - we don't need to calculate the total storages used inside the nft_transfer() and nft_transfer_call() methods even though we've forced the caller to attached 1 yocto Ⓝ (1e24) cause in those methods we first remove the token from self.tokens_per_owner field then add the token to self.tokens_per_owner with a new owner_id therefore the amount of used storage would be 0 in total since the remvoing process will make some space for adding process which the total usded bytes will be the same as before 
// NOTE - for calling none intranal contract methods like private, init and payable methods we have to attach the #[near_bindgen] proc macro attribute on the contract impl and its traits impl
// NOTE - based on the above rule since nft_resolve_transfer() method is a private method (only the owner of the contract can call it) in order to cal&l it from other method inside this contract (can't call it from the cli unless the caller must be the contract owner) we have to attach the #[near_bindgen] proc macro attribute on top of the impl NoneFungibleTokenResolver for <Contract> 
// NOTE - since we've extended the current contract actor by defining the NoneFungibleTokenResolver trait for the cross contract call nft_resolve_transfer() method u&mut &mut sing #[ext_contract()] proc macro we can't use the same trait to attach the #[near_bindgen] proc macro to have wasm compiled of nft_resolve_transfer() callback method thus we have to define another trait with the same name without the #[ext_contract()] proc macro attribute in order to attach the #[near_bindgen] proc macro attribute on top of it
// NOTE - here we implement nft core queries trait like transferring and viewing the nft info for any contract struct by implementing it on any given contract
// NOTE - the reason behind &the trait impl is that we can bound it to any given contract struct to use the nft methods on that contract struct  
// NOTE - cross contract calls or calling between contract actors or shards is done by defining a trait which represents the contract's interface for an existing contract to define cross contract call promise methods inside the implemented trait and overriding them inside the impl <Trait_Interface> for <Contract> block
// NOTE - in order to call a method from another contract actor we need to schedule a Promise object on the second account_id inside the first contract actor to create a pending transaction which is a promise (future object) ActionReceipt object or the async message to pass it using actor address through the mpsc channel to the second contract actor
// NOTE - in order to call an interface method which has implemented on an instance of a struct the interface or the trait must be imported inside the crate where the instance of the struct is
// NOTE - defining cross contract call promise methods of a contract is done by extending the contract interface using impl <Trait_Interface> for <Contract> block cause #[ext_contract(some_name)] can only be used on traits
// NOTE - if a function requires a deposit, we need a full access key of the user to sign that transaction which will redirect them to the NEAR wallet
// NOTE - gas fee is the computational fee paied as raward to validators by attaching them (in gas units) in scheduling function calls in which they mutate the state of the contract which face us cpu usage costs; and also the remaining deposit will get pay back as a refund to the caller by the near protocol
// NOTE - deposit or amount is the cost of the method and must be attached (in yocto Ⓝ (1e24) or near) for scheduling payable function calls based on storages they've used by mutating the state of the contract on chain like updating a collection field inside the contract struct and we have to get pay back the remaining deposit as a refund to the caller and that's what the refund_deposit() function does
// NOTE - if a contract method mutate the state like adding a data into a collection field inside the contract struct; the method must be a payable method (we need to tell the caller attach deposit to cover the cost) and we have to calculate the storage used for updating the contract state inside the function to tell the caller deposit based on the used storage in bytes (like the total size of the new entry inside a collection) then refund the caller with the extra tokens he/she attached
// NOTE - a payable method has &mut self as its first param and all calculated storage must of type u64 bits or 8 bytes maximum length (64 bits arch system usize)
// NOTE - caller in payable methods must deposit one yocto Ⓝ (1e24) for security purposes like always make sure that the user has some $NEAR in order to call this means only those one who have $NEARS can call this method to avoid DDOS attack on this method
// NOTE - a payable method can be used to pay the storage cost, the escrow price or the gas fee and the excess will be refunded by the contract method or the NEAR protocol
// NOTE - gas fee is the computational cost which must be paid if we’re doing cross contract call or moving between shards and actor cause this action will cost some cpu usage performance and must be attached separately in its related call from the cli 
// NOTE - amount or deposit is the cost of the payable function which can be either the cost of the storage usage for mutating contract or the cost of some donation or escrow ops
// NOTE - every cross contract calls for communicating between contract actor accounts in cross sharding pattern takes up cpu usage and network laoding costs which forces us to attach gas units in the contract method call in which the cross contract call method is calling to pass it through the calling of the cross contract call method
// NOTE - in near version 3 `ext_contract` proc macro attribute takes a Rust Trait and converts it to a module with static methods and each of these static methods takes positional arguments defined by the Trait then the last three arguments the receiver_id, the attached deposit and the amount of gas are used behind the scenes and returns a new Promise
// NOTE - we have to log the event also in cross contract calls coming from other contract actor accounts like marketplace like inside the nft_transfer_call() method
// NOTE - a view method can also force the user to attach yocto Ⓝ (1e24) to the call to prevent contract from DDOSing
// NOTE - if a method of the contract is going to mutate the state of the contract the first param of that method must be &mut self and it can be a none payable method like private method
// NOTE - in order to get the result of the cross contract call method we have to define a method inside the sender's or the caller's contract actor account by extending its contract struct interface by defining a trait which contains the definition of the callback method
// NOTE - in order to call and schedule a promise or future object method from other contract actor account we have to define a trait and bound it to #[ext_contract()] proc macro which contains the method signature of the second contract actor account finally we can call in here and catch the the result of the scheduled promise of future object using the NEAR cross contract call syntax
// NOTE - callback methods inside the caller contract actor account must be defined private since no one except the caller contract can resolve the result of the executed promise scheduled in cross contract inside the receiver contract actor account, thus they must be defined as private methods   























// ----------------------------------------
//     CROSS CONTRACT CALLS' INTERFACES
// ----------------------------------------
#[ext_contract(extend_receiver_contract_for_none_fungible_token)] //-- extend_receiver_contract_for_none_fungible_token name that we passed in #[ext_contract()] proc macro is the name of the contract (a hypothetical contract name of course) that we're extending its interface for cross contract call and creating transaction which is a promise (future object) ActionReceipt object and means we want to call the following methods inside that contract which contains a transaction which is a promise (future object) ActionReceipt object that must be executed later
trait NoneFungibleTokenReceiver{ //-- this trait which contains the cross conract call methods will extend the interface of the receiver_id's contract actor with a name inside the #[ext_contract()] proc macro which specifies the extended interface contract name on this contract 

    /////
    /////// ➔ following method must be called and executed inside the receiver_id's contract actor from this contract actor account therefore it'll take a param called account_id which is the one who should call this method on his/her contract actor account and must be the owner of his/her contract
    /////
    fn nft_on_transfer(&mut self, sender_id: AccountId, previous_owner_id: AccountId, token_id: TokenId, msg: String) -> Promise; //-- this method will be used for cross contract call on the receiver_id's contract actor (which must be implemented on the receiver_id's contract actor) once the nft_transfer_call() method is called and will return true if the token should be returned back to the sender

}


















// ---------------------------------------
//   NFT CORE STANDARD INTERFACE METHODS
// ---------------------------------------
pub trait NoneFungibleTokenCore{ //-- defining a trait for nft core queries, we'll implement this for any contract that wants to interact with nft core queries - this is not object safe trait cause we have generic params in its methods
    
    fn nft_transfer(&mut self, receiver_id: AccountId, token_id: TokenId, approval_id: Option<u64>, memo: Option<String>); //-- transferring an nft from the current owner to a receiver_id which usually will be done by selling the NFT by the owner, if an approval_id was passed means that an account_id except the owner can also transfer this NFT on behalf of the owner like a marketplace account; this method is useful for airdrops since we don't pay the owner the royalty after transferring the NFT
    fn nft_transfer_call(&mut self, receiver_id: AccountId, token_id: TokenId, approval_id: Option<u64>, memo: Option<String>, msg: String) -> PromiseOrValue<bool>; //-- transferring an nft to a receiver_id and will return true if the token was successfully transferred from the sender_id's contract actor to the receiver_id's contract actor - we'll call a method on the receiver_id's contract actor by scheduling a transaction which is a promise (future object) ActionReceipt object on this contract which is the sender_id's contract actor - except NFT owner or charity people account_ids with their approval_id can also transfer an NFT on behalf of the owner
    fn nft_token(&self, token_id: TokenId) -> Option<JsonToken>; //-- getting the information about an nft using its id
    fn nft_reveal(&mut self, nfts: Vec<NftRevealInfo>, collection_creator_id: AccountId) -> Vec<Option<JsonToken>>;
    fn nft_update(&mut self, token_id: TokenId, metadata: TokenMetadata, perpetual_royalties: Option<HashMap<AccountId, u32>>) -> Option<JsonToken>;

}


#[near_bindgen] //-- implementing the #[near_bindgen] proc macro attribute on the trait implementation for the `NFTContract` struct in order to have a compiled trait methods for this contract struct so we can call it from the near cli
impl NoneFungibleTokenCore for NFTContract{ //-- implementing the NoneFungibleTokenMetadata trait for our main `NFTContract` struct (or any contract); bounding the mentioned trait to the `NFTContract` struct to query nft metadata infos

    #[payable] //-- means the following would be a payable method and the caller must pay for that and must get pay back the remaining deposit or any excess that is unused at the end by refunding the caller account by our contract (something like refund_deposit() method) or the NEAR protocol - we should bind the #[near_bindgen] proc macro attribute to the contract struct in order to use this proc macro attribute  
    fn nft_transfer(&mut self, receiver_id: AccountId, token_id: TokenId, approval_id: Option<u64>, memo: Option<String>){ //-- we've defined the self to be mutable and borrowed cause we want to mutate some fields and have the isntance with a valid lifetime after calling this method on it

        ////
        /////// ➔ since an evil might call this method thus to transfer an NFT the person who wants to do this must be either an approved account_id or the owner of the NFT itself 
        ////
        // utils::panic_not_self(); //-- the caller of this method must be the owner of the contract means that only the owner of the contract can transfer NFT
        utils::panic_one_yocto(); //-- ensuring that the user has attached exactly one yocto Ⓝ (1e24) to the call to pay for the storage and security reasons (only those caller that have at least 1 yocto Ⓝ (1e24) can call this method; by doing this we're avoiding DDOS attack on the contract) on the contract by forcing the users to sign the transaction with his/her full access key which will redirect them to the NEAR wallet; we'll refund any excess amount from the storage later after calculating the required storage cost
        let sender_id = env::predecessor_account_id(); //-- getting the predecessor_account_id which is the previous contract actor account and the last (current) caller of this method
        let transferred_token = self.internal_transfer(&sender_id, &receiver_id, &token_id, approval_id, None, memo); //-- transferring the NFT from either the sender_id's or an approved account_id's contract actor to the receiver_id's contract actor and return the transferred token info object - we might have no approval_id and make this call a simple transfer from the owner only
        let deposited_amount = env::attached_deposit(); //-- getting the attached deposited from the caller - here we can do some chunky shity job to tranfer the deposit to another contract actor account :)
        utils::refund_approve_account_ids(transferred_token.owner_id, &transferred_token.approved_account_ids); //-- refunding the owner for releasing the storage used up by the approved account ids (cause he/she doesn't own any allocated storage for approval_account_ids for the token cause the token has been transferred to another one) based on the transferred token object since the owner paid for it when he/she was minting the NFT inside the nft_mint() method - passing the approved_account_ids in its borrowed from by taking a reference to its location using & to prevent from moving and losing its ownership so we can have it in later scopes

    }

    #[payable] //-- means the following would be a payable method and the caller must pay for that and must get pay back the remaining deposit or any excess that is unused at the end by refunding the caller account by our contract (something like refund_deposit() method) or the NEAR protocol - we should bind the #[near_bindgen] proc macro attribute to the contract struct in order to use this proc macro attribute 
    fn nft_transfer_call(&mut self, receiver_id: AccountId, token_id: TokenId, approval_id: Option<u64>, memo: Option<String>, msg: String) -> PromiseOrValue<bool>{ //-- returning a promise or a value which can be either true if the token was transferred from the sender_id's contract actor to the receiver_id's contract actor or false otherwise - we've defined the self to be mutable and borrowed cause we want to mutate some fields and have the isntance with a valid lifetime after calling this method on it
        
        /*

            -----------------------------------------------------------------------------
            
            1 - a sender invokes the nft_transfer_call method to send an NFT to a receiver
            2 - the nft_transfer_call method transfers the NFT from sender to receiver
            3 - after the transfer a cross contract call which is a transaction which is a promise (future object) ActionReceipt object is scheduled 
                    an ActionReceipt is created to call the nft_on_transfer method on the receiver contract
                    a callback nft_resolve_transfer is registered on sender_id's contract actor by creating a pending ActionReceipt
            4 - on the next block either in a same shard or other shard, the nft_on_transfer method is executed on the receiver_id's contract actor and a DataReceipt is created
            5 - on the next block either in a same shard or other shard, the pending ActionReceipt from above is ready and the nft_resolve_transfer callback is executed
        

            for every cross contract calls we have to extend the interface of our contract struct by impl a trait for that to define the cross contract call promise methods 
                nft_transfer_call()     ----- inside the sender_id's contract actor
                nft_on_transfer()       ----- inside the receiver_id's contract actor - it must already be defined in there so we can schedule it in caller contract to be executed on receiver_id's contract actor account  
                nft_resolve_transfer()  ----- inside the sender_id's contract actor to solve and fill the pending promise ActionReceipt object with the promise DataReceipt object coming from the receiver_id's contract actor account

        
            nft_transfer_call()    on [sender_id's contract actor]   -> true if the token was transferred from the sender_id's contract actor - schedule the nft_on_transfer() cross contract call promise method to be executed later on receiver_id's contract actor
            nft_resolve_transfer() on [sender_id's contract actor]   -> boolean based on the result of the nft_on_transfer() cross contract call promise method - get the result of the scheduled promise inside this method by solving it using .then() method
            nft_on_transfer()      on [receiver_id's contract actor] -> true if the token should be returned back to the sender otherwise false - execute this promise on receiver_id's contract actor
        
            -----------------------------------------------------------------------------

        */

        utils::panic_one_yocto(); //-- ensuring that the user has attached exactly one yocto Ⓝ (1e24) to the call to pay for the storage on the contract by forcing the users to sign the transaction with his/her full access key which will redirect them to the NEAR wallet; we'll refund any excess amount from the storage later after calculating the required storage cost
        let sender_id = env::predecessor_account_id(); //-- getting the predecessor_account_id which is the previous contract actor account and the last (current) caller of this method
        let transferred_token = self.internal_transfer(&sender_id, &receiver_id, &token_id, approval_id, None, memo.clone()); //-- transferring the NFT from the sender_id's contract actor to the receiver_id's contract actor and return the transferred token info object
        let auth_sender_id = sender_id.clone(); //-- cloning the sender_id to prevent from moving since we can't dereference a shared reference that doesn't implement the Copy trait
        let authorized_id = if sender_id != transferred_token.owner_id{ //-- if the sender_id or the caller (like a marketplace account_id) wasn't the owner of the transferred_token, we set the authorized_id equal to the sender_id since the sender or the caller like the marketplace account_id (since this method can be called from the marketplace contract actor account using cross contract call) can't be still the owner of the transferred token thus the current sender is an approved account_id
            Some(auth_sender_id) 
        } else{
            None //-- the authorized_id must be None since we have no approved account which means that sender is the owner of the transferred token (like the sender is the caller which is the marketplace which made a cross contract call which is not the real owner of the transferred token cause it has to be the approved account) thus can't be an approved account
        };
        //////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
        ////////////// ➔ defaulting GAS weight to 1, no attached deposit, and static GAS equal to the GAS for nft_on_transfer
        //////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
        extend_receiver_contract_for_none_fungible_token::ext(receiver_id.clone()) //--  we're cloning the receiver_id to avoid moving cause we want to use it inside the nft_resolve_transfer() method - the account_id that this method must be called and executed inside since the account_id param is the one who is responsible for executing this call like the market contract actor account
            .with_attached_deposit(NO_DEPOSIT) //-- no deposit is required from the caller for calling nft_on_transfer() cross contract call promise method since this method doesn't require any deposit amount
            .with_static_gas(GAS_FOR_NFT_TRANSFER_CALL) //-- the total gas fee which will be deposited in yocto Ⓝ (1e24) from the caller wallet for this transaction cross contract call
            .nft_on_transfer( //-- initiating the receiver's corss contract call by creating a transaction which is a promise (future object) ActionReceipt object which returns obviously a promise or a future object which contains an async message including the data coming from the receiver_id's contract actor once it gets executed - calling the nft_on_transfer() cross contract call promise method on the receiver side from the extended receiver_id's contract actor interface which is `extend_receiver_contract_for_none_fungible_token`
                sender_id, 
                transferred_token.owner_id.clone(), 
                token_id.clone(), 
                msg
            ).then( //-- wait for the scheduled transaction which is a promise (future object) ActionReceipt object on the receiver_id's contract actor to finish executing to resolve it using .then() method
                ////////////
                /////// ➔ by default ext() method will be attached to the contract struct annotated with #[near_bindgen] which avoids the requirement to re-define the interface with #[ext_contract] 
                ///////    and the method that will be attached to the struct is the same as ext_contract as ext(..) so we can call Self::ext(...) which remove the need to redefine interfaces twice
                /////// ➔ defaulting GAS weight to 1, no attached deposit, and static GAS equal to the GAS for resolve transfer
                ////////////
                Self::ext(env::current_account_id()) //-- the account_id that this method must be called and executed inside which is the current_account_id() and is the one who owns this contract - account_id param is the one who is responsible for executing this call which is the owner of this contract which is the NFT owner
                    .with_attached_deposit(NO_DEPOSIT) //-- no deposit is required from the caller for calling the nft_resolve_transfer() callback method since this method doesn't require any deposit amount
                    .with_static_gas(GAS_FOR_RESOLVE_TRANSFER) //-- the total gas fee required for calling the callback method which has taken from the attached deposited (contract budget) when the caller called the nft_transfer_call() method
                    .nft_resolve_transfer( //-- calling nft_resolve_transfer() method from the extended interface of the current contract actor (our own contract) which is the `extend_this_contract` contract; since this is a private method only the owner of the this contract can call it means the caller must be the signer or the one who initiated, owned and signed the contract or the account of the contract itself or the sender him/her-self to mutate the state of the contract on chain thus we have to pass the current_id's or the sender_id's contract actor which is the owner of this contract actor; since callback methods are private thus the caller of them must be the owner of the contract
                        authorized_id,
                        transferred_token.owner_id.clone(), 
                        receiver_id,
                        token_id, 
                        transferred_token.approved_account_ids, //-- passing the previous token approved_account_ids hashmap to nft_resolve_transfer() callback method cause we'll refund the owner inside the callback method since there would be still the possibility that the transfer gets reverted due to the result of nft_on_transfer() method thus we must keep track of what the approvals (those account_id which have access to transfer the NFT on behalf of the owner) were before and after transferring the NFT 
                        memo
                    )

            )
            .into() //-- returning a boolean

    }

    fn nft_token(&self, token_id: TokenId) -> Option<JsonToken>{
        
        if let Some(token) = self.tokens_by_id.get(&token_id){ //-- if there is some token object (contains owner_id info) with this id in the tokens_by_id collection
            let metadata = self.token_metadata_by_id.get(&token_id).unwrap(); //-- getting the metadata for this token using its id - we're passing the borrowed type of the token_id in order to have it inside other scope and its lifetime be valid
            Some(JsonToken{
                token_id,
                creator_id: token.creator_id,
                owner_id: token.owner_id,
                metadata,
                approved_account_ids: token.approved_account_ids, //-- all approved account_ids with their approval id
                royalty: token.royalty, //-- the royalties hashmap took from the token object
            })
        } else{ //-- if there wasn't a token with this id in tokens_by_id collection we return None
            None
        }

    }

    #[private] //-- means the following would be a private method and the caller or the predecessor_account_id which is the previous contract actor account and the last (current) caller of this method to mutate the state of the contract on chain must be the signer (who initiated and signed the contract) and the owner of this contract
    fn nft_reveal(&mut self, nfts: Vec<NftRevealInfo>, collection_creator_id: AccountId) -> Vec<Option<JsonToken>>{
        ////
        /////// ➔ only the owner of the contract can call this method since this method must be called from the backend therefore we have the keys of the owner of the contract in there and we can call this method with them
        //// 
        let mut revealed_nfts = vec![];
        for nft in nfts{
            let token = if let Some(nft) = self.tokens_by_id.get(&nft.token_id){
                nft
            } else{
                env::panic_str("SMARTIES : No NFT Minted With This Id");
            };
            if token.creator_id == collection_creator_id.clone(){ //-- cloning to prevent from moving in each iteration
                self.token_metadata_by_id.insert(&nft.token_id, &nft.metadata); //-- insert will update the value of the given key if there was any key - mutate the token metadata if there was any token already in there, that's what the insert() method does!
                //////////////////////////////////////////////////////////////////////////////////////////
                ////////////////// CONSTRUCTING THE MINT LOG AS PER THE EVENTS STANDARD //////////////////
                //////////////////////////////////////////////////////////////////////////////////////////
                let nft_reveal_log = EventLog{ //-- emitting the minting event
                    standard: NFT_STANDARD_NAME.to_string(), //-- the current standard
                    version: NFT_METADATA_SPEC.to_string(), //-- the version of the standard based on near announcement
                    event: EventLogVariant::NftReveal(vec![NftRevealLog{ //-- the data related with the minting event stored in a vector 
                        collection_creator_id: collection_creator_id.clone(), //-- cloning to prevent from moving in each iteration
                        owner_id: token.owner_id, // the owner of all the tokens that were minted; since it might be a collection minting
                        token_ids: vec![], //-- list of all minted token ids; since it might be a collection minting - cloning the token_id to have it in later scopes
                        done_at: env::block_timestamp(), //-- the timestamp that this method or transaction is done with executing 
                        memo: None, //-- the memo message which is None
                    }]),
                }; // NOTE - since we've implemented the Display trait for the EventLog struct thus we can convert the nft_reveal_log instance to string to log the nft minting event info at runtime
                env::log_str(&nft_reveal_log.to_string()); //-- format!() returns a String which takes 24 bytes storage, usize * 3 (pointer, len, capacity) bytes (usize is 64 bits or 24 bytes on 64 bits arch)
                let event_vector = self.token_events.entry(nft.token_id.clone()).or_insert(vec![]); //-- we have to clone the token_id since we want to use it to get all its owners
                event_vector.push(nft_reveal_log);
                //////////////////////////////////////////////////////////////////////////////////////////
                revealed_nfts.push(self.nft_token(nft.token_id));
            } else{
                env::panic_str("SMARTIES : Passed In Collection Creator Is Not The Creator Of The NFT");
            }
        }
        revealed_nfts //-- returning the revealed tokens
    }

    #[payable]
    fn nft_update(&mut self, token_id: TokenId, metadata: TokenMetadata, perpetual_royalties: Option<HashMap<AccountId, u32>>) -> Option<JsonToken>{
        assert_one_yocto(); //-- ensuring that the user has attached exactly one yocto Ⓝ (1e24) to the call to pay for the storage and security reasons (only those caller that have at least 1 yocto Ⓝ (1e24) can call this method; by doing this we're avoiding DDOS attack on the contract) on the contract by forcing the users to sign the transaction with his/her full access key which will redirect them to the NEAR wallet; we'll refund any excess amount from the storage later after calculating the required storage cost
        let caller  = env::predecessor_account_id(); //-- getting the caller of this method
        let token = self.tokens_by_id.get(&token_id);
        if let Some(token_info) = token{
            if token_info.creator_id == caller{
                let initial_storage_usage = env::storage_usage(); //-- storage_usage() method calculate current total storage usage as u64 bits or 8 bytes maximum (usize on 64 bits arch system) of this smart contract that this account would be paying for - measuring the initial storage being uses on the contract 
                let mut royalty = HashMap::new(); //-- creating an empty royalty hashmap to keep track of the royalty percentage value for each owner_id that is passed in into the nft_mint() method, the perpetual_royalties param
                match perpetual_royalties{ // NOTE - perpetual_royalties hashmap contains accounts that will get perpetual royalties whenever the token is sold, of course it has the owner or the minter or creator of the collection or the NFT in addition to some charity or collaborator (like the profit of market contract actor account itself) account_ids to get paid them and the minter will get paid on second sell
                    Some(royalties) => {
                        if royalties.len() >= 6{ //-- making sure that the length of the perpetual royalties is below 7 since we won't have enough gas fee to pay out that many people after selling the NFT and getting the payout object from the NFT contract which is deployed on the minter contract actor acctount 
                            env::panic_str("You Are Allowed To Add Only 6 Royalties Per Token Minting!"); //-- &str allocates low cost storage than the String which will get usize (usize is 64 bits or 24 bytes on 64 bits arch) * 3 (pointer, len, capacity) bytes; cause it's just the size of the str itself which is the total length of its utf8 bytes array on either stack, heap or binary which is equals to its length of utf8 bytes and due to its unknown size at compile time we must borrow it by taking a pointer to its location   
                        }
                        for (owner_id, royalty_percentage_value) in royalties{ //-- NOTE - no need to call iter() method on royalties hashmap since we only want to insert the key and the value of perpetual_royalties hashmap into the royalty hashmap thus we don't the borrowed type of key and value
                            royalty.insert(owner_id, royalty_percentage_value); //-- filling the royalty hashmap with the incoming perpetual royalties from the call
                        }
                    },
                    None => {
                        env::log_str("No Royalty Hashmap was Passed For Updating NFT"); //-- &str allocates low cost storage than the String which will get usize (usize is 64 bits or 24 bytes on 64 bits arch) * 3 (pointer, len, capacity) bytes; cause it's just the size of the str itself which is the total length of its utf8 bytes array on either stack, heap or binary which is equals to its length of utf8 bytes and due to its unknown size at compile time we must borrow it by taking a pointer to its location
                    }
                }
                
                let token = Token{
                    creator_id: token_info.creator_id,
                    owner_id: token_info.owner_id,
                    approved_account_ids: token_info.approved_account_ids, //-- creating an empty hashmap or {} for all approved account ids 
                    next_approval_id: token_info.next_approval_id, //-- next approval id must be started from 0 when we're minting the token
                    royalty, //-- a mapping between owner_ids or some charity or collaborator (like the profit of market contract actor account itself) account_ids and their royalty percentage value to calculate the payout later for each owner based on the NFT amount - the old owner which can be the main owner or the minter or creator on second sell will get paid at the end - total perpetual royalties 
                };


                self.tokens_by_id.insert(&token_id, &token);
                self.token_metadata_by_id.insert(&token_id, &metadata); //-- insert will update the value of the given key if there was any key - mutate the token metadata if there was any token already in there, that's what the insert() method does!
                
                //////////////////////////////////////////////////////////////////////////////////////////
                ////////////////// CONSTRUCTING THE MINT LOG AS PER THE EVENTS STANDARD //////////////////
                //////////////////////////////////////////////////////////////////////////////////////////
                let nft_update_log = EventLog{ //-- emitting the minting event
                    standard: NFT_STANDARD_NAME.to_string(), //-- the current standard
                    version: NFT_METADATA_SPEC.to_string(), //-- the version of the standard based on near announcement
                    event: EventLogVariant::NftUpdate(vec![NftUpdateLog{ //-- the data related with the minting event stored in a vector 
                        owner_id: token.owner_id.clone(), // the owner of all the tokens that were minted; since it might be a collection minting
                        token_ids: vec![token_id.clone()], //-- list of all minted token ids; since it might be a collection minting - cloning the token_id to have it in later scopes
                        done_at: env::block_timestamp(), //-- the timestamp that this method or transaction is done with executing 
                        memo: None, //-- the memo message which is None
                    }]),
                }; // NOTE - since we've implemented the Display trait for the EventLog struct thus we can convert the nft_update_log instance to string to log the nft minting event info at runtime
                env::log_str(&nft_update_log.to_string()); //-- format!() returns a String which takes 24 bytes storage, usize * 3 (pointer, len, capacity) bytes (usize is 64 bits or 24 bytes on 64 bits arch)
                let event_vector = self.token_events.entry(token_id.clone()).or_insert(vec![]); //-- we have to clone the token_id since we want to use it to get all its owners
                event_vector.push(nft_update_log);
                //////////////////////////////////////////////////////////////////////////////////////////
                
                let required_storage_in_bytes = env::storage_usage() - initial_storage_usage; // -- calculating the required storage in u64 bits or 8 bytes which is total used unitl now - the initial storage | this is the way that we're calculating the minting cost 
                refund_deposit(required_storage_in_bytes); //-- depositing some $NEARs based on used bytes in the contract and get pay back the remaining deposit or any excess that is unused at the end by refunding the caller account by our contract (something like refund_deposit() method) or the NEAR protocol; if the caller didn't attach enough it'll panic
                self.nft_token(token_id) //-- return the token info in JsonToken format
            } else{
                self.nft_token(token_id); //-- no update is doen and simply we return the old info of the NFT
                env::panic_str("Only Collection Creator Can Update NFT"); //-- &str allocates low cost storage than the String which will get usize (usize is 64 bits or 24 bytes on 64 bits arch) * 3 (pointer, len, capacity) bytes; cause it's just the size of the str itself which is the total length of its utf8 bytes array on either stack, heap or binary which is equals to its length of utf8 bytes and due to its unknown size at compile time we must borrow it by taking a pointer to its location
            }
        } else{
            env::panic_str("No NFT Found; Mint NFT First"); //-- &str allocates low cost storage than the String which will get usize (usize is 64 bits or 24 bytes on 64 bits arch) * 3 (pointer, len, capacity) bytes; cause it's just the size of the str itself which is the total length of its utf8 bytes array on either stack, heap or binary which is equals to its length of utf8 bytes and due to its unknown size at compile time we must borrow it by taking a pointer to its location
        }
    }
}
















// --------------------------------------------------------------
//    NFT RESOLVER INTERFACE CALLBACK METHODS IMPLEMENTATION
// --------------------------------------------------------------
trait NoneFungibleTokenResolver{ //-- this trait which contains the cross conract call methods will extend the interface of the current contract actor which is the sender_id's contract actor with a name inside the #[ext_contract()] proc macro which specifies the extended interface contract name
    
    ////
    /////// ➔ we'll use this method as a callback inside this contract to get the result of the cross contract call the nft_on_transfer() method which has been scheduled inside the nft_transfer_call() method to be executed on a receiver contract actor account
    //// 
    fn nft_resolve_transfer(&mut self, authorized_id: Option<AccountId>, owner_id: AccountId, receiver_id: AccountId, token_id: TokenId, approved_account_ids: HashMap<AccountId, u64>, memo: Option<String>) -> bool; //-- resolves the pending DataReceipt object of the created and scheduled promise on this contract of the cross contract call to the receiver contract, this is the callback from calling the nft_on_transfer() cross contract call promise method that we want to await on and solve it inside the nft_transfer_call() method which will analyze what happened in the cross contract call when nft_on_transfer was called as part of the nft_transfer_call method - we've passed the approval_account_ids in to this method so we can keep track of what the approvals (those account_id which have access to transfer the NFT on behalf of the owner) were before transferring the NFT

}


#[near_bindgen] //-- implementing the #[near_bindgen] proc macro attribute on the trait implementation for the `NFTContract` struct in order to have a compiled wasm trait methods for this contract struct so we can call it from the near cli
impl NoneFungibleTokenResolver for NFTContract{ //-- implementing the NonFungibleTokenResolver trait for our main `NFTContract` struct (or any contract); extending the `NFTContract` struct interface to define the body of cross contract call promise methods which is the callback method for the current or the sender contract actor to fill the pending promise ActionReceipt object with the promise DataReceipt object coming from the receiver_id's contract actor

    #[private] //-- means the following would be a private method and the caller or the predecessor_account_id which is the previous contract actor account and the last (current) caller of this method to mutate the state of the contract on chain must be the signer (who initiated and signed the contract) and the owner of this contract
    fn nft_resolve_transfer(&mut self, authorized_id: Option<AccountId>, owner_id: AccountId, receiver_id: AccountId, token_id: TokenId, approved_account_ids: HashMap<AccountId, u64>, memo: Option<String>) -> bool{ //-- since we're calling self.internal_remove_token_from_owner() and self.internal_add_token_to_owner() methods we must define the first param of this method as &mut self cause these methods are mutating the state of the contract and in order to call them inside this method the first param must be &mut self - returning a boolean to where it has called (the nft_transfer_call() method) based on the result of the nft_on_tranfer() cross contract call promise method - a private method to use it as a callback which will handle the result of the executed nft_on_transfer() promise or future object
        
        /*

            ------------------------------------------------------------------------------------

            this means that if nft_on_transfer() cross contract call promise method inside the 
            receiver_id's contract actor returned true, you should return false, 
            this is because if the token is being returned its original owner, 
            the receiver_id didn't successfully receive the token in the end,
            if nft_on_transfer() cross contract call promise method returned false, 
            you should return true since we don't need to return the token 
            and thus the receiver_id successfully owns the token.
            
            ------------------------------------------------------------------------------------

        */
        
        if let PromiseResult::Successful(value) = env::promise_result(0){ //-- getting the successful variant result using PromiseResult enum of the first .then() promise result using promise_result() method which has solved inside the nft_resolve_transfer() callback method; nft_resolve_transfer() method is the first callback which contains the first promise execution result - whether the receiver_id's contract actor wants to return the token back to the sender_id's contract actor based on the result of calling nft_on_transfer() cross contract call promise method which is either 1 or 0 cause it'll return true on successful transferred NFT to the receiver_id's contract actor
            if let Ok(return_token) = near_sdk::serde_json::from_slice::<bool>(&value){ //-- deserializing the vector of utf8 bytes took from the result of the first promise into bool data structure - deserializing the vector of utf8 bytes into bool data structure (a boolean that tells us whether we should return the token to it's owner or not) which has taken from the first solved (solved inside the the nft_resolve_transfer() callback method) promise which is the promise (future object) DataReceipt object contains the result of the transaction which is a promise (future object) ActionReceipt object scheduled inside the sender_id's contract actor in the nft_transfer_call() method cross contract call by calling the nft_on_transfer() cross contract call promise method of the receiver_id's contract actor 
                if !return_token{ //-- since nft_on_transfer() cross contract call promise method inside the receiver_id's contract actor returned 0 or false means that the token has been transferred successfully to the receiver_id's contract actor thus we shouldn't transfer the token back to its sender anymore
                    utils::refund_approve_account_ids(owner_id, &approved_account_ids); //-- refunding the owner for releasing the storage used up by the approved account ids (cause he/she doesn't own any allocated storage for approval_account_ids for the token cause the token has been transferred to another one) based on the transferred token object since the owner paid for it when he/she was minting the NFT inside the nft_mint() method; by doing this we are sure that the transfer went through and the token is transferred successfully cause the nft_on_transfer() cross contract call promise method returned false means that the token transferred successfully to the receiver_id's contract actor - passing the approved_account_ids in its borrowed from by taking a reference to its location using & to prevent from moving and losing its ownership so we can have it in later scopes
                    return true; //-- returning true to where this method has called which is inside the nft_transfer_call() method; since the nft_transfer_call() method returns true only if the token was transferred successfully from the sender_id's contract actor to the receiver_id's contract actor thus nft_on_transfer() cross contract call promise method must return false or 0 due to returning false on successfully transferred token to the receiver_id's contract actor code flow
                }
            }
        }
        
        
        let mut token = match self.tokens_by_id.get(&token_id){ //-- getting the token object related to the token_id (passed by reference to borrow it) if there is some token object from the self.tokens_by_id LookupMap
            Some(token) => {
                if token.owner_id != receiver_id{ //-- if the current token owner_id wasn't equaled to the current receiver_id means that this token doesn't belong to this receiver anymore since it might be transferred successfully to another receiver_id's contract actor thus we must return true anyway :)
                    utils::refund_approve_account_ids(owner_id, &approved_account_ids); //-- refunding the owner for releasing the storage used up by the approved account ids (cause he/she doesn't own any allocated storage for approval_account_ids for the token cause the token has been transferred to another one) based on the transferred token object since the owner paid for it when he/she was minting the NFT inside the nft_mint() method; by doing this we are sure that the transfer went through and the token might be transferred successfully to another receiver_id's contract actor - passing the approved_account_ids in its borrowed from by taking a reference to its location using & to prevent from moving and losing its ownership so we can have it in later scopes
                    let token_status_message = format!("The Owner of the Token {} is not @{:?} cause it might be transferred successfully to another receiver_id's contract actor", token_id, receiver_id); //-- format!() returns a String which takes 24 bytes storage, usize * 3 (pointer, len, capacity) bytes (usize is 64 bits or 24 bytes on 64 bits arch)
                    env::log_str(token_status_message.as_str()); //-- &str allocates low cost storage than the String which will get usize (usize is 64 bits or 24 bytes on 64 bits arch) * 3 (pointer, len, capacity) bytes; cause it's just the size of the str itself which is the total length of its utf8 bytes array on either stack, heap or binary which is equals to its length of utf8 bytes and due to its unknown size at compile time we must borrow it by taking a pointer to its location
                    return true; //-- returning true to where this method has called which is inside the nft_transfer_call() method; since the current token owner_id is not equals to the current receiver_id thus we must return true cause the token might be transferred successfully to another receiver_id's contract actor and doesn't belong to the current or this receiver_id's contract actor anymore
                }
                token //-- if we're here means we must return the token from this arm to transfer it back to its sender since the token was not successfully transferred to the receiver_id's contract actor which means the nft_on_transfer() method returned true
            },
            None => { //-- returning true to where this method has called which is inside the nft_transfer_call() method; since we didn't found the token means it was burned and doesn't exist thus no need to transfer the token back to the sender
                utils::refund_approve_account_ids(owner_id, &approved_account_ids); //-- refunding the owner for releasing the storage used up by the approved account ids (cause he/she doesn't own any allocated storage for approval_account_ids for the token cause the token has been transferred to another one) based on the transferred token object since the owner paid for it when he/she was minting the NFT inside the nft_mint() method; by doing this we are sure that the token is burned since there is no token found thus the allocated storage for that token must be released and refund the owner - passing the approved_account_ids in its borrowed from by taking a reference to its location using & to prevent from moving and losing its ownership so we can have it in later scopes
                return true
            },
        };
        

        env::log_str(&format!("Return the token {} from @{:?} to @{:?}", token_id, receiver_id, owner_id).as_str()); //-- format!() returns a String which takes 24 bytes storage, usize * 3 (pointer, len, capacity) bytes (usize is 64 bits or 24 bytes on 64 bits arch)
        self.internal_remove_token_from_owner(&receiver_id, &token_id); //-- passing the token_id and the receiver_id by reference to borrow them to prevent from moving cause we want to use them inside later scopes - removing the NFT from the self.tokens_per_owner LookupMap
        self.internal_add_token_to_owner(&owner_id, &token_id); //-- passing the token_id and the owner_id by reference to borrow them to prevent from moving cause we want to use them inside later scopes - adding the NFT to self.tokens_per_owner LookupMap
        token.owner_id = owner_id; //-- updating the owner_id field of the token object with the last original owner_id
        

        utils::refund_approve_account_ids(receiver_id.clone(), &token.approved_account_ids); //-- cloning the receiver_id since we want to use it in NftTransferLog struct - refunding the receiver for releasing the storage used up by the approved account ids based on the transferred token object since the receiver might be called nft_approve() method which has paid for approving an account; by doing this we are sure that the token didn't transferred successfully from sender (either the owner or an approved account_id) to the receiver_id's contract actor therefore we must immediately refund the receiver for all account_ids that he/she might has approved on his/her token - passing the approved_account_ids in its borrowed from by taking a reference to its location using & to prevent from moving and losing its ownership so we can have it in later scopes
        token.approved_account_ids = approved_account_ids; //-- resetting the approved_account_ids field of the transferred token object to what they were before the transfer since the transfer didn't go through successfully we must update the approved_account_ids field again with the old one passed in nft_resolve_transfer() callback method
        self.tokens_by_id.insert(&token_id, &token); //-- insert the updated token back into the self.tokens_by_id collection, insert() method will update the value on second call if there was any entry with that key exists cause hashmap based data structures use the hash of the key to validate the uniquness of their values and we must use enum based storage key if we want to add same key twice but with different values in two different collections to avoid data collision
        
        
        

        //////////////////////////////////////////////////////////////////////////////////////////////////
        ////////////////// CONSTRUCTING THE TRANSFER LOG AS PER THE EVENTS STANDARD //////////////////////
        ////////////////////////////////////////////////////////////////////////////////////////////////// 
        //// ➔ shared reference can't dereference between threads and can't move out of it cause by 
        ////     moving or dereferencing it it'll lose its ownership and lifetime while some methods and 
        ////     threads are using it; we can sovle this using as_ref() method wich converts a &wrapped 
        ////     type into &T or by cloning the type.
        //// ➔ we need to log that the NFT was reverted back to the original owner
        ////     the old_owner_id will be the receiver and the new_owner_id will be the
        ////     original owner of the token since we're reverting the transfer.
        //// ➔ if you only place the log in the internal_transfer function, the log will be emitted 
        ////     and the indexer will think that the NFT was transferred if the transfer is reverted 
        ////     during nft_resolve_transfer, however, that event should also be emittedanywhere that 
        ////     an NFT could be transferred, we should add logs.
        //// ➔ the old_owner_id will be the receiver_id that we've tried to transfer the NFT to it
        ////     and the receiver_id will be the previous owner of the NFT since we haven't returned true, 
        ////     that means that we should return the token to it's original owner.
        //////////////////////////////////////////////////////////////////////////////////////////////////
        let event_receiver_id = token.owner_id.clone(); //-- cloning the owner_id to prevent from moving since we can't dereference a shared reference that doesn't implement the Copy trait
        let old_owner_id = receiver_id.clone(); //-- cloning the receiver_id to prevent the token from moving since we can't dereference a shared reference that doesn't implement the Copy trait
        let nft_transfer_log = EventLog{ //-- emitting the transferring event
            standard: NFT_STANDARD_NAME.to_string(), //-- the current standard
            version: NFT_METADATA_SPEC.to_string(), //-- the version of the standard based on near announcement
            event: EventLogVariant::NftTransfer(vec![NftTransferLog{ //-- the data related with the transferring event stored in a vector 
                authorized_id, //-- using the passed in authorized_id for logging the transfer event and it can be the NFT owner or an approved account like the marketplace account_id - if there was any approved account_id to transfer the NFT on behalf of the owner
                old_owner_id, //-- the receiver id the old owner
                new_owner_id: event_receiver_id, //-- the old owner is the new receiver
                token_ids: vec![token_id.to_string()], //-- list of all minted token ids; since it might be an airdrop or giveaway batch
                price: None,
                done_at: env::block_timestamp(), //-- the timestamp that this method or transaction is done with executing 
                memo: None, //-- the memo message which is None
            }]),
        }; // NOTE - since we've implemented the Display trait for the EventLog struct thus we can convert the nft_transfer_log instance to string to log the nft transferring event info at runtime
        env::log_str(&nft_transfer_log.to_string()); //-- format!() returns a String which takes 24 bytes storage, usize * 3 (pointer, len, capacity) bytes (usize is 64 bits or 24 bytes on 64 bits arch)
        let event_vector = self.token_events.entry(token_id).or_insert(vec![]);
        event_vector.push(nft_transfer_log);
        //////////////////////////////////////////////////////////////////////////////////////////////////
        
        
        
        
        false //-- returning false to where this method has called which is inside the nft_transfer_call() method; since we haven't returned true, that means that we should return the token to it's original owner cause when we're here means nft_on_transfer() method returned true
    
    }

}














