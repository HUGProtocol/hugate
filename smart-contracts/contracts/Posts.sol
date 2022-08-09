// SPDX-License-Identifier: MIT
pragma solidity ^0.8.7;


import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/utils/introspection/ERC165StorageUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/token/ERC721/extensions/ERC721URIStorageUpgradeable.sol";
import "@openzeppelin/contracts/utils/Counters.sol";
import "./utils/SolOwnable.sol";

contract Posts is ERC721URIStorageUpgradeable, SolOwnable {
    using Counters for Counters.Counter;
    uint16 public version;

    function setVersion(uint16 newVersion) public onlyOwner {
        version = newVersion;
    }

    Counters.Counter private _tokenIds;

    event NewPost(
        address indexed owner,
        uint256 tokenID,
        string tokenURI
    );

    event EditPost(
        address indexed owner,
        uint256 tokenID,
        string tokenURI
    );

    function post(string memory tokenURI) public onlyOwner {
        uint256 newId = _tokenIds.current();
        ERC721Upgradeable._mint(msg.sender, newId);
        _tokenIds.increment();
        ERC721URIStorageUpgradeable._setTokenURI(newId, tokenURI);
        emit NewPost(msg.sender, newId, tokenURI);
    }

    function editPost(uint256 tokenId, string memory tokenURI) public onlyOwner {
        //require token exist
        require(ERC721Upgradeable._exists(tokenId) == true, "tokenId not exist");
        ERC721URIStorageUpgradeable._setTokenURI(tokenId, tokenURI);
        emit EditPost(msg.sender, tokenId, tokenURI);
    }

    function initialize(address payable nowner) public initializer {
        version = 1;
        SolOwnable.__Ownable_init(nowner);
    }

    function supportsInterface(bytes4 interfaceId)
    public
    view
    virtual
    override(ERC721Upgradeable)
    returns (bool)
    {
        return super.supportsInterface(interfaceId);
    }

    receive() external payable {}

}
