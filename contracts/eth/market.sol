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

