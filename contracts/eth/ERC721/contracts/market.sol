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
    }

    struct OfferRequest {
        bool hasOffer;
        uint256 tokenId;
        address requester;
        uint256 price;
    }

    struct Auction {
        bool isActive;
        uint256 tokenId;
        address highestBidder;
        uint256 highestBid;
        uint256 endTime;
    }

    struct UserOffer {
        bool exists;
        uint256 tokenId;
        address buyer;
        uint256 price;
    }

    struct OwnerCopies{
        address owner;
        uint256 copies;
    }

    mapping(uint256 => Offer) public tokenIdToOffer;
    mapping(uint256 => Auction) public tokenIdToAuction;
    mapping(address => uint256) public pendingWithdrawals;  
    mapping(uint256 => OfferRequest) public tokenIdToOfferRequest;
    mapping(uint256 => OwnerCopies) public tokenOwnerCopies;
    mapping(uint256 => UserOffer[]) public userOffers;

    event MarketFeePercentageChanged(uint256 newMarketFeePercentage);
    event TokenMinted(uint256 tokenId, address owner);
    event OfferCreated(uint256 tokenId, uint256 price, uint256 royalty);
    event OfferCanceled(uint256 tokenId);
    event AuctionCreated(uint256 tokenId, uint256 endTime);
    event AuctionEnded(uint256 tokenId, address highestBidder, uint256 highestBid);
    event BidPlaced(uint256 tokenId, address bidder, uint256 bid);
    event OfferRequested(uint256 indexed tokenId, address indexed requester, uint256 price);
    event OfferRequestAccepted(uint256 indexed tokenId, address indexed requester, uint256 price);
    event OfferRequestRejected(uint256 indexed tokenId, address indexed requester);
    event NFTListed(uint256 indexed tokenId, address indexed seller, uint256 price);
    event NFTListingCanceled(uint256 indexed tokenId, address indexed seller);
    event UserOfferAdded(uint256 indexed tokenId, address indexed buyer, uint256 price);
    event UserOfferRemoved(uint256 indexed tokenId, address indexed buyer);
    event UserOfferAccepted(uint256 indexed tokenId, address indexed buyer, uint256 price);
    event UserOfferRejected(uint256 indexed tokenId, address indexed buyer);

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

    function mintNFT(address recipient, string memory tokenURI, uint256 tokenId, uint256 copies) public returns (uint256){
        // _tokenIds.increment();
        // uint256 tokenId = _tokenIds.current();
        _mint(recipient, tokenId);
        _setTokenURI(tokenId, tokenURI);
        tokenOwnerCopies[tokenId] = OwnerCopies({
            owner: recipient,
            copies: copies
        });
        emit TokenMinted(tokenId, recipient);
        return tokenId;
    }
    function listNFT(uint256 tokenId, uint256 price) public {
        require(_isApprovedOrOwner(_msgSender(), tokenId), "Not approved or owner");
        require(price > 0, "Price must be greater than 0");

        tokenIdToOffer[tokenId] = Offer(true, tokenId, _msgSender(), price);
        emit NFTListed(tokenId, _msgSender(), price);
    }
    function cancelNFTListing(uint256 tokenId) public {
        require(tokenIdToOffer[tokenId].seller == _msgSender(), "Not the seller");

        tokenIdToOffer[tokenId].isForSale = false;
        emit NFTListingCanceled(tokenId, _msgSender());
    }
    function addOffer(uint256 tokenId, uint256 price) public {
        require(price > 0, "Price must be greater than 0");
    
        UserOffer memory offer = UserOffer(true, tokenId, _msgSender(), price);
        userOffers[tokenId].push(offer);
        
        pendingWithdrawals[_msgSender()] -= price;
        pendingWithdrawals[owner()] += price;
        
        emit UserOfferAdded(tokenId, _msgSender(), price);

        
    }
    function removeOffer(uint256 tokenId, uint256 index) public {
        require(index < userOffers[tokenId].length, "Invalid offer index");
        require(userOffers[tokenId][index].buyer == _msgSender(), "Not the buyer");

        userOffers[tokenId][index].exists = false;
        emit UserOfferRemoved(tokenId, _msgSender());
    }
    function acceptUserOffer(uint256 tokenId, uint256 index, address[] memory _Royaltyrecipients, uint256[] memory _Royaltyamounts) public {
        require(_isApprovedOrOwner(_msgSender(), tokenId), "Not approved or owner");
        require(index < userOffers[tokenId].length, "Invalid offer index");
        require(userOffers[tokenId][index].exists, "Offer does not exist");
        
        UserOffer memory offer = userOffers[tokenId][index];
        uint256 price = offer.price;
        address buyer = offer.buyer;

        require(buyer != address(0), "Invalid buyer address");

        userOffers[tokenId][index].exists = false;

        
        uint256 total_royalties;
        uint royalties = _Royaltyrecipients.length;
        for (uint r = 0; r <= royalties; r++){
            address royaltyRecipient = _Royaltyrecipients[r];
            uint256 royaltyFee = _Royaltyamounts[r];
            require(royaltyFee >= 0 && royaltyFee <= 100, "Royalty must be between 0 and 100");
            if (royaltyFee > 0) {
                uint256 royaltyAmount = (price * royaltyFee) / 100;
                total_royalties += royaltyAmount;
                pendingWithdrawals[royaltyRecipient] += royaltyAmount;
            }
        }
        uint256 marketFeeAmount = (price * marketFeePercentage) / 1000;
        uint256 sellerAmount = price - total_royalties - marketFeeAmount;
        pendingWithdrawals[_msgSender()] += sellerAmount;
        pendingWithdrawals[owner()] += marketFeeAmount;


        // Transfer the NFT from the seller (owner) to the buyer
        _transfer(_msgSender(), buyer, tokenId);
        emit UserOfferAccepted(tokenId, buyer, price);
    }
    function rejectUserOffer(uint256 tokenId, uint256 index) public {
        require(_isApprovedOrOwner(_msgSender(), tokenId), "Not approved or owner");
        require(index < userOffers[tokenId].length, "Invalid offer index");
        require(userOffers[tokenId][index].exists, "Offer does not exist");

        address buyer = userOffers[tokenId][index].buyer;
        uint256 price = userOffers[tokenId][index].price;
        userOffers[tokenId][index].exists = false;
        pendingWithdrawals[buyer] += price;
        pendingWithdrawals[owner()] -= price;

        emit UserOfferRejected(tokenId, buyer);
    }
    function depositForOffer(uint256 tokenId) public payable {
        Offer storage offer = tokenIdToOffer[tokenId];
        // require(offer.isForSale, "Token not for sale");
        require(msg.value >= offer.askingPrice, "Insufficient funds");
        pendingWithdrawals[_msgSender()] += msg.value;
    }
    function buyNFT(uint256 tokenId, address[] memory _Royaltyrecipients, uint256[] memory _Royaltyamounts, uint256 quantity) public payable {
        Offer memory offer = tokenIdToOffer[tokenId];
        require(offer.isForSale, "Token not for sale");
        require(msg.value >= offer.askingPrice * quantity, "Insufficient funds");

        uint256 total_royalties;
        uint royalties = _Royaltyrecipients.length;
        for (uint r = 0; r <= royalties; r++){
            address royaltyRecipient = _Royaltyrecipients[r];
            uint256 royaltyFee = _Royaltyamounts[r];
            require(royaltyFee >= 0 && royaltyFee <= 100, "Royalty must be between 0 and 100");
            if (royaltyFee > 0) {
                uint256 royaltyAmount = (msg.value * royaltyFee) / 100;
                total_royalties += royaltyAmount;
                pendingWithdrawals[royaltyRecipient] += royaltyAmount;
            }
        }
        uint256 marketFeeAmount = (msg.value * marketFeePercentage) / 1000;
        uint256 sellerAmount = msg.value - total_royalties - marketFeeAmount;
        pendingWithdrawals[_msgSender()] += sellerAmount;
        pendingWithdrawals[owner()] += marketFeeAmount;

        _transfer(offer.seller, _msgSender(), tokenId);
        tokenIdToOffer[tokenId].isForSale = false;
    }
    function createAuction(uint256 tokenId, uint256 duration) public {
        require(_isApprovedOrOwner(_msgSender(), tokenId), "Not approved or owner");
        uint256 endTime = block.timestamp + duration;
    
         tokenIdToAuction[tokenId] = Auction(true, tokenId, address(0), 0, endTime);
        emit AuctionCreated(tokenId, endTime);
    }
    function cancelAuction(uint256 tokenId) public {
        Auction storage auction = tokenIdToAuction[tokenId];
        require(auction.isActive, "Auction not active");
        require(_isApprovedOrOwner(_msgSender(), tokenId), "Not approved or owner");

        if (auction.highestBidder != address(0)) {
            pendingWithdrawals[auction.highestBidder] += auction.highestBid;
        }

        auction.isActive = false;
        emit AuctionEnded(tokenId, address(0), 0);
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

    function endAuction(uint256 tokenId, address[] memory _Royaltyrecipients, uint256[] memory _Royaltyamounts) public {
        Auction storage auction = tokenIdToAuction[tokenId];
        require(auction.isActive, "Auction not active");
        require(block.timestamp >= auction.endTime, "Auction has not ended yet");

        uint256 total_royalties;
        uint royalties = _Royaltyrecipients.length;
        for (uint r = 0; r <= royalties; r++){
            address royaltyRecipient = _Royaltyrecipients[r];
            uint256 royaltyFee = _Royaltyamounts[r];
            require(royaltyFee >= 0 && royaltyFee <= 100, "Royalty must be between 0 and 100");
            if (royaltyFee > 0) {
                uint256 royaltyAmount = (auction.highestBid * royaltyFee) / 100;
                total_royalties += royaltyAmount;
                pendingWithdrawals[royaltyRecipient] += royaltyAmount;
            }
        }
        uint256 marketFeeAmount = (auction.highestBid * marketFeePercentage) / 1000;
        uint256 sellerAmount = auction.highestBid - total_royalties - marketFeeAmount;
        pendingWithdrawals[_msgSender()] += sellerAmount;
        pendingWithdrawals[owner()] += marketFeeAmount;

        _transfer(ownerOf(tokenId), auction.highestBidder, tokenId);
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
    function buyNFTs(
        uint256[] memory tokenIds,
        uint256[] memory quantities,
        address[][] memory _Royaltyrecipients,
        uint256[][] memory _Royaltyamounts
    ) public payable {
        require(tokenIds.length == quantities.length, "Token IDs and quantities length mismatch");

        uint256 totalCost;
        for (uint256 i = 0; i < tokenIds.length; i++) {
            uint256 tokenId = tokenIds[i];
            uint256 quantity = quantities[i];

            Offer memory offer = tokenIdToOffer[tokenId];
            require(offer.isForSale, "Token not for sale");

            uint256 cost = offer.askingPrice * quantity;
            totalCost += cost;
        }

        require(msg.value >= totalCost, "Insufficient funds");

        for (uint256 i = 0; i < tokenIds.length; i++) {
            uint256 tokenId = tokenIds[i];
            uint256 quantity = quantities[i];

            Offer memory offer = tokenIdToOffer[tokenId];
            uint256 cost = offer.askingPrice * quantity;

            uint256 total_royalties;
            uint royalties = _Royaltyrecipients[i].length;
            for (uint r = 0; r < royalties; r++){
                address royaltyRecipient = _Royaltyrecipients[i][r];
                uint256 royaltyFee = _Royaltyamounts[i][r];
                require(royaltyFee >= 0 && royaltyFee <= 100, "Royalty must be between 0 and 100");
                if (royaltyFee > 0) {
                    uint256 royaltyAmount = (cost * royaltyFee) / 100;
                    total_royalties += royaltyAmount;
                    pendingWithdrawals[royaltyRecipient] += royaltyAmount;
                }
            }

            uint256 marketFeeAmount = (cost * marketFeePercentage) / 1000;
            uint256 sellerAmount = cost - total_royalties - marketFeeAmount;

            pendingWithdrawals[offer.seller] += sellerAmount;
            pendingWithdrawals[owner()] += marketFeeAmount;

            for (uint256 q = 0; q < quantity; q++) {
                _transfer(offer.seller, _msgSender(), tokenId);
            }

            tokenIdToOffer[tokenId].isForSale = false;
        }
    }
    // Override the tokenURI function
    function tokenURI(uint256 tokenId) public view override(ERC721, ERC721URIStorage) returns (string memory) {
        return ERC721URIStorage.tokenURI(tokenId);
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
