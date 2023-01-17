// contracts/GameMarketplace.sol
// SPDX-License-Identifier: MIT

pragma solidity ^0.8.8;

import "@openzeppelin/contracts/token/ERC1155/ERC1155.sol";
import "@openzeppelin/contracts/utils/Counters.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
import "hardhat/console.sol";

contract GetFreeItems is ERC1155, Ownable{
    using Counters for Counters.Counter;
    Counters.Counter private itemId;
    address public buyAndSellAddress;
    address public auctionAddress;
    mapping(uint256 => ITEM) private IdToItem;


    enum TYPE {
        NONE,
        RIFLE,
        PLAYER,
        BULLET_PROOF
    }

    struct ITEM {
        uint256 id;
        address owner;
        TYPE _type;
        bool onSell;
    }

    // ------------- Events -----------
    event FreeItemMinted(
        uint256 indexed id,
        address indexed minter,
        TYPE _type
    );

    constructor() ERC1155("") {
    }

    // -------------  Get ITEMS NOW It's FREE ------------

    // this modifier allow only ower smart contracts use some functions

    // only the contract buyandsell can accees to this modifer

    modifier onlyContract(){
        require(buyAndSellAddress == msg.sender || auctionAddress == msg.sender,"not allowed");
        _;
    }

    //  Get free items :)

    function getFreeItem(TYPE itemType) external {
        itemId.increment();
        uint256 currentId = itemId.current();
        IdToItem[currentId].id = currentId;
        IdToItem[currentId]._type = itemType;
        IdToItem[currentId].owner = msg.sender;
        IdToItem[currentId].onSell = false;
        _mint(msg.sender, currentId, 1, "");
        console.log("sender = '%s'",msg.sender);
        emit FreeItemMinted(currentId, msg.sender, itemType);
    }

    // details of an item

    function getItemDetails(uint256 _itemId)
        external
        view
        returns (ITEM memory)
    {
        return IdToItem[_itemId];
    }

    // get user iventory

    function getUserInventory(address _user)
        external
        view
        returns (ITEM[] memory)
    {
        uint256 totalItems = itemId.current();
        uint256 userItemsCounter = 0;
        uint256 currentIndex = 0;
        // get length
        for (uint256 i = 1; i <= totalItems; i++) {
            if (IdToItem[i].owner == _user) {
                userItemsCounter += 1;
            }
        }
        ITEM[] memory items = new ITEM[](userItemsCounter);

        for (uint256 i = 1; i <= totalItems; i++) {
            if (IdToItem[i].owner == _user) {
                ITEM storage currentItem = IdToItem[i];
                items[currentIndex] = currentItem;
                currentIndex += 1;
            }
        }

        return items;
    }

    // total items minted
    function totalItemsMinted() external view returns (uint256) {
        return itemId.current();
    }

    // change owner of an item

    function changeOwner(address _newOwner, uint _itemId) external onlyContract {
        IdToItem[_itemId].owner = _newOwner;
    }

    // change item state 

    function changeState(uint _itemId) external onlyContract{
        IdToItem[_itemId].onSell = !IdToItem[_itemId].onSell;
    }

    function changeBuyAndSellAddress(address _buyAndSellAddress, address _auctionAddress) external onlyOwner{
        buyAndSellAddress = _buyAndSellAddress;
        auctionAddress = _auctionAddress;
    }


    function getType(uint _choice) external pure returns(TYPE){
        if(_choice ==1){
            return TYPE.RIFLE;
        }
        else if (_choice ==2){
            return TYPE.PLAYER;
        } else if (_choice ==3){
            return TYPE.BULLET_PROOF;
        }
        else {
            return TYPE.NONE;
        }
    }
}

