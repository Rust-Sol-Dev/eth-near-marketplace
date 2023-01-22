

## Notes

* contract owner is the collection creator in ERC721

* change the `--network` parameter to `mainnet` for production.

* remember to fill the `.env` file with appropriate parameters.

* remember to charge your wallet on testnet using https://goerlifaucet.com/

* example deploy output:
```console
DortzioNFTFactory deployed to:  0xD10B797Dbf591b1B56F47268e5A2B8c314D9824B
DortzioNFTMarketplace deployed to:  0xC3f16594A6A89Df60A4E3fC2F780496623e27789
```

## Setup

0. Change directory to `ERC721` or `ERC1155`

1. Install packages.
```bash
npm install
```

2. Compile Smart Contract
```bash
npx hardhat compile
```

3. Deploy Smart Contarct
```bash
npx hardhat run scripts/deploy.ts --network goerli
```
4. Test Smart Contract
```bash
npx hardhat test test/dortzio.ts --network goerli
```

# 🍟 Features

* ERC1155 and ERC721 Contracts

* Batch Buying and Minting 

* Deposit and Withdraw Token For Offer

* NFT Update Metadata URI             

* NFT Events   

* List NFT          

* Buy NFT

* Offer NFT

* Accept Offer

* Create Auction 

* Place A Bid

* Royalty 

## References

* https://github.com/nazhG/ERC1155-Marketplace/

* https://github.com/Mowgli9/ERC1155-NFT-Marketplace

* https://github.com/TronzitVeca/ERC1155-Marketplace-Contract/blob/main/contracts/marketplace.sol

* https://github.com/saibaneer/erc1155-Marketplace

* https://coinsbench.com/fully-decentralized-erc-721-and-erc-1155-nfts-6c229adf9c9b

* https://medium.com/coinmonks/how-to-make-your-nfts-reveal-in-solidity-d231ec8413c6

* https://coinsbench.com/fully-decentralized-erc-721-and-erc-1155-nfts-6c229adf9c9b

* https://github.com/kofkuiper/kuiper-nft-marketplace

* https://github.com/OMGWINNING/NFT-Marketplace-Tutorial/

* https://github.com/Markkop/nft-marketplace/

* https://github.com/fjun99/nftmarketplace/tree/main/contracts

* https://trufflesuite.com/guides/nft-marketplace/