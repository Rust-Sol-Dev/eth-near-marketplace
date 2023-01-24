const {deployNFTMint} = require ("./deploy")
const { ethers } = require("hardhat");


async function NFT(){
    const contract = await deployNFTMint()
    console.log(contract.address);
}

NFT()