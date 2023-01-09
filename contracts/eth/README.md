

## Notes

* change the `--network` parameter to `mainnet` for production.

* remember to fill the `.env` file with appropriate parameters.

* remember to charge your wallet on testnet using https://goerlifaucet.com/

* last deploy:
```console
DortzioNFTFactory deployed to:  0x43C9b9B30178375d40eB41F30D40A0B0F47a8748
DortzioNFTMarketplace deployed to:  0xDBB6399E7cb66E5469bD211Cd7d3F5EAd7136C92
```

## Setup

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

## WIP

* ERC1155

* Contract Method Tests

* Deploy Contracts

## References

* https://coinsbench.com/fully-decentralized-erc-721-and-erc-1155-nfts-6c229adf9c9b

* https://github.com/cryptonomicon46/ERC1155

* https://github.com/kofkuiper/kuiper-nft-marketplace

* https://github.com/OMGWINNING/NFT-Marketplace-Tutorial/

* https://github.com/Markkop/nft-marketplace/

* https://github.com/fjun99/nftmarketplace/tree/main/contracts

* https://trufflesuite.com/guides/nft-marketplace/