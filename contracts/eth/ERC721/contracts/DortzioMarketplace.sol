pragma solidity ^0.8.4;

import "@chainlink/contracts/src/v0.8/interfaces/AggregatorV3Interface.sol";
import "@openzeppelin/contracts/token/ERC721/IERC721.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/security/ReentrancyGuard.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
error NotOwner();
error PriceMustBeAboveZero();


contract PriceOFETHTOUSD{

    uint minimalValue = 50;
    
    function PriceFeed() public payable{
        msg.value > minimalValue;
    }

    function GetLastestPrice() public view returns (uint){
        AggregatorV3Interface Price = AggregatorV3Interface(0x8A753747A1Fa494EC906cE90E9f37563A8AF630e);
        (,int256 price,,,) = Price.latestRoundData();
        return uint (price * 1e18);
    }

    function GetValueInDOllar(uint _ethAmount) public view returns(uint){
        uint ValuePrice = GetLastestPrice();
        uint AmountinDollars =(ValuePrice * _ethAmount) /1e18;
        return AmountinDollars;
    }

}

interface IDortzioNFTFactory {
    function createNFTCollection(
        string memory _name,
        string memory _symbol
    ) external;
    function isdortzioNFT(address _nft) external view returns (bool);
}

interface IDortzioNFT {
    function getRoyaltiesCountOfNFT(uint256 _tokenId) external view returns (uint);
    function getRoyaltyReceiverOfNFT(uint256 _tokenId, uint index) external view returns (address);
    function getRoyaltyFeeOfNFT(uint256 _tokenId, uint index) external view returns (uint256);
    function totalNFTsMinted() external view returns (uint256);
}

/*  

    -------------------
    the market contract
    -------------------
    
    ** msg.sender IS THE CONTRACT OWNER WHICH IS THE COLLECTION CREATOR ** 
    
    

*/
contract dortzioNFTMarketplace is Ownable, ReentrancyGuard, PriceOFETHTOUSD {
    IDortzioNFTFactory private immutable dortzioNFTFactory;

    uint256 private platformFee;
    address private feeRecipient; // the address of the marketplace account that will receive the fee of the platform


    struct RoyaltyInfo {
        address receiver;
        uint256 royaltyFee;
    }


    struct ListNFT {
        address nft;
        uint256 tokenId;
        address seller;
        address payToken; // buyer
        uint256 price;
        bool sold;
    }

    struct OfferNFT {
        address nft;
        uint256 tokenId;
        address offerer;
        address payToken;
        uint256 offerPrice;
        bool accepted;
    }

    struct AuctionNFT {
        address nft;
        uint256 tokenId;
        address creator;
        address payToken;
        uint256 initialPrice;
        uint256 minBid;
        uint256 startTime;
        uint256 endTime;
        address lastBidder;
        uint256 heighestBid;
        address winner;
        bool success;
    }


    mapping (address => uint) public escrowAmount;

    mapping(address => bool) private payableToken;
    address[] private tokens;

    // nft => tokenId => list struct 
    mapping(address => mapping(uint256 => ListNFT)) private listNfts;

    // nft => tokenId => royalty struct 
    mapping(address => mapping(uint256 => RoyaltyInfo[])) private NftRoyatlies;

    // nft => tokenId => offerer address => offer struct
    mapping(address => mapping(uint256 => mapping(address => OfferNFT)))
        private offerNfts;

    // nft => tokenId => acution struct
    mapping(address => mapping(uint256 => AuctionNFT)) private auctionNfts;

    // auciton index => bidding counts => bidder address => bid price
    mapping(uint256 => mapping(uint256 => mapping(address => uint256)))
        private bidPrices;

    // events
    event ListedNFT(
        address indexed nft,
        uint256 indexed tokenId,
        address payToken,
        uint256 price,
        address indexed seller
    );

    event NFTRoyalty(
        address indexed nft,
        uint256 indexed tokenId,
        RoyaltyInfo[] royalties
    );

    event BoughtNFT(
        address indexed nft,
        uint256 indexed tokenId,
        address payToken,
        uint256 price,
        address seller,
        address indexed buyer
    );
    event OfferredNFT(
        address indexed nft,
        uint256 indexed tokenId,
        address payToken,
        uint256 offerPrice,
        address indexed offerer
    );
    event CanceledOfferredNFT(
        address indexed nft,
        uint256 indexed tokenId,
        address payToken,
        uint256 offerPrice,
        address indexed offerer
    );
    event AcceptedNFT(
        address indexed nft,
        uint256 indexed tokenId,
        address payToken,
        uint256 offerPrice,
        address offerer,
        address indexed nftOwner
    );
    event CreatedAuction(
        address indexed nft,
        uint256 indexed tokenId,
        address payToken,
        uint256 price,
        uint256 minBid,
        uint256 startTime,
        uint256 endTime,
        address indexed creator
    );
    event PlacedBid(
        address indexed nft,
        uint256 indexed tokenId,
        address payToken,
        uint256 bidPrice,
        address indexed bidder
    );

    event ResultedAuction(
        address indexed nft,
        uint256 indexed tokenId,
        address creator,
        address indexed winner,
        uint256 price,
        address caller
    );

    constructor(
        uint256 _platformFee,
        address _feeRecipient,
        IDortzioNFTFactory _dortzioNFTFactory
    ) {
        require(_platformFee <= 10000, "can't more than 10 percent");
        platformFee = _platformFee;
        feeRecipient = _feeRecipient;
        dortzioNFTFactory = _dortzioNFTFactory;
    }

    modifier isdortzioNFT(address _nft) {
        require(dortzioNFTFactory.isdortzioNFT(_nft), "not dortzio NFT");
        _;
    }

    modifier isListedNFT(address _nft, uint256 _tokenId) {
        ListNFT memory listedNFT = listNfts[_nft][_tokenId];
        require(
            listedNFT.seller != address(0) && !listedNFT.sold,
            "not listed"
        );
        _;
    }

    modifier HasNFTRoyalty(address _nft, uint256 _tokenId){
        RoyaltyInfo[] memory royalties = NftRoyatlies[_nft][_tokenId];
        for (uint256 i = 0; i < royalties.length; i++) {
            require(royalties[i].royaltyFee <= 10000, "can't more than 10 percent");
            require(royalties[i].receiver != address(0));
        }
        _;
    }

    modifier isNotListedNFT(address _nft, uint256 _tokenId) {
        ListNFT memory listedNFT = listNfts[_nft][_tokenId];
        require(
            listedNFT.seller == address(0) || listedNFT.sold,
            "already listed"
        );
        _;
    }

    modifier isOwner(
        address _nft,
        uint256 _tokenId,
        address spender
    ) {
        IERC721 nft = IERC721(_nft);
        address owner = nft.ownerOf(_tokenId);
        if (spender != owner) {
            revert NotOwner();
        }
        _;
    }

    modifier isAuction(address _nft, uint256 _tokenId) {
        AuctionNFT memory auction = auctionNfts[_nft][_tokenId];
        require(
            auction.nft != address(0) && !auction.success,
            "auction already created"
        );
        _;
    }

    modifier isNotAuction(address _nft, uint256 _tokenId) {
        AuctionNFT memory auction = auctionNfts[_nft][_tokenId];
        require(
            auction.nft == address(0) || auction.success,
            "auction already created"
        );
        _;
    }

    modifier isOfferredNFT(
        address _nft,
        uint256 _tokenId,
        address _offerer
    ) {
        OfferNFT memory offer = offerNfts[_nft][_tokenId][_offerer];
        require(
            offer.offerPrice > 0 && offer.offerer != address(0),
            "not offerred nft"
        );
        _;
    }

    modifier isPayableToken(address _payToken) {
        require(
            _payToken != address(0) && payableToken[_payToken],
            "invalid pay token"
        );
        _;
    }

    // List NFT on Marketplace
    function listNft(
        address _nft,
        uint256 _tokenId,
        address _payToken,
        uint256 _price
    ) external isdortzioNFT(_nft) isPayableToken(_payToken) {
        IERC721 nft = IERC721(_nft);
        require(nft.ownerOf(_tokenId) == msg.sender, "not nft owner");
        nft.transferFrom(msg.sender, address(this), _tokenId);

        listNfts[_nft][_tokenId] = ListNFT({
            nft: _nft,
            tokenId: _tokenId,
            seller: msg.sender,
            payToken: _payToken,
            price: _price,
            sold: false
        });

        emit ListedNFT(_nft, _tokenId, _payToken, _price, msg.sender);
    }

    // Cancel listed NFT
    function cancelListedNFT(address _nft, uint256 _tokenId)
        external
        isListedNFT(_nft, _tokenId)
    {
        ListNFT memory listedNFT = listNfts[_nft][_tokenId];
        require(listedNFT.seller == msg.sender, "not listed owner");
        IERC721(_nft).transferFrom(address(this), msg.sender, _tokenId);
        delete listNfts[_nft][_tokenId];
    }

    function addRoyalty(address _nft, uint256 _tokenId, RoyaltyInfo[] memory royalties ) 
        external
        isListedNFT(_nft, _tokenId)
        nonReentrant
        isOwner(_nft, _tokenId, msg.sender) {
            RoyaltyInfo[] memory ri = new RoyaltyInfo[](royalties.length);
            for (uint i = 0; i < royalties.length; i++){
                require(royalties[i].royaltyFee <= 10000, "can't more than 10 percent");
                require(royalties[i].receiver != address(0));
                ri[i].royaltyFee = royalties[i].royaltyFee;
                ri[i].receiver = royalties[i].receiver;
            }
                NftRoyatlies[_nft][_tokenId] = ri;

            emit NFTRoyalty(_nft, _tokenId, ri);
        }

    function updateListing(
        address _nft,
        uint256 _tokenId,
        uint256 newPrice
    )
        external
        isListedNFT(_nft, _tokenId)
        nonReentrant
        isOwner(_nft, _tokenId, msg.sender)
    {
        if (newPrice == 0) {
            revert PriceMustBeAboveZero();
        }

        listNfts[_nft][_tokenId].price = newPrice;
        address payToken = listNfts[_nft][_tokenId].payToken;
        emit ListedNFT(_nft, _tokenId, payToken, newPrice, msg.sender);
    }

    //-----------------------------------------------
    // for buy batch just create a batch tx in front
    //-----------------------------------------------
    // Buy listed NFT
    function buyNFT(
        address _nft,
        uint256 _tokenId,
        address _payToken,
        uint256 _price
    ) external isListedNFT(_nft, _tokenId) HasNFTRoyalty(_nft, _tokenId){
        ListNFT storage listedNft = listNfts[_nft][_tokenId];
        require(
            _payToken != address(0) && _payToken == listedNft.payToken,
            "invalid pay token"
        );
        require(!listedNft.sold, "nft already sold");
        require(_price >= listedNft.price, "invalid price");

        listedNft.sold = true;

        uint256 totalPrice = _price;
        IDortzioNFT dortzioNft = IDortzioNFT(listedNft.nft);
        RoyaltyInfo[] memory royalties = NftRoyatlies[_nft][_tokenId];
        for (uint r = 0; r <= royalties.length; r++){
            address royaltyRecipient = royalties[r].receiver;
            uint256 royaltyFee = royalties[r].royaltyFee;
            if (royaltyFee > 0) {
                uint256 royaltyTotal = calculateRoyalty(royaltyFee, _price);
                // Transfer royalty fee to receivers
                IERC20(listedNft.payToken).transferFrom(
                    msg.sender,
                    royaltyRecipient,
                    royaltyTotal
                );
                totalPrice -= royaltyTotal;
            }
        }

        // Calculate & Transfer platfrom fee
        uint256 platformFeeTotal = calculatePlatformFee(_price);
        IERC20(listedNft.payToken).transferFrom(
            msg.sender,
            feeRecipient,
            platformFeeTotal
        );

        // Transfer to nft owner
        IERC20(listedNft.payToken).transferFrom(
            msg.sender,
            listedNft.seller,
            totalPrice - platformFeeTotal
        );

        // Transfer NFT to buyer
        IERC721(listedNft.nft).safeTransferFrom(
            address(this),
            msg.sender,
            listedNft.tokenId
        );

        emit BoughtNFT(
            listedNft.nft,
            listedNft.tokenId,
            listedNft.payToken,
            _price,
            listedNft.seller,
            msg.sender
        );
    }

    // Offer listed NFT
    function offerNFT(
        address _nft,
        uint256 _tokenId,
        address _payToken,
        uint256 _offerPrice
    ) external isListedNFT(_nft, _tokenId) {
        require(_offerPrice > 0, "price can not 0");

        ListNFT memory nft = listNfts[_nft][_tokenId];
        IERC20(nft.payToken).transferFrom(
            msg.sender,
            address(this),
            _offerPrice
        );

        offerNfts[_nft][_tokenId][msg.sender] = OfferNFT({
            nft: nft.nft,
            tokenId: nft.tokenId,
            offerer: msg.sender,
            payToken: _payToken,
            offerPrice: _offerPrice,
            accepted: false
        });


        escrowAmount[msg.sender] += _offerPrice;


        emit OfferredNFT(
            nft.nft,
            nft.tokenId,
            nft.payToken,
            _offerPrice,
            msg.sender
        );
    }

    // Offerer cancel offerring
    function cancelOfferNFT(address _nft, uint256 _tokenId)
        external
        isOfferredNFT(_nft, _tokenId, msg.sender)
    {
        OfferNFT memory offer = offerNfts[_nft][_tokenId][msg.sender];
        require(offer.offerer == msg.sender, "not offerer");
        require(!offer.accepted, "offer already accepted");
        require(offer.offerPrice <= escrowAmount[msg.sender], "cancelOffer: lower amount in escrow  ");
        delete offerNfts[_nft][_tokenId][msg.sender];
        escrowAmount[msg.sender] -= offer.offerPrice;
        IERC20(offer.payToken).transfer(offer.offerer, offer.offerPrice);
        emit CanceledOfferredNFT(
            offer.nft,
            offer.tokenId,
            offer.payToken,
            offer.offerPrice,
            msg.sender
        );
    }

    function getNftRoyalty(address _nft, uint256 _tokenId) private returns (RoyaltyInfo[] memory){
        RoyaltyInfo[] memory royalties = NftRoyatlies[_nft][_tokenId];
        return royalties;
    }

    // listed NFT owner accept offerring
    function acceptOfferNFT(
        address _nft,
        uint256 _tokenId,
        address _offerer
    )
        external
        isOfferredNFT(_nft, _tokenId, _offerer)
        isListedNFT(_nft, _tokenId)
        HasNFTRoyalty(_nft, _tokenId)
    {
        require(
            listNfts[_nft][_tokenId].seller == msg.sender,
            "not listed owner"
        );
        OfferNFT storage offer = offerNfts[_nft][_tokenId][_offerer];
        ListNFT storage list = listNfts[offer.nft][offer.tokenId];
        RoyaltyInfo[] memory royalties = getNftRoyalty(_nft, _tokenId);
        require(!list.sold, "already sold");
        require(!offer.accepted, "offer already accepted");

        list.sold = true;
        offer.accepted = true;

        uint256 offerPrice = offer.offerPrice;
        uint256 totalPrice = offerPrice;

        require(offerPrice <= escrowAmount[offer.offerer], "acceptOffer: lower amount in escrow");

        IDortzioNFT dortzioNft = IDortzioNFT(offer.nft);
        IERC20 payToken = IERC20(offer.payToken);
        for (uint r = 0; r <= royalties.length; r++){
            address royaltyRecipient = royalties[r].receiver;
            uint256 royaltyFee = royalties[r].royaltyFee;
            if (royaltyFee > 0) {
                uint256 royaltyTotal = calculateRoyalty(royaltyFee, offerPrice);
                // Transfer royalty fee to receivers
                payToken.transfer(royaltyRecipient, royaltyTotal);
                totalPrice -= royaltyTotal;
            }
        }


        // Calculate & Transfer platfrom fee
        uint256 platformFeeTotal = calculatePlatformFee(offerPrice);
        payToken.transfer(feeRecipient, platformFeeTotal);

        // Transfer to seller
        payToken.transfer(list.seller, totalPrice - platformFeeTotal);


        escrowAmount[offer.offerer] -= offerPrice;

        // Transfer NFT to offerer
        IERC721(list.nft).safeTransferFrom(
            address(this),
            offer.offerer,
            list.tokenId
        );

        emit AcceptedNFT(
            offer.nft,
            offer.tokenId,
            offer.payToken,
            offer.offerPrice,
            offer.offerer,
            list.seller
        );
    }

    // Create autcion
    function createAuction(
        address _nft,
        uint256 _tokenId,
        address _payToken,
        uint256 _price,
        uint256 _minBid,
        uint256 _startTime,
        uint256 _endTime
    ) external isPayableToken(_payToken) isNotAuction(_nft, _tokenId) {
        IERC721 nft = IERC721(_nft);
        require(nft.ownerOf(_tokenId) == msg.sender, "not nft owner");
        require(_endTime > _startTime, "invalid end time");

        nft.transferFrom(msg.sender, address(this), _tokenId);

        auctionNfts[_nft][_tokenId] = AuctionNFT({
            nft: _nft,
            tokenId: _tokenId,
            creator: msg.sender,
            payToken: _payToken,
            initialPrice: _price,
            minBid: _minBid,
            startTime: _startTime,
            endTime: _endTime,
            lastBidder: address(0),
            heighestBid: _price,
            winner: address(0),
            success: false
        });

        emit CreatedAuction(
            _nft,
            _tokenId,
            _payToken,
            _price,
            _minBid,
            _startTime,
            _endTime,
            msg.sender
        );
    }

    // Cancel auction
    function cancelAuction(address _nft, uint256 _tokenId)
        external
        isAuction(_nft, _tokenId)
    {
        AuctionNFT memory auction = auctionNfts[_nft][_tokenId];
        require(auction.creator == msg.sender, "not auction creator");
        require(block.timestamp < auction.startTime, "auction already started");
        require(auction.lastBidder == address(0), "already have bidder");

        IERC721 nft = IERC721(_nft);
        nft.transferFrom(address(this), msg.sender, _tokenId);
        delete auctionNfts[_nft][_tokenId];
    }

    // Bid place auction
    function bidPlace(
        address _nft,
        uint256 _tokenId,
        uint256 _bidPrice
    ) external isAuction(_nft, _tokenId) {
        require(
            block.timestamp >= auctionNfts[_nft][_tokenId].startTime,
            "auction not start"
        );
        require(
            block.timestamp <= auctionNfts[_nft][_tokenId].endTime,
            "auction ended"
        );
        require(
            _bidPrice >=
                auctionNfts[_nft][_tokenId].heighestBid +
                    auctionNfts[_nft][_tokenId].minBid,
            "less than min bid price"
        );

        AuctionNFT storage auction = auctionNfts[_nft][_tokenId];
        IERC20 payToken = IERC20(auction.payToken);
        payToken.transferFrom(msg.sender, address(this), _bidPrice);

        if (auction.lastBidder != address(0)) {
            address lastBidder = auction.lastBidder;
            uint256 lastBidPrice = auction.heighestBid;

            // Transfer back to last bidder
            payToken.transfer(lastBidder, lastBidPrice);
        }

        // Set new heighest bid price
        auction.lastBidder = msg.sender;
        auction.heighestBid = _bidPrice;

        emit PlacedBid(_nft, _tokenId, auction.payToken, _bidPrice, msg.sender);
    }

    // Result auction, can call by auction creator, heighest bidder, or marketplace owner only!
    function resultAuction(address _nft, uint256 _tokenId) external {
        require(!auctionNfts[_nft][_tokenId].success, "already resulted");
        require(
            msg.sender == owner() ||
                msg.sender == auctionNfts[_nft][_tokenId].creator ||
                msg.sender == auctionNfts[_nft][_tokenId].lastBidder,
            "not creator, winner, or owner"
        );
        require(
            block.timestamp > auctionNfts[_nft][_tokenId].endTime,
            "auction not ended"
        );

        AuctionNFT storage auction = auctionNfts[_nft][_tokenId];
        IERC20 payToken = IERC20(auction.payToken);
        IERC721 nft = IERC721(auction.nft);

        auction.success = true;
        auction.winner = auction.creator;

        IDortzioNFT dortzioNft = IDortzioNFT(_nft);
        uint256 heighestBid = auction.heighestBid;
        uint256 totalPrice = heighestBid;
        RoyaltyInfo[] memory royalties = NftRoyatlies[_nft][_tokenId];
        for (uint r = 0; r <= royalties.length; r++){
            address royaltyRecipient = royalties[r].receiver;
            uint256 royaltyFee = royalties[r].royaltyFee;
            if (royaltyFee > 0) {
                uint256 royaltyTotal = calculateRoyalty(royaltyFee, heighestBid);
                // Transfer royalty fee to receivers
                payToken.transfer(royaltyRecipient, royaltyTotal);
                totalPrice -= royaltyTotal;
            }
        }

        // Calculate & Transfer platfrom fee
        uint256 platformFeeTotal = calculatePlatformFee(heighestBid);
        payToken.transfer(feeRecipient, platformFeeTotal);

        // Transfer to auction creator
        payToken.transfer(auction.creator, totalPrice - platformFeeTotal);

        // Transfer NFT to the winner
        nft.transferFrom(address(this), auction.lastBidder, auction.tokenId);

        emit ResultedAuction(
            _nft,
            _tokenId,
            auction.creator,
            auction.lastBidder,
            auction.heighestBid,
            msg.sender
        );
    }

    function calculatePlatformFee(uint256 _price)
        public
        view
        returns (uint256)
    {
        return (_price * platformFee) / 10000;
    }

    function calculateRoyalty(uint256 _royalty, uint256 _price)
        public
        pure
        returns (uint256)
    {
        return (_price * _royalty) / 10000;
    }

    function getListedNFT(address _nft, uint256 _tokenId)
        public
        view
        returns (ListNFT memory)
    {
        return listNfts[_nft][_tokenId];
    }

    function getListedNFTsOf(address _nft) public view returns (ListNFT[] memory){ // get all listed nfts inside the passed in nft contract
        IDortzioNFT nft = IDortzioNFT(_nft);
        ListNFT[] memory listed_nfts;
        uint256 pagination = 0; 
        uint256 total_nft_ids = nft.totalNFTsMinted();
        for (uint256 tokenId = total_nft_ids; tokenId >= 0; tokenId--){
            ListNFT storage list = listNfts[_nft][tokenId];
            listed_nfts[tokenId] = list;
        }
        return listed_nfts;
    }

    function getListedNFTsOfOwner(address _nft) public view returns (ListNFT[] memory){ // get all listed nfts inside the passed in nft contract
        IDortzioNFT nftObj = IDortzioNFT(_nft);
        IERC721 nft = IERC721(_nft);
        ListNFT[] memory listed_nfts;
        uint256 pagination = 0; 
        uint256 total_nft_ids = nftObj.totalNFTsMinted();
        for (uint256 tokenId = total_nft_ids; tokenId >= 0; tokenId--){
            require(nft.ownerOf(tokenId) == msg.sender, "not nft owner");
            ListNFT storage list = listNfts[_nft][tokenId];
            listed_nfts[tokenId] = list;
        }
        return listed_nfts;
    }

    function getPayableTokens() external view returns (address[] memory) {
        return tokens;  
    }

    function checkIsPayableToken(address _payableToken)
        external
        view
        returns (bool)
    {
        return payableToken[_payableToken];
    }

    function addPayableToken(address _token) external onlyOwner {
        require(_token != address(0), "invalid token");
        require(!payableToken[_token], "already payable token");
        payableToken[_token] = true;
        tokens.push(_token);
    }

    function updatePlatformFee(uint256 _platformFee) external onlyOwner {
        require(_platformFee <= 10000, "can't more than 10 percent");
        platformFee = _platformFee;
    }

    function changeFeeRecipient(address _feeRecipient) external onlyOwner {
        require(_feeRecipient != address(0), "can't be 0 address");
        feeRecipient = _feeRecipient;
    }

    function depositEscrow() external payable returns (bool) {
        escrowAmount[msg.sender] += msg.value;
        return true;

    }

    function getEscrowAmount() external view returns (uint){
        return escrowAmount[msg.sender];
    } 

    function withdrawEscrow(
        uint256 _amount
    ) external returns (bool) {
        require(_amount < escrowAmount[msg.sender], "withdrawEscrow: lower amount");
        payable(msg.sender).transfer(_amount);
        escrowAmount[msg.sender] -= _amount;
        return true;

    }
}
