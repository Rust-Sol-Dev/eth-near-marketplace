

pragma solidity ^0.8.8;

import "@openzeppelin/contracts/token/ERC1155/ERC1155.sol";
import "@openzeppelin/contracts/utils/Counters.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/token/ERC1155/extensions/ERC1155URIStorage.sol";
import "@openzeppelin/contracts/token/ERC1155/utils/ERC1155Holder.sol";


contract GetFreeNFTs is ERC1155, ERC1155Holder, Ownable, ERC1155URIStorage {
    using EnumerableSet for EnumerableSet.UintSet;
    using Counters for Counters.Counter;
    Counters.Counter private itemId;
    address public buyAndSellAddress;
    address public auctionAddress;
    mapping(uint256 => NFT) private IdToNFT;
    mapping(uint256 => RoyaltyInfo[]) private _tokenRoyaltyInfo; // NFT id => royalty
    mapping (uint256 => string) private _tokenURIs;
    mapping(uint256 => uint256) public tokensHeldBalances; // id => balance (amount held not for sale)
    uint256 private royaltyFee;
    address private royaltyRecipient;


    struct RoyaltyInfo {
        address receiver;
        uint256 royaltyFee;
    }
    
    struct NFT {
        uint256 id;
        address owner;
        bool onSell;
        RoyaltyInfo[] royaltyinfo;
    }

    // ------------- Events -----------
    event FreeNFTMinted(
        uint256 indexed id,
        address indexed minter,
    );

    constructor(
    ) ERC1155("") {

    }

    function supportsInterface(bytes4 interfaceId)
        public
        view
        virtual
        override(ERC1155, ERC1155Receiver)
        returns (bool)
    {
        return super.supportsInterface(interfaceId);
    }

    // -------------  Get NFTS NOW It's FREE ------------

    // this modifier allow only ower smart contracts use some functions

    // only the contract buyandsell can accees to this modifer

    modifier onlyContract(){
        require(buyAndSellAddress == msg.sender || auctionAddress == msg.sender,"not allowed");
        _;
    }


    function gettokensHeldBalances(uint256[] tokenIds)
    public
        view
        returns (uint256[] memory){

        uint256[] amounts;
        for (a = 0; a <= tokenIds.length; a++){
            amounts[i] = tokensHeldBalances[tokenIds[a]];
        }

        return amounts;

    }
    

    function getNFTRoyaltyInfo(uint256 tokenId) 
    public
        view
        returns (RoyaltyInfo[] memory)
        {
            RoyaltyInfo memory ri = _tokenRoyaltyInfo[tokenId];
            return ri;
        }

    function MintNFT() external {
        itemId.increment();
        uint256 currentId = itemId.current();
        IdToNFT[currentId].id = currentId;
        IdToNFT[currentId].owner = msg.sender;
        IdToNFT[currentId].onSell = false;
        _tokenURIs[IdToNFT[currentId].id] = uri;
        _mint(msg.sender, currentId, 1, "");
        emit FreeNFTMinted(currentId, msg.sender);
    }

    function batchMint(
        address to,
        string[] memory tokenUris, 
        uint256[] memory ids,
        RoyaltyInfo[] memory royaltyInfo,
        uint256[] memory amounts
    ) external onlyOwner {
        require(tokenUris.length == amounts.length , "Ids and TokenUri length mismatch");
        _mintBatch(to, ids, amounts, "");
        for (uint256 i = 0; i < ids.length; i++) {
            require(amounts[i] > 0);
            if (tokensHeldBalances[ids[i]] == 0) {
                uint256 currentId = itemId.current();
                itemId.increment();
                IdToNFT[currentId].id = currentId;
                IdToNFT[currentId].owner = msg.sender;
                IdToNFT[currentId].onSell = false;
                IdToNFT[currentId].royaltyinfo = royaltyInfo;
                _tokenURIs[currentId] = tokenUris[i];
                tokensHeldBalances[currentId] += amounts[i];
                _tokenRoyaltyInfo[currentId] = royaltyInfo;
                emit FreeNFTMinted(currentId, msg.sender);
            }
        }
    }

    function getNFTDetails(uint256 _itemId)
        external
        view
        returns (NFT memory)
    {
        return IdToNFT[_itemId];
    }

    function tokenURI(uint256 tokenId)
        public
        view
        override(ERC1155, ERC1155URIStorage)
        returns (string memory)
    {
        // return super.tokenURI(tokenId);
        require(_exists(tokenId), "token with this id doesn't exist");
        return _tokenURIs[tokenId];
        
    }

    function setTokenURIs(string[] memory newtokenUris, uint256[] memory ids)
        public
        onlyContract
        
    {   
        for (uint256 i = 0; i < ids.length; i++) {
            require(_exists(ids[i]), "token with this id doesn't exist");
            _tokenURIs[ids[i]] = newtokenUris[ids[i]];
        }
        
    }

    function getUserInventory(address _user)
        external
        view
        returns (NFT[] memory)
    {
        uint256 totalNFTs = itemId.current();
        uint256 userNFTsCounter = 0;
        uint256 currentIndex = 0;
        // get length
        for (uint256 i = 1; i <= totalNFTs; i++) {
            if (IdToNFT[i].owner == _user) {
                userNFTsCounter += 1;
            }
        }
        NFT[] memory items = new NFT[](userNFTsCounter);

        for (uint256 i = 1; i <= totalNFTs; i++) {
            if (IdToNFT[i].owner == _user) {
                NFT storage currentNFT = IdToNFT[i];
                items[currentIndex] = currentNFT;
                currentIndex += 1;
            }
        }

        return items;
    }

    function totalNFTsMinted() external view returns (uint256) {
        return itemId.current();
    }


    function changeOwner(address _newOwner, uint _itemId) external onlyContract {
        IdToNFT[_itemId].owner = _newOwner;
    }


    function changeState(uint _itemId) external onlyContract{
        IdToNFT[_itemId].onSell = !IdToNFT[_itemId].onSell;
    }

    function changeBuyAndSellAddress(address _buyAndSellAddress, address _auctionAddress) external onlyOwner{
        buyAndSellAddress = _buyAndSellAddress;
        auctionAddress = _auctionAddress;
    }



}

