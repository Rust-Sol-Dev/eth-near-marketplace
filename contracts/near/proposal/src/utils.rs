



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









#[derive(BorshSerialize, BorshDeserialize)]
pub enum StorageKey {
    ProposalById,
    ProposalByIdCollectionName,
}



pub fn refund_deposit(storage_used: u64){ //-- refunding the initial deposit based on the amount of storage that was used up - all balances are of type u128 to cover the yocto Ⓝ (1e24) amounts
    let required_cost = env::storage_byte_cost() * Balance::from(storage_used); //-- getting the required cost of mutating the contract state on the chain based on specified balance which is of type u128 from the used or released storage - storage_byte_cost() is the balance needed to store one byte on chain    
    let attached_deposit = env::attached_deposit(); //-- getting the attached deposit - attached_deposit() method will get the balance that was attached to the call that will be immediately deposited before the contract execution starts; this is the minimum balance required to call the nft_mint() method 0.1 $NEAR is attached and the caller will get refunded any excess that is unused at the end 
    if required_cost > attached_deposit{ //-- 1 yocto is 10^-24
        let panic_message = format!("Need {} yocto Ⓝ (1e24) for creating proposal", required_cost); //-- format!() returns a String which takes 24 bytes storage, usize * 3 (pointer, len, capacity) bytes (usize is 64 bits or 24 bytes on 64 bits arch)
        env::panic_str(panic_message.as_str()); //-- &str allocates low cost storage than the String which will get usize (usize is 64 bits or 24 bytes on 64 bits arch) * 3 (pointer, len, capacity) bytes; cause it's just the size of the str itself which is the total length of its utf8 bytes array on either stack, heap or binary which is equals to its length of utf8 bytes and due to its unknown size at compile time we must borrow it by taking a pointer to its location
    }
    let refund = attached_deposit - required_cost; //-- refunding the owner account by subtracting the required_cost from his/her attached_deposit in yocto Ⓝ (1e24)
    if refund > 1{ //-- if the refund was greater than 1 yocto Ⓝ (1e24), means we have to get pay back the remaining deposit as a refund to the predecessor_account_id - refund is of type u128 or 16 bytes
        Promise::new(env::predecessor_account_id()).transfer(refund); //-- transfer the refund (using system account_id) to the predecessor_account_id which is the previous contract actor account and the last (current) caller of a method - we've scheduled a promise object here to create a transaction for transferring some $NEARs asyncly to the predecessor account which is the last caller as the receiver_id's contract actor and where the scheduled promise must be executed in
    }
}