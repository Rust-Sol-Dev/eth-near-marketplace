const { expect, assert } = require("chai");
const { ethers } = require("hardhat");
const { deployGetFreeMint } = require("../scripts/deploy")
const hre = require("hardhat");

describe("Get Free Items", function () {
    it("should the user hold item id = 1 ", async function () {
  
      const account1 = await ethers.getSigner()
      const contract = await deployGetFreeMint()
      const getFreeItem = await contract.connect(account1).getFreeItem(2)
      const balance = await contract.balanceOf(account1.address,1)
      expect(balance).to.equal(1)
      
    })
    it("the owner of the item.id =1 should be the user who minted the item",async function(){
      const account1 = await ethers.getSigner()
      const contract = await deployGetFreeMint()
      const getFreeItem = await contract.connect(account1).getFreeItem(2)
      const itemDetails = await contract.connect(account1).getItemDetails(1)
      expect(itemDetails.owner).to.equal(account1.address)
    })

    it("the user should have 1 item in inventory",async function(){
      const account1 = await ethers.getSigner()
      const contract = await deployGetFreeMint()
      const getFreeItem = await contract.connect(account1).getFreeItem(2)
      const getInventory = await contract.connect(account1).getUserInventory(account1.address)
      expect(getInventory.length).to.equal(1)
    })

    it("total items minted should be 1",async function(){
      const account1 = await ethers.getSigner()
      const contract = await deployGetFreeMint()
      const getFreeItem = await contract.connect(account1).getFreeItem(2)
      const totalItemsMinted = await contract.connect(account1).totalItemsMinted()
      expect(totalItemsMinted).to.equal(1)
    })

    it("should updtae address of buy&sell and Auction&Bids",async function(){
    [owner, account1, account2, account3] = await ethers.getSigners()
      const contract = await deployGetFreeMint()
      const changeBuyAndSellAddress = await contract.connect(owner).changeBuyAndSellAddress(contract.address,owner.address)
      expect(await contract.buyAndSellAddress()).to.equal(contract.address)
      expect(await contract.auctionAddress()).to.equal(owner.address)
    })
    it("should reveted because the caller is not the admin",async function(){
      [owner, account1, account2, account3] = await ethers.getSigners()
        const contract = await deployGetFreeMint()
       await expect( contract.connect(account2).changeBuyAndSellAddress(contract.address,owner.address)).to.be.revertedWith('Ownable: caller is not the owner')
        
      })


    it("the owner should be the new owner in mapping",async function(){
      [owner, account1, account2, account3] = await ethers.getSigners()
      const contract = await deployGetFreeMint()
      const getFreeItem = await contract.connect(account1).getFreeItem(2)
      const changeBuyAndSellAddress = await contract.connect(owner).changeBuyAndSellAddress(contract.address,owner.address)
      const changeOwner = await contract.connect(owner).changeOwner(account2.address,1)
      const itemDetails = await contract.connect(owner).getItemDetails(1)
      expect(itemDetails.owner).to.equal(account2.address)

    })

    it("only allowed address can change state and owner",async function(){
      [owner, account1, account2, account3] = await ethers.getSigners()
      const contract = await deployGetFreeMint()
      const getFreeItem = await contract.connect(account1).getFreeItem(2)
      const changeBuyAndSellAddress = await contract.connect(owner).changeBuyAndSellAddress(contract.address,owner.address)

      await expect(contract.connect(account1).changeOwner(account2.address,1)).to.be.revertedWith("not allowed")

    })
  })