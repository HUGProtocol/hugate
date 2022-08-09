// SPDX-License-Identifier: MIT
pragma solidity ^0.8.7;

interface INftMintHook {
    function tokenURI(
        address nftAddress, //nft contract address 
        address caller,  
        address nftOwner, //new owner of the nft token
        uint256 tokenId 
    ) external view returns (string memory);

    //check mint perm
    function beforeMint(
        address nftAddress, //nft contract address 
        address caller,  
        address nftOwner, //new owner of the nft token
        uint256 tokenId, //new token id or 1155
        uint256 balance
    )external;

    function afterMint(
        address nftAddress, //nft contract address 
        address caller,  
        address nftOwner, //new owner of the nft token
        uint256 tokenId, //new token id or 1155
        uint256 balance
    )external;

    
}  