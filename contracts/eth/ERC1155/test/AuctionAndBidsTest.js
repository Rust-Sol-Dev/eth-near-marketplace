const { expect, assert } = require("chai");
const { ethers } = require("hardhat");
const { deployGetFreeMint, deployAuctionAndBid } = require("../scripts/deploy")
const hre = require("hardhat");

describe("Create Auction", function () {
    it("smart contract should be the owner of item auction", async function () {

        [owner, account1, account2, account3] = await ethers.getSigners()
        const contract1 = await deployGetFreeMint()
        const getFreeItem = await contract1.connect(account1).getFreeItem(2)
        const contract2 = await deployAuctionAndBid()
        const changeBuyAndSellAddress = await contract1.connect(owner).changeBuyAndSellAddress(contract2.address,contract2.address)
        const approve = await contract1.connect(account1).setApprovalForAll(contract2.address,true)
        const createAuction = await contract2.connect(account1).createAuction(1,1,900000000)
        const erc1155 = await  ethers.getContractAt("IERC1155",contract1.address)
        const contractBalance = await erc1155.balanceOf(contract2.address,1)
        expect(contractBalance).to.equal(1)
    })

    it("Should revert because the price is 0",async function(){
        [owner, account1, account2, account3] = await ethers.getSigners()
        const contract1 = await deployGetFreeMint()
        const getFreeItem = await contract1.connect(account1).getFreeItem(2)
        const contract2 = await deployAuctionAndBid()
        const changeBuyAndSellAddress = await contract1.connect(owner).changeBuyAndSellAddress(contract2.address,contract2.address)
        const approve = await contract1.connect(account1).setApprovalForAll(contract2.address,true)
        await expect(contract2.connect(account1).createAuction(1,1,0)).to.be.revertedWith("price = 0")


    })
})

describe("Start Bids", function () {
    it("Should revert because the amount < highest bid", async function () {

        [owner, account1, account2, account3] = await ethers.getSigners()
        const contract1 = await deployGetFreeMint()
        const getFreeItem = await contract1.connect(account1).getFreeItem(2)
        const contract2 = await deployAuctionAndBid()
        const changeBuyAndSellAddress = await contract1.connect(owner).changeBuyAndSellAddress(contract2.address,contract2.address)
        const approve = await contract1.connect(account1).setApprovalForAll(contract2.address,true)
        const createAuction = await contract2.connect(account1).createAuction(1,1,900000000)
        await expect(contract2.connect(account2).bid(1,{value:9000000})).to.be.revertedWith("value < H.B")

    })
    it("Should revert because the amount < highest bid", async function () {
        [owner, account1, account2, account3] = await ethers.getSigners()
        const contract1 = await deployGetFreeMint()
        const getFreeItem = await contract1.connect(account1).getFreeItem(2)
        const contract2 = await deployAuctionAndBid()
        const changeBuyAndSellAddress = await contract1.connect(owner).changeBuyAndSellAddress(contract2.address,contract2.address)
        const approve = await contract1.connect(account1).setApprovalForAll(contract2.address,true)
        const createAuction = await contract2.connect(account1).createAuction(1,1,900000000)
        await expect(contract2.connect(account2).bid(1,{value:9000000})).to.be.revertedWith("value < H.B")
    })
    it("Should revert because the time end", async function () {
        [owner, account1, account2, account3] = await ethers.getSigners()
        const contract1 = await deployGetFreeMint()
        const getFreeItem = await contract1.connect(account1).getFreeItem(2)
        const contract2 = await deployAuctionAndBid()
        const changeBuyAndSellAddress = await contract1.connect(owner).changeBuyAndSellAddress(contract2.address,contract2.address)
        const approve = await contract1.connect(account1).setApprovalForAll(contract2.address,true)
        const createAuction = await contract2.connect(account1).createAuction(1,1,900000000)
        await network.provider.send("evm_increaseTime", [3700])
        await expect(contract2.connect(account2).bid(1,{value:90000000000})).to.be.revertedWith("time out")
    })
    it("Should revert because is not started", async function () {
        [owner, account1, account2, account3] = await ethers.getSigners()
        const contract1 = await deployGetFreeMint()
        const getFreeItem = await contract1.connect(account1).getFreeItem(2)
        const contract2 = await deployAuctionAndBid()
        const changeBuyAndSellAddress = await contract1.connect(owner).changeBuyAndSellAddress(contract2.address,contract2.address)
        const approve = await contract1.connect(account1).setApprovalForAll(contract2.address,true)
        await network.provider.send("evm_increaseTime", [3700])
        await expect(contract2.connect(account2).bid(1,{value:90000000000})).to.be.revertedWith("!started")
    })

    it("the amount sent is the highest amount and the caller is the highest bidder", async function () {
        [owner, account1, account2, account3] = await ethers.getSigners()
        const contract1 = await deployGetFreeMint()
        const getFreeItem = await contract1.connect(account1).getFreeItem(2)
        const contract2 = await deployAuctionAndBid()
        const changeBuyAndSellAddress = await contract1.connect(owner).changeBuyAndSellAddress(contract2.address,contract2.address)
        const approve = await contract1.connect(account1).setApprovalForAll(contract2.address,true)
        const createAuction = await contract2.connect(account1).createAuction(1,1,900000000)
        await network.provider.send("evm_increaseTime", [3000])
        const amount = await ethers.utils.parseEther("1")
        const bid = await contract2.connect(account2).bid(1,{value:amount})
        const auction = await contract2.connect(account2).getAuctionDetails(1)
        expect(auction.highestBidder).to.equal(account2.address)
        expect(auction.highestBid).to.equal(amount)
    })
})

describe("End Auction", function () {
    it("Should revert because time is not out", async function () {

        [owner, account1, account2, account3] = await ethers.getSigners()
        const contract1 = await deployGetFreeMint()
        const getFreeItem = await contract1.connect(account1).getFreeItem(2)
        const contract2 = await deployAuctionAndBid()
        const changeBuyAndSellAddress = await contract1.connect(owner).changeBuyAndSellAddress(contract2.address,contract2.address)
        const approve = await contract1.connect(account1).setApprovalForAll(contract2.address,true)
        const createAuction = await contract2.connect(account1).createAuction(1,1,900000000)
        await expect(contract2.connect(account1).endAuction(1)).to.be.revertedWith("not yet")
    })
    it("the owner of the item is the highest bidder", async function () {

        [owner, account1, account2, account3] = await ethers.getSigners()
        const contract1 = await deployGetFreeMint()
        const getFreeItem = await contract1.connect(account1).getFreeItem(2)
        const contract2 = await deployAuctionAndBid()
        const changeBuyAndSellAddress = await contract1.connect(owner).changeBuyAndSellAddress(contract2.address,contract2.address)
        const approve = await contract1.connect(account1).setApprovalForAll(contract2.address,true)
        const createAuction = await contract2.connect(account1).createAuction(1,1,900000000)
        await network.provider.send("evm_increaseTime", [3700])
        const endAuction = await contract2.connect(account2).endAuction(1)
        const erc1155 = await  ethers.getContractAt("IERC1155",contract1.address)
        const auction = await contract2.connect(account1).getAuctionDetails(1)
        expect(auction.highestBidder).to.equal(account1.address)

    })

})

describe("Cancel Auction", function () {
    it("Revert if the caller is not the owner of this auction", async function () {

        [owner, account1, account2, account3] = await ethers.getSigners()
        const contract1 = await deployGetFreeMint()
        const getFreeItem = await contract1.connect(account1).getFreeItem(2)
        const contract2 = await deployAuctionAndBid()
        const changeBuyAndSellAddress = await contract1.connect(owner).changeBuyAndSellAddress(contract2.address,contract2.address)
        const approve = await contract1.connect(account1).setApprovalForAll(contract2.address,true)
        const createAuction = await contract2.connect(account1).createAuction(1,1,900000000)
        await expect(contract2.connect(account2).cancelAuction(1)).to.be.revertedWith("you can't")
    })
    it("Revert if it end", async function () {

        [owner, account1, account2, account3] = await ethers.getSigners()
        const contract1 = await deployGetFreeMint()
        const getFreeItem = await contract1.connect(account1).getFreeItem(2)
        const contract2 = await deployAuctionAndBid()
        const changeBuyAndSellAddress = await contract1.connect(owner).changeBuyAndSellAddress(contract2.address,contract2.address)
        const approve = await contract1.connect(account1).setApprovalForAll(contract2.address,true)
        const createAuction = await contract2.connect(account1).createAuction(1,1,900000000)
        await network.provider.send("evm_increaseTime", [3700])

        const endAuction = await contract2.connect(account1).endAuction(1)

        await expect(contract2.connect(account1).cancelAuction(1)).to.be.revertedWith("ended")
    })
    it("should end the auction", async function () {

        [owner, account1, account2, account3] = await ethers.getSigners()
        const contract1 = await deployGetFreeMint()
        const getFreeItem = await contract1.connect(account1).getFreeItem(2)
        const contract2 = await deployAuctionAndBid()
        const changeBuyAndSellAddress = await contract1.connect(owner).changeBuyAndSellAddress(contract2.address,contract2.address)
        const approve = await contract1.connect(account1).setApprovalForAll(contract2.address,true)
        const createAuction = await contract2.connect(account1).createAuction(1,1,900000000)
        const cancel = await contract2.connect(account1).cancelAuction(1)
        const auction = await contract2.connect(account2).getAuctionDetails(1)
        expect(auction.end).to.equal(true)
    })
    it("should give back the item to the seller", async function () {

        [owner, account1, account2, account3] = await ethers.getSigners()
        const contract1 = await deployGetFreeMint()
        const getFreeItem = await contract1.connect(account1).getFreeItem(2)
        const contract2 = await deployAuctionAndBid()
        const changeBuyAndSellAddress = await contract1.connect(owner).changeBuyAndSellAddress(contract2.address,contract2.address)
        const approve = await contract1.connect(account1).setApprovalForAll(contract2.address,true)
        const createAuction = await contract2.connect(account1).createAuction(1,1,900000000)
        const cancel = await contract2.connect(account1).cancelAuction(1)
        const erc1155 = await  ethers.getContractAt("IERC1155",contract1.address)
        const balance = await erc1155.balanceOf(account1.address,1)
        expect(balance).to.equal(1)
    })
    it("revert if there's a bidder", async function () {

        [owner, account1, account2, account3] = await ethers.getSigners()
        const contract1 = await deployGetFreeMint()
        const getFreeItem = await contract1.connect(account1).getFreeItem(2)
        const contract2 = await deployAuctionAndBid()
        const changeBuyAndSellAddress = await contract1.connect(owner).changeBuyAndSellAddress(contract2.address,contract2.address)
        const approve = await contract1.connect(account1).setApprovalForAll(contract2.address,true)
        const createAuction = await contract2.connect(account1).createAuction(1,1,900000000)
        const bid = await contract2.connect(account2).bid(1,{value:9000000000})
        await expect(contract2.connect(account1).cancelAuction(1)).to.be.revertedWith("you can't cancel")

    })

})

describe("Withdraw Bids", function () {
    it("Bidder can withdraw if he's not the highest bidder", async function () {

        [owner, account1, account2, account3] = await ethers.getSigners()
        const contract1 = await deployGetFreeMint()
        const getFreeItem = await contract1.connect(account1).getFreeItem(2)
        const contract2 = await deployAuctionAndBid()
        const changeBuyAndSellAddress = await contract1.connect(owner).changeBuyAndSellAddress(contract2.address,contract2.address)
        const approve = await contract1.connect(account1).setApprovalForAll(contract2.address,true)
        const createAuction = await contract2.connect(account1).createAuction(1,1,900000000)
        const bid = await contract2.connect(account2).bid(1,{value:9000000000})
        const bid1= await contract2.connect(account3).bid(1,{value:90000000000})
        const withdraw  = await contract2.connect(account2).withdrawBids(1)
        const userbalance = await contract2.connect(account2).getBalanceOf(account2.address)
        expect(userbalance).to.equal(0)
    })
    it("Revert because the caller is the highest bidder", async function () {

        [owner, account1, account2, account3] = await ethers.getSigners()
        const contract1 = await deployGetFreeMint()
        const getFreeItem = await contract1.connect(account1).getFreeItem(2)
        const contract2 = await deployAuctionAndBid()
        const changeBuyAndSellAddress = await contract1.connect(owner).changeBuyAndSellAddress(contract2.address,contract2.address)
        const approve = await contract1.connect(account1).setApprovalForAll(contract2.address,true)
        const createAuction = await contract2.connect(account1).createAuction(1,1,900000000)
        const bid = await contract2.connect(account2).bid(1,{value:9000000000})

        await expect(contract2.connect(account2).withdrawBids(1)).to.be.revertedWith("you're H.B")
    })

})