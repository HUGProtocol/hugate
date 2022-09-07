// SPDX-License-Identifier: MIT
pragma solidity ^0.8.7;


import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/utils/introspection/ERC165StorageUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/token/ERC721/extensions/ERC721URIStorageUpgradeable.sol";
import "@openzeppelin/contracts/utils/Counters.sol";
import "@openzeppelin/contracts-upgradeable/token/ERC20/ERC20Upgradeable.sol";
import "@openzeppelin/contracts-upgradeable/token/ERC20/IERC20Upgradeable.sol";
import "@openzeppelin/contracts-upgradeable/token/ERC20/utils/SafeERC20Upgradeable.sol";
import "./utils/SolOwnable.sol";

contract NFTTree is ERC721URIStorageUpgradeable, SolOwnable {
    using Counters for Counters.Counter;
    using SafeERC20Upgradeable for IERC20Upgradeable;
    uint16 public version;
    string private baseURL;
    address public RootOwner;
    uint256 public Price;
    address public token;
    uint256 private pathReward;
    uint16 private pathMax;

    function setVersion(uint16 newVersion) public onlyOwner {
        version = newVersion;
    }

    Counters.Counter private _tokenIds;
    mapping(uint256 => uint256) private Parents;

    function _baseURI() internal view override(ERC721Upgradeable) returns (string memory){
        return baseURL;
    }

    function setBaseURI(string memory uri) public onlyOwner {
        baseURL = uri;
    }

    function setPrice(uint256 price) public onlyOwner {
        Price = price;
    }

    function mint(uint256 leaf) public {
        uint256 newId = _tokenIds.current();
        _tokenIds.increment();
        ERC721Upgradeable._safeMint(msg.sender, newId);
        Parents[newId] = leaf;
        //iterate parent path to root
        uint256 pathPrice = Price / 100 * pathReward;
        uint256 rootPrice = Price;
        IERC20Upgradeable paymentToken = IERC20Upgradeable(token);
        for (uint i = 0; i < pathMax; i++) {
            uint256 parent = Parents[leaf];
            if (parent == 0) {
                break;
            }
            address parentOwner = ERC721Upgradeable.ownerOf(parent);
            if (parentOwner == address(0)) {
                continue;
            }
            rootPrice -= pathPrice;
            paymentToken.safeTransferFrom(msg.sender, parentOwner, pathPrice);
        }
        paymentToken.safeTransferFrom(msg.sender, RootOwner, rootPrice);
    }

    function initialize(address payable nowner, uint256 price, uint256 pathPer) public initializer {
        version = 1;
        pathMax = 5;
        RootOwner = nowner;
        Price = price;
        pathReward = pathPer;
        //todo:add params
        SolOwnable.__Ownable_init(nowner);
        ERC721URIStorageUpgradeable.__ERC721URIStorage_init();
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
