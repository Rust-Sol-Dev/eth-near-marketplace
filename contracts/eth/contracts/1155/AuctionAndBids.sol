// contracts/GameMarketplace.sol
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.8;

import "@openzeppelin/contracts/utils/Counters.sol";
import "@openzeppelin/contracts/token/ERC1155/IERC1155.sol";
import "@openzeppelin/contracts/security/ReentrancyGuard.sol";
import "@openzeppelin/contracts/token/ERC1155/utils/ERC1155Receiver.sol";
import "./IGetFreeItems.sol";
import "hardhat/console.sol";

contract AuctionAndBids is ReentrancyGuard {
    // --------------- Var --------------
    using Counters for Counters.Counter;
    Counters.Counter private auctionId;
    mapping(uint256 => Auction) private IdToAuction;
    mapping(address => mapping(uint=>uint)) private BidersBalances; // balance of each bidder
    address public platFormAddress;
    IGetFreeItems getFreeItem;
    address private getFreeItemsAddress;

    struct Auction {
        uint256 id;
        bool start;
        bool end;
        uint256 endAt;
        address[] bidders;
        address payable highestBidder;
        uint256 highestBid;
        address payable seller;
        uint256 itemId;
    }

    constructor(address _getFreeItemAddress) {
        getFreeItem = IGetFreeItems(_getFreeItemAddress);
        platFormAddress = address(0);
        getFreeItemsAddress = _getFreeItemAddress;
        console.log("ADDRESS '%s' ",getFreeItemsAddress );
    }

    // ------------- Events ------------
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

    event WihdrawSuccess(address indexed seller, uint256 balance);

    event AuctionCanceled(
        uint256 indexed auctionId,
        address indexed seller,
        uint256 indexed itemId
    );

    // create an Auction
    // our contract can recieve ERC1155 tokens
    function onERC1155Received(
        address,
        address,
        uint256,
        uint256,
        bytes memory
    ) public virtual returns (bytes4) {
        return this.onERC1155Received.selector;
    }

    function createAuction(
        uint256 _itemId,
        uint256 _timesInHour,
        uint256 _firstBid
    ) external {
        require(_firstBid > 0, "price = 0");
        auctionId.increment();
        uint256 currentBidId = auctionId.current();
        IdToAuction[currentBidId].id = currentBidId;
        IdToAuction[currentBidId].start = true;
        IdToAuction[currentBidId].end = false;
        IdToAuction[currentBidId].endAt = block.timestamp + _timesInHour * 1 hours;
        IdToAuction[currentBidId].bidders.push(msg.sender);
        IdToAuction[currentBidId].highestBidder = payable(msg.sender);
        IdToAuction[currentBidId].highestBid = _firstBid;
        IdToAuction[currentBidId].seller = payable(msg.sender);
        IdToAuction[currentBidId].itemId = _itemId;
        getFreeItem.changeOwner(address(this), _itemId);
        getFreeItem.changeState(_itemId);
        IERC1155(getFreeItemsAddress).safeTransferFrom(
            msg.sender,
            address(this),
            _itemId,
            1,
            ""
        );
        emit AuctionCreated(
            currentBidId,
            _itemId,
            msg.sender,
            block.timestamp + _timesInHour * 1 hours,
            _firstBid
        );
    }

    // Enter to the auction (bid)
    function bid(uint256 _auctionId) external payable nonReentrant {
        uint256 highest_bid = IdToAuction[_auctionId].highestBid;
        bool isStarted = IdToAuction[_auctionId].start;
        bool isEnded = IdToAuction[_auctionId].end;
        uint256 endAt = IdToAuction[_auctionId].endAt;
        require(msg.value > highest_bid, "value < H.B");
        require(isStarted, "!started");
        require(isEnded == false, "ended");
        require(block.timestamp < endAt, "time out");
        BidersBalances[msg.sender][_auctionId] += msg.value;
        uint256 itemId = IdToAuction[_auctionId].itemId;
        IdToAuction[_auctionId].highestBid = msg.value;
        IdToAuction[_auctionId].highestBidder = payable(msg.sender);
        IdToAuction[_auctionId].bidders.push(msg.sender);
        emit Bid(_auctionId, msg.sender, itemId, msg.value);
    }

    // end the auction everyone can call this function require timeend

    function endAuction(uint256 _auctionId) external nonReentrant {
        uint256 endTime = IdToAuction[_auctionId].endAt;
        bool isStarted = IdToAuction[_auctionId].start;
        bool isEnded = IdToAuction[_auctionId].end;
        require(block.timestamp >= endTime, "not yet");
        require(isStarted == true, "not started");
        require(isEnded == false, "ended");
        address payable highestBidder = IdToAuction[_auctionId].highestBidder;
        BidersBalances[highestBidder][_auctionId] = 0;
        uint256 highest_bid = IdToAuction[_auctionId].highestBid;
        uint256 itemId = IdToAuction[_auctionId].itemId;
        IdToAuction[_auctionId].end = true;
        IdToAuction[_auctionId].start = false;
        getFreeItem.changeOwner(address(this), itemId);
        getFreeItem.changeState(itemId);
        IERC1155(getFreeItemsAddress).safeTransferFrom(
            address(this),
            highestBidder,
            itemId,
            1,
            ""
        );
        emit AuctionEnded(_auctionId, highestBidder, itemId, highest_bid);
    }

    // Cancel Auction


    function cancelAuction(uint256 _auctionId) external {
        bool isStarted = IdToAuction[_auctionId].start;
        bool isEnded = IdToAuction[_auctionId].end;
        address seller = IdToAuction[_auctionId].seller;
        address highest_bidder = IdToAuction[_auctionId].highestBidder;
        require(highest_bidder == seller ,"you can't cancel");// there's already auction
        require(msg.sender == seller, "you can't");
        require(isStarted == true, "ended");
        require(isEnded == false, "ended");
        uint256 itemId = IdToAuction[_auctionId].itemId;
        IdToAuction[_auctionId].end = true;
        IdToAuction[_auctionId].start = false;
        getFreeItem.changeOwner(address(this), itemId);
        getFreeItem.changeState(itemId);
        IERC1155(getFreeItemsAddress).safeTransferFrom(
            address(this),
            msg.sender,
            itemId,
            1,
            ""
        );

        emit AuctionCanceled(_auctionId, msg.sender, itemId);
    }

    // withdraw bids but the highest bidder can't

    function withdrawBids(uint256 _auctionId) external nonReentrant {
        address highestBidder = IdToAuction[_auctionId].highestBidder;
        require(highestBidder != msg.sender,"you're H.B");
        uint256 allowedwithraw = 0;
        if (msg.sender != highestBidder) {
            allowedwithraw = BidersBalances[msg.sender][_auctionId];
            BidersBalances[msg.sender][_auctionId] = 0;
        }

        (bool sent, ) = payable(msg.sender).call{value: allowedwithraw}("");
        require(sent, "failed");
        emit WihdrawSuccess(msg.sender, allowedwithraw);
    }

    // Get Auction Details

    function getAuctionDetails(uint256 _auctionId)
        external
        view
        returns (Auction memory)
    {
        return (IdToAuction[_auctionId]);
    }

    
    function isUserBidder(address _user) external view returns(bool){
        uint tatalCreated = auctionId.current();
        for(uint i =1;i<=tatalCreated;i++){
            Auction memory auc = IdToAuction[i];
            for(uint j=0;j<auc.bidders.length;j++){
                if(_user == auc.bidders[j]){
                    return true;
                }
            }
        }
        return false;
    }

 


// FIX BALANCE

    function getBalanceOf(address _bidder,uint _auctionId) external view returns(uint){
        return BidersBalances[_bidder][_auctionId];
    }

    function totalAuctionsCreated() external view returns(uint){
        return auctionId.current();
    }
}
