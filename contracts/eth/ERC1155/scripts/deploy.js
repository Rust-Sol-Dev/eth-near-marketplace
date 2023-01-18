// We require the Hardhat Runtime Environment explicitly here. This is optional
// but useful for running the script in a standalone fashion through `node <script>`.
//
// When running the script with `npx hardhat run <script>` you'll find the Hardhat
// Runtime Environment's members available in the global scope.
const { ethers } = require("hardhat");
const hre = require("hardhat");
let getFreeItemAddress=""

// ------------- FOR TESTS -------------
// async function deployGetFreeMint(){
//   [owner, account1, account2, account3] = await ethers.getSigners()
//   const getFreeItem = await ethers.getContractFactory("GetFreeItems")
//   const getFreeItemContract = await getFreeItem.connect(owner).deploy()
//   getFreeItemAddress =  getFreeItemContract.address
//   const lastGetFreeItemContract = await ethers.getContractAt("GetFreeItems",getFreeItemAddress)
//   console.log("GetFreeItem deployed At ",getFreeItemAddress)
//   return lastGetFreeItemContract
// }

// async function deployBuyAndSell(){
//   const owner = await ethers.getSigner()
//   const buyAndSell = await ethers.getContractFactory("BuyAndSell")  
//   const buyAndSellContract = await buyAndSell.connect(owner).deploy(getFreeItemAddress)
//   const lastedDeployedBuyAndSell = await ethers.getContractAt("BuyAndSell",buyAndSellContract.address)
//   console.log("buy and sell ",buyAndSellContract.address);
//   return lastedDeployedBuyAndSell
// }

// async function deployAuctionAndBid(){
//   const owner = await ethers.getSigner()
//   const auctionAndBid = await ethers.getContractFactory("AuctionAndBids")
//   const auctionAndBidsContract = await auctionAndBid.connect(owner).deploy(getFreeItemAddress)
//   const lastedDeployedAuctionAndBid = await ethers.getContractAt("AuctionAndBids",auctionAndBidsContract.address)
//   console.log("Auction and Bids ",auctionAndBidsContract.address);
//   return lastedDeployedAuctionAndBid
// }
//  ------------ END --------------
// FOR WEBSITE 
async function deployGetFreeMint2(){
  [owner, account1, account2, account3] = await ethers.getSigners()
  const getFreeItem = await ethers.getContractFactory("GetFreeItems")
  const getFreeItemContract = await getFreeItem.connect(owner).deploy()
  getFreeItemAddress =  getFreeItemContract.address
  const lastGetFreeItemContract = await ethers.getContractAt("GetFreeItems",getFreeItemAddress)
  console.log("GetFreeItem deployed At ",getFreeItemAddress)
  return lastGetFreeItemContract
}

async function deployBuyAndSell2(){
  const owner = await ethers.getSigner()
  const buyAndSell = await ethers.getContractFactory("BuyAndSell")  
  const buyAndSellContract = await buyAndSell.connect(owner).deploy("0x5406e19c95DCB5F37b62b6fd5b506ed533fbb6f9")
  const lastedDeployedBuyAndSell = await ethers.getContractAt("BuyAndSell",buyAndSellContract.address)
  console.log("buy and sell ",buyAndSellContract.address);
  return lastedDeployedBuyAndSell
}

async function deployAuctionAndBid2(){
  const owner = await ethers.getSigner()
  const auctionAndBid = await ethers.getContractFactory("AuctionAndBids")
  const auctionAndBidsContract = await auctionAndBid.connect(owner).deploy("0x5406e19c95DCB5F37b62b6fd5b506ed533fbb6f9")
  const lastedDeployedAuctionAndBid = await ethers.getContractAt("AuctionAndBids",auctionAndBidsContract.address)
  console.log("Auction and Bids ",auctionAndBidsContract.address);
  return lastedDeployedAuctionAndBid
}
// give permission to buyandsell & auctionandbids contracts to change state in FreeItem contrct
async function allowedContracts(){
  [owner, account1, account2, account3] = await ethers.getSigners()
  const getFContract = await ethers.getContractAt("GetFreeItems","0x5406e19c95DCB5F37b62b6fd5b506ed533fbb6f9")

  const change = await getFContract.connect(owner).changeBuyAndSellAddress("0x9f20b8D4d1b3209D29A235002d1B274407866C81","0xF9D5867d180FCeC0900a74406b696E42B81CF6a0")

}



//deployGetFreeMint2()
//deployBuyAndSell2()
 //deployAuctionAndBid2()
//allowedContracts()
module.exports = {
  // deployGetFreeMint,deployBuyAndSell,deployAuctionAndBid,
  deployGetFreeMint2,deployBuyAndSell2,deployAuctionAndBid2
}