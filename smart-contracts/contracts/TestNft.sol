// SPDX-License-Identifier: MIT
pragma solidity ^0.8.7;

import "@openzeppelin/contracts-upgradeable/token/ERC1155/ERC1155Upgradeable.sol";
import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/utils/introspection/ERC165StorageUpgradeable.sol";

import "./utils/SolOwnable.sol";

contract TestNft is ERC1155Upgradeable, SolOwnable, ERC165StorageUpgradeable {

    function version() public pure returns (uint256){
        return 1;
    }

    function initialize(address payable nowner) public initializer {
        SolOwnable.__Ownable_init(nowner);
    }

    function supportsInterface(bytes4 interfaceId)
    public
    view
    virtual
    override(ERC1155Upgradeable, ERC165StorageUpgradeable)
    returns (bool)
    {
        return super.supportsInterface(interfaceId);
    }

    receive() external payable {}

}
