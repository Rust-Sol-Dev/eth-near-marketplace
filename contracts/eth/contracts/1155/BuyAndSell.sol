// contracts/GameMarketplace.sol
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.8;
import "@openzeppelin/contracts/security/ReentrancyGuard.sol";
import "@openzeppelin/contracts/utils/Counters.sol";
import "@openzeppelin/contracts/token/ERC1155/IERC1155.sol";
import "@openzeppelin/contracts/token/ERC1155/utils/ERC1155Receiver.sol";

import "./IGetFreeItems.sol";
import "hardhat/console.sol";

contract BuyAndSell is ReentrancyGuard {
    // -----------  VAR --------------
    using Counters for Counters.Counter;

    Counters.Counter private productId;
    address public getFreeItemsAddress;
    IGetFreeItems getFreeItem;
    address public platFormAddress;
    mapping(uint256 => PRODUCT) private IdToProduct;
    mapping(address => uint256[]) private UserToSoldItems;

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

    event ItemCanceled(
        uint256 indexed productId,
        uint256 indexed itemId,
        address indexed seller
    );

    constructor(address _getFreeItemsAddress) {
        getFreeItem = IGetFreeItems(_getFreeItemsAddress);
        platFormAddress = address(0);
        getFreeItemsAddress = _getFreeItemsAddress;
        console.log("ADDRESS '%s' ",getFreeItemsAddress );

    }

    // -------------------- MARKETPLACE ----------------
    // put Product to sell = >
    // require price > 0

    function putProductToSell(uint256 _itemId, uint256 _price)
        external
        nonReentrant
    {
        require(_price > 0, "Price = 0");
        uint256 pricePlusFees = _price + (_price / 100) * 1;
        productId.increment();
        uint256 currentProductId = productId.current();
        getFreeItem.changeOwner(address(this), _itemId);
        getFreeItem.changeState(_itemId);

        IdToProduct[currentProductId].id = currentProductId;
        IdToProduct[currentProductId].price = pricePlusFees;
        IdToProduct[currentProductId].isSold = false;
        IdToProduct[currentProductId].seller = msg.sender;
        IdToProduct[currentProductId].itemId = _itemId;
        IERC1155(getFreeItemsAddress).safeTransferFrom(
            msg.sender,
            address(this),
            _itemId,
            1,
            ""
        );
        emit ItemIsOnSale(currentProductId, _itemId, pricePlusFees, msg.sender);
    }

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

    // ------------ Normal Buy Method -----------
    // require msg.value => price
    // product still availaibe

    function purchaseProduct(uint256 _productId) external payable nonReentrant {
        uint256 price = IdToProduct[_productId].price;
        bool isSold = IdToProduct[_productId].isSold;
        require(msg.value >= price, "amount < price");
        require(isSold == false, "Sold out");
        uint256 seller_share = msg.value - ((msg.value * 1) / 100);
        address seller = payable(IdToProduct[_productId].seller);
        //paiment send eth to seller and owner
        (bool sent, ) = seller.call{value: seller_share}("");
        require(sent, "failed");
        (bool sent2, ) = platFormAddress.call{
            value: (msg.value - seller_share)
        }("");
        require(sent2, "failed");
        // -------
        uint256 itemId = IdToProduct[_productId].itemId;
        getFreeItem.changeOwner(msg.sender, itemId);
        getFreeItem.changeState(itemId);
        UserToSoldItems[seller].push(itemId);
        IdToProduct[_productId].isSold = true;
        // transfer NFT
        IERC1155(getFreeItemsAddress).safeTransferFrom(
            address(this),
            msg.sender,
            itemId,
            1,
            ""
        );
        emit ItemsSold(_productId, itemId, seller_share, msg.sender);
    }

    // ----------- Cancel Sell --------------

    function cancelSell(uint256 _productId) external {
        PRODUCT memory product = IdToProduct[_productId];
        require(product.seller == msg.sender, "not S");
        require(product.isSold == false, "sold out");
        uint256 itemId = product.itemId;
        getFreeItem.changeOwner(msg.sender, itemId);
        getFreeItem.changeState(itemId);
        IdToProduct[_productId].isSold = true;
        IERC1155(getFreeItemsAddress).safeTransferFrom(
            address(this),
            msg.sender,
            itemId,
            1,
            ""
        );
        emit ItemCanceled(_productId, itemId, msg.sender);
    }

    // ----------- VIEWS---------

    // get user Sold Product

    function getUserSoldProducts(address _user)
        external
        view
        returns (uint256[] memory)
    {
        return UserToSoldItems[_user];
    }

    // get detail of a product

    function getProductDetail(uint256 _productId)
        external
        view
        returns (PRODUCT memory)
    {
        return IdToProduct[_productId];
    }

    function getUserOnSellItems(address _user) external view returns(PRODUCT[] memory){
        uint totalProducts = productId.current();
        uint userProductCounter  = 0 ;
        uint256 currentIndex = 0;
        // get length
        for (uint256 i = 1; i <= totalProducts; i++) {
            if (IdToProduct[i].seller == _user && IdToProduct[i].isSold == false) {
                userProductCounter += 1;
            }
        }
        PRODUCT[] memory products = new PRODUCT[](userProductCounter);

        for (uint256 i = 1; i <= totalProducts; i++) {
            if (IdToProduct[i].seller == _user && IdToProduct[i].isSold == false) {
                PRODUCT storage currentProduct = IdToProduct[i];
                products[currentIndex] = currentProduct;
                currentIndex += 1;
            }
        }
        return products;
    }

    // get sold product

    function getUserSoldProduct(address _user) external view returns(PRODUCT[] memory){
        uint totalProducts = productId.current();
        uint userProductCounter  = 0 ;
        uint256 currentIndex = 0;
        // get length
        for (uint256 i = 1; i <= totalProducts; i++) {
            if (IdToProduct[i].seller == _user && IdToProduct[i].isSold == true) {
                userProductCounter += 1;
            }
        }
        PRODUCT[] memory products = new PRODUCT[](userProductCounter);

        for (uint256 i = 1; i <= totalProducts; i++) {
            if (IdToProduct[i].seller == _user && IdToProduct[i].isSold == true) {
                PRODUCT storage currentProduct = IdToProduct[i];
                products[currentIndex] = currentProduct;
                currentIndex += 1;
            }
        }
        return products;
    }

    function getTotalProductCreated() external view returns (uint256) {
        return productId.current();
    }
}
