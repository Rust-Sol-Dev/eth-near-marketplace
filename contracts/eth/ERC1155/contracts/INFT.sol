
pragma solidity ^0.8.8;

interface INFT {

    struct NFT {
        uint256 id;
        address owner;
        bool onSell;
    }
    event FreeNFTMinted(
        uint256 indexed id,
        address indexed minter,
    );

   function getFreeNFT() external ;
   function getNFTDetails(uint _itemId) external view returns(NFT memory);
   function totalNFTsMinted () external view returns (uint);
   function getUserInventory(address _user) external view returns (NFT[] memory);
   function changeOwner(address _newOwner, uint _itemId) external;
   function changeState(uint _itemId) external;
   function changeBuyAndSellAddress(address _buyAndSellAddress) external;
   function getType(uint _choice) external view returns(TYPE);
}
