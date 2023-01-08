import { BigNumber } from 'ethers';
import { ethers } from 'hardhat';
import { DortzioNFTFactory__factory, DortzioNFTMarketplace__factory } from '../typechain';


async function main() {

  const signers = await ethers.getSigners();
  const DortNFTFactory = new DortzioNFTFactory__factory(signers[0]);
  const dortzioNFTFactory = await DortNFTFactory.deploy();
  await dortzioNFTFactory.deployed();
  console.log('DortzioNFTFactory deployed to: ', dortzioNFTFactory.address);

  const DortzioNFTMarketplace = new DortzioNFTMarketplace__factory(signers[0]);
  const platformFee = BigNumber.from(10); // 10%
  const feeRecipient = signers[0].address;
  const dortzioNFTMarketplace =  await DortzioNFTMarketplace.deploy(platformFee, feeRecipient, dortzioNFTFactory.address);
  await dortzioNFTMarketplace.deployed();
  console.log('DortzioNFTMarketplace deployed to: ', dortzioNFTMarketplace.address);
  
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;

})