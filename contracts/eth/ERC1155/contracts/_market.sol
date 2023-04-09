pragma solidity ^0.8.0;

import "@openzeppelin/contracts/token/ERC721/ERC721.sol";
import "@openzeppelin/contracts/token/ERC721/extensions/ERC721Enumerable.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/utils/math/SafeMath.sol";
import "@openzeppelin/contracts/utils/Counters.sol";
import "@openzeppelin/contracts/token/ERC721/extensions/ERC721URIStorage.sol";

contract NFTMarketplace is ERC721, Ownable, ERC721URIStorage  {
    using Counters for Counters.Counter;
    Counters.Counter private _tokenIds;
    uint256 public marketFeePercentage;

    struct Offer {
        bool isForSale;
        uint256 tokenId;
        address seller;
        uint256 askingPrice;
        uint256 royalty;
    }

    struct Auction {
        bool isActive;
        uint256 tokenId;
        address highestBidder;
        uint256 highestBid;
        uint256 endTime;
    }

    mapping(uint256 => Offer) public tokenIdToOffer;
    mapping(uint256 => Auction) public tokenIdToAuction;
    mapping(address => uint256) public pendingWithdrawals;

    event MarketFeePercentageChanged(uint256 newMarketFeePercentage);
    event TokenMinted(uint256 tokenId, address owner);
    event OfferCreated(uint256 tokenId, uint256 price, uint256 royalty);
    event OfferCanceled(uint256 tokenId);
    event AuctionCreated(uint256 tokenId, uint256 endTime);
    event AuctionEnded(uint256 tokenId, address highestBidder, uint256 highestBid);
    event BidPlaced(uint256 tokenId, address bidder, uint256 bid);

    constructor() ERC721("NFT Marketplace", "NFTM") {
        // Set an initial market fee percentage, e.g., 2.5%
        marketFeePercentage = 25;
    }

    // Add a function to update the market fee percentage
    function setMarketFeePercentage(uint256 newMarketFeePercentage) public onlyOwner {
        require(newMarketFeePercentage >= 0 && newMarketFeePercentage <= 1000, "Invalid market fee percentage");
        marketFeePercentage = newMarketFeePercentage;
        emit MarketFeePercentageChanged(newMarketFeePercentage);
    }

    function mintNFT(address recipient, string memory tokenURI) public onlyOwner {
        _tokenIds.increment();
        uint256 tokenId = _tokenIds.current();
        _mint(recipient, tokenId);
        _setTokenURI(tokenId, tokenURI);
        emit TokenMinted(tokenId, recipient);
    }

    function createOffer(uint256 tokenId, uint256 price, uint256 royalty) public {
        require(_isApprovedOrOwner(_msgSender(), tokenId), "Not approved or owner");
        require(price > 0, "Price must be greater than 0");
        require(royalty >= 0 && royalty <= 100, "Royalty must be between 0 and 100");
        tokenIdToOffer[tokenId] = Offer(true, tokenId, _msgSender(), price, royalty);
        emit OfferCreated(tokenId, price, royalty);
    }

    function cancelOffer(uint256 tokenId) public {
        require(tokenIdToOffer[tokenId].seller == _msgSender(), "Not the seller");
        tokenIdToOffer[tokenId].isForSale = false;
        emit OfferCanceled(tokenId);
    }

    function acceptOffer(uint256 tokenId) public {
        Offer storage offer = tokenIdToOffer[tokenId];
        require(offer.isForSale, "Token not for sale");
        require(offer.seller == _msgSender(), "Not the seller");

        uint256 sellerAmount = offer.askingPrice;
        uint256 marketFeeAmount = (offer.askingPrice * marketFeePercentage) / 1000;
        uint256 royaltyAmount = (offer.askingPrice * offer.royalty) / 100;

        pendingWithdrawals[offer.seller] += sellerAmount - royaltyAmount - marketFeeAmount;
        pendingWithdrawals[owner()] += marketFeeAmount;

        _transfer(offer.seller, _msgSender(), tokenId);
        tokenIdToOffer[tokenId].isForSale = false;
    }

    function depositForOffer(uint256 tokenId) public payable {
        Offer storage offer = tokenIdToOffer[tokenId];
        require(offer.isForSale, "Token not for sale");
        require(msg.value >= offer.askingPrice, "Insufficient funds");

        uint256 marketFeeAmount = (msg.value * marketFeePercentage) / 1000;
        uint256 royaltyAmount = (msg.value * offer.royalty) / 100;
        uint256 sellerAmount = msg.value - royaltyAmount - marketFeeAmount;
        address seller = offer.seller;

        pendingWithdrawals[seller] += sellerAmount;
        pendingWithdrawals[owner()] += marketFeeAmount;

        _transfer(seller, _msgSender(), tokenId);
        tokenIdToOffer[tokenId].isForSale = false;
    }

    function buyNFT(uint256 tokenId) public payable {
        Offer memory offer = tokenIdToOffer[tokenId];
        require(offer.isForSale, "Token not for sale");
        require(msg.value >= offer.askingPrice, "Insufficient funds");

        uint256 marketFeeAmount = (msg.value * marketFeePercentage) / 1000;
        uint256 royaltyAmount = (msg.value * offer.royalty) / 100;
        uint256 sellerAmount = msg.value - royaltyAmount - marketFeeAmount;
        address seller = offer.seller;

        pendingWithdrawals[seller] += sellerAmount;
        pendingWithdrawals[owner()] += marketFeeAmount;

        _transfer(seller, _msgSender(), tokenId);
        tokenIdToOffer[tokenId].isForSale = false;
    }

    function createAuction(uint256 tokenId, uint256 duration) public {
        require(_isApprovedOrOwner(_msgSender(), tokenId), "Not approved or owner");
        uint256 endTime = block.timestamp + duration;
    
         tokenIdToAuction[tokenId] = Auction(true, tokenId, address(0), 0, endTime);
        emit AuctionCreated(tokenId, endTime);
    }

    function placeBid(uint256 tokenId) public payable {
        Auction storage auction = tokenIdToAuction[tokenId];
        require(auction.isActive, "Auction not active");
        require(block.timestamp < auction.endTime, "Auction has ended");
        require(msg.value > auction.highestBid, "Bid must be higher than the current highest bid");

        if (auction.highestBidder != address(0)) {
            pendingWithdrawals[auction.highestBidder] += auction.highestBid;
        }

        auction.highestBidder = _msgSender();
        auction.highestBid = msg.value;
        emit BidPlaced(tokenId, _msgSender(), msg.value);
    }

    function endAuction(uint256 tokenId) public {
        Auction storage auction = tokenIdToAuction[tokenId];
        require(auction.isActive, "Auction not active");
        require(block.timestamp >= auction.endTime, "Auction has not ended yet");

        address seller = ownerOf(tokenId);
        uint256 marketFeeAmount = (auction.highestBid * marketFeePercentage) / 1000;
        uint256 royaltyAmount = (auction.highestBid * tokenIdToOffer[tokenId].royalty) / 100;
        uint256 sellerAmount = auction.highestBid - royaltyAmount - marketFeeAmount;

        pendingWithdrawals[seller] += sellerAmount;
        pendingWithdrawals[owner()] += marketFeeAmount;

        _transfer(seller, auction.highestBidder, tokenId);
        auction.isActive = false;

        emit AuctionEnded(tokenId, auction.highestBidder, auction.highestBid);
    }

    function withdraw() public {
        uint256 amount = pendingWithdrawals[_msgSender()];
        require(amount > 0, "No funds to withdraw");

        pendingWithdrawals[_msgSender()] = 0;
        (bool success, ) = _msgSender().call{value: amount}("");
        require(success, "Withdrawal failed");
    }

    function tokensOfOwner(address _contract, address owner) external view returns (uint256[] memory) {
        uint256 tokenCount = balanceOf(owner);

        uint256[] memory tokens = new uint256[](tokenCount);
        for (uint256 i = 0; i < tokenCount; i++) {
            tokens[i] = IERC721Enumerable(_contract).tokenOfOwnerByIndex(owner, i);
        }

        return tokens;
    }

    // Override the tokenURI function
    function tokenURI(uint256 tokenId) public view override(ERC721, ERC721URIStorage) returns (string memory) {
        return ERC721URIStorage.tokenURI(tokenId);
    }

    function listNFTOnMarket(uint256 tokenId, uint256 price, uint256 royalty) public {
        createOffer(tokenId, price, royalty);
    }

    function updateMetadataURI(uint256 tokenId, string memory newTokenURI) public {
        require(_isApprovedOrOwner(_msgSender(), tokenId), "Not approved or owner");
        _setTokenURI(tokenId, newTokenURI);
    }

    // Override the _burn function
    function _burn(uint256 tokenId) internal override(ERC721, ERC721URIStorage) {
        ERC721URIStorage._burn(tokenId);
    }

}
