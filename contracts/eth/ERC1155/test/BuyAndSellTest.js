const { expect, assert } = require("chai");
const { ethers } = require("hardhat");
const { deployGetFreeMint, deployBuyAndSell } = require("../scripts/deploy")
const hre = require("hardhat");

describe("Put Product to sell", function () {
    it("product with id 1 should exist ", async function () {

        [owner, account1, account2, account3] = await ethers.getSigners()
        const contract1 = await deployGetFreeMint()
        const getFreeItem = await contract1.connect(account1).getFreeItem(2)
        const contract2 = await deployBuyAndSell()
        const approve = await contract1.connect(account1).setApprovalForAll(contract2.address,true)
        const changeBuyAndSellAddress = await contract1.connect(owner).changeBuyAndSellAddress(contract2.address,contract2.address)
        const putProductToSell = await contract2.connect(account1).putProductToSell(1,9000000000)
        const product = await contract2.connect(account1).getProductDetail(1)
        expect(product.id).to.equal(1)

    })
    it("caller should be the seller of product 1", async function () {

        [owner, account1, account2, account3] = await ethers.getSigners()
        const contract1 = await deployGetFreeMint()
        const getFreeItem = await contract1.connect(account1).getFreeItem(2)
        const contract2 = await deployBuyAndSell()
        const approve = await contract1.connect(account1).setApprovalForAll(contract2.address,true)
        const changeBuyAndSellAddress = await contract1.connect(owner).changeBuyAndSellAddress(contract2.address,contract2.address)
        const putProductToSell = await contract2.connect(account1).putProductToSell(1,9000000000)
        const product = await contract2.connect(account1).getProductDetail(1)
        expect(product.seller).to.equal(account1.address)

    })
    it("Should revert because price is 0", async function () {

        [owner, account1, account2, account3] = await ethers.getSigners()
        const contract1 = await deployGetFreeMint()
        const getFreeItem = await contract1.connect(account1).getFreeItem(2)
        const contract2 = await deployBuyAndSell()
        const approve = await contract1.connect(account1).setApprovalForAll(contract2.address,true)
        const changeBuyAndSellAddress = await contract1.connect(owner).changeBuyAndSellAddress(contract2.address,contract2.address)
        await expect(contract2.connect(account1).putProductToSell(1,0)).to.be.revertedWith("Price = 0")

    })

})

describe("purchase product ", function () {
    it("the buyer should be the new owner of the item sold", async function () {
        [owner, account1, account2, account3] = await ethers.getSigners()
        const contract1 = await deployGetFreeMint()
        const getFreeItem = await contract1.connect(account1).getFreeItem(2)
        const contract2 = await deployBuyAndSell()
        const approve = await contract1.connect(account1).setApprovalForAll(contract2.address,true)
        const changeBuyAndSellAddress = await contract1.connect(owner).changeBuyAndSellAddress(contract2.address,contract2.address)
        const putProductToSell = await contract2.connect(account1).putProductToSell(1,9000000000)
        const product = await contract2.connect(account1).getProductDetail(1)
        const purchase = await contract2.connect(account2).purchaseProduct(1,{value: product.price})
        const productAfterPurchase = await contract2.connect(account2).getProductDetail(1)
        const item = await contract1.connect(account1).getItemDetails(productAfterPurchase.id)
        expect(item.owner).to.equal(account2.address)

    })
})

describe("Cancel Sell ", function () {
    it("the owner of the product should get his item back", async function () {
        [owner, account1, account2, account3] = await ethers.getSigners()
        const contract1 = await deployGetFreeMint()
        const getFreeItem = await contract1.connect(account1).getFreeItem(2)
        const contract2 = await deployBuyAndSell()
        const approve = await contract1.connect(account1).setApprovalForAll(contract2.address,true)
        const changeBuyAndSellAddress = await contract1.connect(owner).changeBuyAndSellAddress(contract2.address,contract2.address)
        const putProductToSell = await contract2.connect(account1).putProductToSell(1,9000000000)
        const product = await contract2.connect(account1).getProductDetail(1)
        const fristBalance = await contract1.balanceOf(account1.address,1)
        const cancelSell = await contract2.connect(account1).cancelSell(1)
        expect(await contract1.balanceOf(account1.address,1)).to.equal(1)
    })
    it("Only the owner of this product can cancel", async function () {
        [owner, account1, account2, account3] = await ethers.getSigners()
        const contract1 = await deployGetFreeMint()
        const getFreeItem = await contract1.connect(account1).getFreeItem(2)
        const contract2 = await deployBuyAndSell()
        const approve = await contract1.connect(account1).setApprovalForAll(contract2.address,true)
        const changeBuyAndSellAddress = await contract1.connect(owner).changeBuyAndSellAddress(contract2.address,contract2.address)
        const putProductToSell = await contract2.connect(account1).putProductToSell(1,9000000000)
        const product = await contract2.connect(account1).getProductDetail(1)
        const fristBalance = await contract1.balanceOf(account1.address,1)
        
        await expect(contract2.connect(account2).cancelSell(1)).to.be.revertedWith("not S")
    })
    it("Only the owner of this product can cancel", async function () {
        [owner, account1, account2, account3] = await ethers.getSigners()
        const contract1 = await deployGetFreeMint()
        const getFreeItem = await contract1.connect(account1).getFreeItem(2)
        const contract2 = await deployBuyAndSell()
        const approve = await contract1.connect(account1).setApprovalForAll(contract2.address,true)
        const changeBuyAndSellAddress = await contract1.connect(owner).changeBuyAndSellAddress(contract2.address,contract2.address)
        const putProductToSell = await contract2.connect(account1).putProductToSell(1,9000000000)
        const product = await contract2.connect(account1).getProductDetail(1)
        const fristBalance = await contract1.balanceOf(account1.address,1)
        
        await expect(contract2.connect(account2).cancelSell(1)).to.be.revertedWith("not S")
    })
    it("Revert if the product has been sold", async function () {
        [owner, account1, account2, account3] = await ethers.getSigners()
        const contract1 = await deployGetFreeMint()
        const getFreeItem = await contract1.connect(account1).getFreeItem(2)
        const contract2 = await deployBuyAndSell()
        const approve = await contract1.connect(account1).setApprovalForAll(contract2.address,true)
        const changeBuyAndSellAddress = await contract1.connect(owner).changeBuyAndSellAddress(contract2.address,contract2.address)
        const putProductToSell = await contract2.connect(account1).putProductToSell(1,9000000000)
        const product = await contract2.connect(account1).getProductDetail(1)
        const purchase = await contract2.connect(account2).purchaseProduct(1,{value: product.price})
       
        
        await expect(contract2.connect(account1).cancelSell(1)).to.be.revertedWith("sold out")
    })

})

