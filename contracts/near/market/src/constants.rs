



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





// NOTE - all gas fees are in gas unit amount which will be attached to a specific call
// NOTE - every 100 kb cost 1 $NEAR or 10_000_000_000_000_000_000 (10^19 in yocto Ⓝ (1e24)) per byte thus every 1 kb costs 0.01 $NEARs on the chain since 10^19 * 1000 = 10^22 * 10^-24 = 0.01 $NEARs
// NOTE - since evey 1 kb costs 0.01 $NEARs thus we have to multiply the cost of 1 byte in yocto Ⓝ (1e24) amount which is 10^19 by 1000 to get 10^22 finally to get total amount in $NEAR we must multiply this amount by 10^-24 which would be 0.01 $NEARs




pub const NO_DEPOSIT: Balance = 0;
pub const GAS_FOR_RESOLVE_PURCHASE: Gas = Gas(115_000_000_000_000); //-- the required gas to resolve the result of cross contract call nft_transfer_payout() 
pub const GAS_FOR_NFT_TRANSFER: Gas = Gas(15_000_000_000_000); //-- the required gas to transfer for cross contract call nft_transfer_payout() method on the NFT owner which is the seller contract actor account
pub const STORAGE_PER_SALE: u128 = 1000 * STORAGE_PRICE_PER_BYTE; //-- the required storage cost per sell would be 10^19 yocto Ⓝ (1e24) per byte or 0.01 $NEARs per kb - this is the required minimum storage to have one sell on the market contract since we have to cover the storage cost of mutating the state of the contract on the chain by creating a sell object
pub static DELIMETER: &str = "."; //-- every sale will have a unique id which is in form `nft_contract_actor_account_id + DELIMETER + token_id` - nft_contract_actor_account_id is the account_id that the NFT contract is deployed on
pub type SalePriceInYoctoNear = U128; //-- the price of the sale in yocto Ⓝ (1e24) - Balance is of type u128 which is big enough to store yocto Ⓝ (1e24) 
pub type TokenId = String;
pub type FungibleTokenId = AccountId;
pub type ContractAndTokenId = String; //-- sale unique id
pub type Bids = Vec<Bid>;
pub type TimestampSec = u32;
pub type ContractAccountIdTokenId = String;
pub const FIVEMINUTES: u64 = 300000000000; // 5 mins to nano sec
pub const MAX_PRICE: Balance = 1_000_000_000 * 10u128.pow(24);
pub const BASE_GAS: Gas = Gas(5_000_000_000_000);
pub const GAS_FOR_ROYALTIES: Gas = Gas(BASE_GAS.0 * 10u64);