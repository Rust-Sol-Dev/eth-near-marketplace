
// contracts/GameMarketplace.sol
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.8;

interface IAuctionAndBids {
    struct Auction {
        uint256 id;
        bool start;
        bool end;
        uint256 endAt;
        address payable highestBidder;
        uint256 highestBid;
        address payable seller;
        uint256 itemId;
    }
    event AuctionCreated(
        uint256 indexed acutionId,
        uint256 indexed nftId,
        address indexed seller,
        uint256 endAt,
        uint256 price
    );
    event Bid(
        uint256 indexed auctionId,
        address indexed bidder,
        uint256 indexed nftId,
        uint256 bid
    );
    event AuctionEnded(
        uint256 indexed auctionId,
        address indexed bidder,
        uint256 indexed nftId,
        uint256 price
    );

    event WihdrawSuccess(
        address indexed seller,
        uint balance
    );

    event AuctionCanceled(
        uint indexed auctionId,
        address indexed seller,
        uint indexed itemId
    );
    
    function createAuction(uint256 _itemId,uint256 _endAt,uint256 _firstBid) external;
    function bid(uint256 _auctionId) external payable;
    function endAuction(uint256 _auctionId) external;
    function cancelAuction(uint _auctionId) external;
    function withdrawBids(uint256 _auctionId) external;
    function getAuctionDetails(uint _auctionId) external view returns(Auction memory);
    function getBalanceOf(address _bidder) external view returns(uint);
}