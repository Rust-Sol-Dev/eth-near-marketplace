pragma solidity ^0.8.4;

import "./DortzioNFT.sol";

/* Dortzio NFT Factory
    Create new Dortzio NFT collection
*/
contract DortzioNFTFactory {
    // owner address => nft list
    mapping(address => address[]) private nfts;
    mapping(address => bool) private dortzioNFT;

    struct RoyaltyInfo {
        address receiver;
        uint256 royaltyFee;
    }

    event CreatedNFTCollection(
        address creator,
        address nft,
        string name,
        string symbol
    );

    function createNFTCollection(
        string memory _name,
        string memory _symbol,
        RoyaltyInfo[] memory royaltyInfo
        // uint256 _royaltyFee,
        // address _royaltyRecipient
    ) external {
        DortzioNFT nft = new DortzioNFT(
            _name,
            _symbol,
            msg.sender,
            royaltyInfo
            // _royaltyFee,
            // _royaltyRecipient
        );
        nfts[msg.sender].push(address(nft));
        dortzioNFT[address(nft)] = true;
        emit CreatedNFTCollection(msg.sender, address(nft), _name, _symbol);
    }

    function getOwnCollections() external view returns (address[] memory) {
        return nfts[msg.sender];
    }

    function isDortzioNFT(address _nft) external view returns (bool) {
        return dortzioNFT[_nft];
    }
}
