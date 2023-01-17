// contracts/GameMarketplace.sol
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.8;

interface IBuyAndSell {
    struct PRODUCT {
        uint256 id;
        uint256 itemId;
        uint256 price;
        bool isSold;
        address seller;
    }
    // ---------------  Events ------------
    event ItemIsOnSale(
        uint256 indexed productId,
        uint256 indexed nftId,
        uint256 price,
        address seller
    );
    event ItemsSold(
        uint256 indexed productId,
        uint256 indexed nftId,
        uint256 price,
        address buyer
    );

    function putProductToSell(uint256 _itemId, uint256 _price) external;

    function purchaseProduct(uint256 _productId) external payable;

    function cancelSell(uint256 _productId) external;

    function getUserSoldProducts(address _user)
        external
        view
        returns (uint256[] memory);

    function getProductDetail(uint256 _productId)
        external
        view
        returns (PRODUCT memory);
}
