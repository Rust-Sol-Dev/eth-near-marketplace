// contracts/GameMarketplace.sol
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.8;

interface IGetFreeItems {
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
    event FreeItemMinted(
        uint256 indexed id,
        address indexed minter,
        TYPE _type
    );

   function getFreeItem(TYPE itemType) external ;
   function getItemDetails(uint _itemId) external view returns(ITEM memory);
   function totalItemsMinted () external view returns (uint);
   function getUserInventory(address _user) external view returns (ITEM[] memory);
   function changeOwner(address _newOwner, uint _itemId) external;
   function changeState(uint _itemId) external;
   function changeBuyAndSellAddress(address _buyAndSellAddress) external;
   function getType(uint _choice) external view returns(TYPE);
}
