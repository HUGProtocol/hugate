// SPDX-License-Identifier: MIT
pragma solidity ^0.8.7;

interface INftTokenIdHook {
    function tokenURI(
        address nftAddress, //nft contract address 
        address caller,  
        address nftOwner, //new owner of the nft token
        uint256 tokenId 
    ) external view returns (string memory);
}  