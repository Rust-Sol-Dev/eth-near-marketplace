const {deployGetFreeMint} = require ("./deploy")
const { ethers } = require("hardhat");


async function getFreeItem(){
    const contract = await deployGetFreeMint()
    console.log(contract.address);
}

getFreeItem()