


// https://github.com/OMGWINNING/NFT-Marketplace-Tutorial/blob/master/contracts/NFTMarketplace.sol
// https://github.com/Markkop/nft-marketplace/tree/main/contracts
// https://github.com/fjun99/nftmarketplace/tree/main/contracts
// https://trufflesuite.com/guides/nft-marketplace/#build-the-marketplace-contract



// -----------------------
// market and nft contract
// Author: wildonion
// -----------------------


pragma solidity ^0.8.4;

import "@openzeppelin/contracts/token/ERC721/IERC721.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/security/ReentrancyGuard.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";



interface DortzioNFTFactory{
    function createNFTCollection(
        string memory _name,
        string memory _symbol,
        uint256 _royaltyFee
    ) external;

    function isDortzioNFT(address _nft) external view returns (bool);
}


interface DortzioNFT{
    function getRoyaltyFee() external view returns(uint256);
    function getRoyaltyRecipient() external view returns (address);
}



// the market contract

contract DortzioNFTMarketPlace is Ownable, ReentrancyGuard{

    DortzioNFTFactory private immutable dortzioNFTFactory;

    uint256 private paltformFee;
    address private feeRecipient;

    struct ListNFT{

    }

    struct OfferNFT{

    }

    struct AuctionNFT{
        
    }



}
