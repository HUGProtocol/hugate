// SPDX-License-Identifier: MIT
pragma solidity ^0.8.7;

import "@openzeppelin/contracts-upgradeable/token/ERC1155/ERC1155Upgradeable.sol";
import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/utils/introspection/ERC165StorageUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/token/ERC1155/extensions/ERC1155URIStorageUpgradeable.sol";
import "@openzeppelin/contracts/utils/Counters.sol";
import "@openzeppelin/contracts-upgradeable/token/ERC20/ERC20Upgradeable.sol";
import "@openzeppelin/contracts-upgradeable/token/ERC20/IERC20Upgradeable.sol";
import "@openzeppelin/contracts-upgradeable/token/ERC20/utils/SafeERC20Upgradeable.sol";
import "./CoreFactory.sol";
import "./interfaces/hooks/IUserProfile.sol";

contract Thoughts is
    ERC1155URIStorageUpgradeable,
    OwnableUpgradeable,
    ERC165StorageUpgradeable
{
    using SafeERC20Upgradeable for IERC20Upgradeable;
    using Counters for Counters.Counter;
    struct CollectionBasic {
        address payable owner;
        uint256 price;
        uint256 thoughtCount;
        address token;
    }

    struct ThoughtInfo {
        uint256 tokenId;
        address owner;
        uint256 total;
        uint256 amount;
        uint256 max;
        string tokenURI;
        uint256 price;
        address token;
    }

    Counters.Counter private _tokenIds;
    Counters.Counter private _version;
    mapping(uint256 => CollectionBasic) public CollectionInfo;
    mapping(address => uint256[]) public OwnedCollections;
    mapping(uint256 => uint256) public MaxCount;

    event UpdateCollectionEvent(
        address indexed owner,
        uint256 price,
        uint256 thoughtCount,
        address indexed token
    );

    event CreateCollectionEvent(address indexed owner, uint256 tokenId);

    event MintEvent(address indexed owner, uint256 tokenId, uint256 amount);

    /// ***********************
    //Core
    /// ***********************
    function initialize() public initializer {
        ERC165StorageUpgradeable.__ERC165Storage_init();
        ERC1155URIStorageUpgradeable.__ERC1155URIStorage_init();
        OwnableUpgradeable.__Ownable_init();
        _version.increment();
    }

    function _beforeMint(uint256 tokenId, uint256 amount) internal virtual {
        CollectionBasic storage basic = CollectionInfo[tokenId];
        basic.thoughtCount += amount;
    }

    function _mint(
        address to,
        uint256 id,
        uint256 amount,
        bytes memory data
    ) internal override {
        _beforeMint(id, amount);
        uint256 am = ERC1155Upgradeable.balanceOf(to, id);
        if (am == 0) {
            uint256[] storage list = OwnedCollections[to];
            list.push(id);
        }
        ERC1155Upgradeable._mint(to, id, amount, data);
        emit MintEvent(to, id, amount);
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

    function _beforeTokenTransfer(
        address operator,
        address from,
        address to,
        uint256[] memory ids,
        uint256[] memory amounts,
        bytes memory data
    ) internal override(ERC1155Upgradeable) {
        for (uint256 i = 0; i < ids.length; ++i) {
            uint256 tokenId = ids[i];
            uint256 b = amounts[i];
            if (b != 0) {
                uint256 amount = ERC1155Upgradeable.balanceOf(to, tokenId);
                if (amount == 0) {
                    uint256[] storage list = OwnedCollections[to];
                    list.push(tokenId);
                }
            }
        }
    }

    receive() external payable {}

    //Contract Owner
    function updateVersion() public onlyOwner {
        _version.increment();
    }

    /// ***********************
    //Channel Owner
    /// ***********************
    //set token url
    function setTokenURI(uint256 tokenId, string memory tokenURI)
        public
        onlyCollectionOwner(tokenId)
    {
        ERC1155URIStorageUpgradeable._setURI(tokenId, tokenURI);
    }

    //init channel
    function newCollection(
        string memory tokenURI,
        uint256 price,
        address payment,
        uint256 amount
    ) public {
        uint256 newId = _tokenIds.current();

        CollectionBasic memory basic = CollectionBasic(
            payable(msg.sender),
            price,
            0,
            payment
        );
        CollectionInfo[newId] = basic;

        setTokenURI(newId, tokenURI);

        _mint(msg.sender, newId, 1, "");
        _tokenIds.increment();
        MaxCount[newId] = amount;
        emit CreateCollectionEvent(msg.sender, newId);
    }

    //update channel basic info
    function updateCollectionBasic(
        uint256 tokenId,
        uint256 price,
        address token
    ) public onlyCollectionOwner(tokenId) {
        // require(token != address(0));
        CollectionBasic storage basic = CollectionInfo[tokenId];
        basic.price = price;
        basic.token = token;
        emit UpdateCollectionEvent(
            msg.sender,
            price,
            basic.thoughtCount,
            basic.token
        );
    }

    function updateCollectionOwner(uint256 tokenId, address payable newOwner)
        public
        onlyCollectionOwner(tokenId)
    {
        require(newOwner != address(0));
        CollectionBasic storage basic = CollectionInfo[tokenId];
        basic.owner = newOwner;
        emit UpdateCollectionEvent(
            msg.sender,
            basic.price,
            basic.thoughtCount,
            basic.token
        );
    }

    /// ***********************
    //Channel Subscriber
    /// ***********************
    function collectThoughts(uint256 tokenId, uint256 amount)
        public
        payable
        onlyCollectionExist(tokenId)
    {
        CollectionBasic memory basic = CollectionInfo[tokenId];
        uint256 max = MaxCount[tokenId];
        require(
            max >= (basic.thoughtCount + amount),
            "expire collection amount"
        );
        if (basic.price != 0) {
            if (basic.token == address(0)) {
                require(
                    msg.value == (basic.price * amount),
                    "value not equal to price"
                );
                (bool sent, ) = basic.owner.call{value: basic.price}("");
                require(sent, "Failed to send Ether");
            } else {
                IERC20Upgradeable paymentToken = IERC20Upgradeable(basic.token);
                paymentToken.safeTransferFrom(
                    msg.sender,
                    basic.owner,
                    basic.price
                );
            }
        }
        _mint(msg.sender, tokenId, amount, "");
    }

    // function batchSend(uint256 tokenId, address[] memory addressList)
    //     public
    //     onlyCollectionExist(tokenId)
    // {
    //     require(addressList.length < 10, "receiver expand");
    //     for (uint256 i = 0; i < addressList.length; ++i) {
    //         address receiver = addressList[i];
    //         uint256 amount = ERC1155Upgradeable.balanceOf(receiver, tokenId);
    //         if (amount == 0) {
    //             uint256[] storage list = OwnedCollections[receiver];
    //             list.push(tokenId);
    //         }
    //         ERC1155Upgradeable.safeTransferFrom(
    //             msg.sender,
    //             receiver,
    //             tokenId,
    //             1,
    //             ""
    //         );
    //     }
    // }

    /// ***********************
    //View Functions
    /// ***********************
    function version() public view returns (uint256) {
        return _version.current();
    }

    function checkUserCollections(address addr)
        public
        view
        returns (ThoughtInfo[] memory)
    {
        uint256[] memory ownedTokens = OwnedCollections[addr];
        ThoughtInfo[] memory passInfos = new ThoughtInfo[](ownedTokens.length);
        for (uint256 i = 0; i < ownedTokens.length; ++i) {
            uint256 tokenId = ownedTokens[i];
            CollectionBasic memory basic = CollectionInfo[tokenId];
            string memory tokenURI = ERC1155URIStorageUpgradeable.uri(tokenId);
            uint256 amount = ERC1155Upgradeable.balanceOf(addr, tokenId);
            uint256 max = MaxCount[tokenId];
            ThoughtInfo memory info = ThoughtInfo(
                tokenId,
                basic.owner,
                basic.thoughtCount,
                amount,
                max,
                tokenURI,
                basic.price,
                basic.token
            );
            passInfos[i] = info;
        }
        return passInfos;
    }

    function getCollectionInfo(address addr, uint256 tokenId)
        public
        view
        onlyCollectionExist(tokenId)
        returns (ThoughtInfo memory)
    {
        CollectionBasic memory basic = CollectionInfo[tokenId];
        string memory tokenURI = ERC1155URIStorageUpgradeable.uri(tokenId);
        uint256 amount = ERC1155Upgradeable.balanceOf(addr, tokenId);
        uint256 max = MaxCount[tokenId];
        ThoughtInfo memory info = ThoughtInfo(
            tokenId,
            basic.owner,
            basic.thoughtCount,
            amount,
            max,
            tokenURI,
            basic.price,
            basic.token
        );
        return info;
    }

    /// ***********************
    /// Modifier
    /// ***********************
    modifier onlyCollectionExist(uint256 channelId) {
        CollectionBasic memory basic = CollectionInfo[channelId];
        require(basic.owner != address(0), "channel not exist");
        _;
    }

    modifier onlyCollectionOwner(uint256 channelId) {
        CollectionBasic memory basic = CollectionInfo[channelId];
        require(msg.sender == basic.owner, "only channel owner");
        _;
    }
}
