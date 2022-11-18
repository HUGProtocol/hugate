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

contract Cuckoo is
    ERC1155URIStorageUpgradeable,
    OwnableUpgradeable,
    ERC165StorageUpgradeable
{
    using SafeERC20Upgradeable for IERC20Upgradeable;
    using Counters for Counters.Counter;
    struct ChannelBasic {
        address payable owner;
        uint256 price;
        uint256 passCount;
        address token;
    }

    struct PassInfo {
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
    mapping(uint256 => ChannelBasic) public ChannelInfo;
    mapping(address => uint256[]) public OwnedChannels;
    mapping(uint256 => uint256) public MaxCount;

    event UpdateChannelEvent(
        address indexed owner,
        uint256 price,
        uint256 passCount,
        address indexed token
    );

    event CreateChannelEvent(address indexed owner, uint256 tokenId);

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
        ChannelBasic storage basic = ChannelInfo[tokenId];
        basic.passCount += amount;
        uint256 max = MaxCount[tokenId];
        require(basic.passCount <= max);
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
            uint256[] storage list = OwnedChannels[to];
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
        onlyChannelOwner(tokenId)
    {
        ERC1155URIStorageUpgradeable._setURI(tokenId, tokenURI);
    }

    //init channel
    function newChannel(
        string memory tokenURI,
        uint256 price,
        address payment,
        uint256 amount
    ) public {
        //new channel token
        // require(payment != address(0));
        uint256 newId = _tokenIds.current();

        ChannelBasic memory basic = ChannelBasic(
            payable(msg.sender),
            price,
            0,
            payment
        );
        ChannelInfo[newId] = basic;

        setTokenURI(newId, tokenURI);

        MaxCount[newId] = amount;
        _mint(msg.sender, newId, 1, "");
        _tokenIds.increment();

        emit CreateChannelEvent(msg.sender, newId);
    }

    //update channel basic info
    function updateChannelBasic(
        uint256 tokenId,
        uint256 price,
        address token
    ) public onlyChannelOwner(tokenId) {
        // require(token != address(0));
        ChannelBasic storage basic = ChannelInfo[tokenId];
        basic.price = price;
        basic.token = token;
        emit UpdateChannelEvent(
            msg.sender,
            price,
            basic.passCount,
            basic.token
        );
    }

    function updateChannelOwner(uint256 tokenId, address payable newOwner)
        public
        onlyChannelOwner(tokenId)
    {
        require(newOwner != address(0));
        ChannelBasic storage basic = ChannelInfo[tokenId];
        basic.owner = newOwner;
        emit UpdateChannelEvent(
            msg.sender,
            basic.price,
            basic.passCount,
            basic.token
        );
    }

    /// ***********************
    //Channel Subscriber
    /// ***********************
    function subscribeChannel(
        uint256 tokenId,
        address addr,
        uint256 amount
    ) public payable onlyChannelExist(tokenId) {
        ChannelBasic memory basic = ChannelInfo[tokenId];
        require(amount > 0);
        if (basic.price != 0) {
            if (basic.token == address(0)) {
                require(
                    msg.value == basic.price * amount,
                    "value not equal to price"
                );
                (bool sent, ) = basic.owner.call{value: basic.price}("");
                require(sent, "Failed to send Ether");
            } else {
                IERC20Upgradeable paymentToken = IERC20Upgradeable(basic.token);
                paymentToken.safeTransferFrom(
                    msg.sender,
                    basic.owner,
                    basic.price * amount
                );
            }
        }
        _mint(addr, tokenId, amount, "");
    }

    function batchSend(uint256 tokenId, address[] memory addressList)
        public
    // onlyChannelOwner(tokenId)
    {
        // ChannelBasic memory basic = ChannelInfo[tokenId];
        // if (msg.sender == basic.owner) {
        //     uint256 balance = ERC1155Upgradeable.balanceOf(msg.sender, tokenId);
        //     if (balance < addressList.length) {
        //         _mint(msg.sender, tokenId, addressList.length - balance, "");
        //     }
        // }
        require(addressList.length < 10, "receiver expand");
        for (uint256 i = 0; i < addressList.length; ++i) {
            address receiver = addressList[i];
            if (receiver == msg.sender) {
                continue;
            }
            uint256 amount = ERC1155Upgradeable.balanceOf(receiver, tokenId);
            if (amount == 0) {
                uint256[] storage list = OwnedChannels[receiver];
                list.push(tokenId);
            }
            ERC1155Upgradeable.safeTransferFrom(
                msg.sender,
                receiver,
                tokenId,
                1,
                ""
            );
        }
    }

    /// ***********************
    //View Functions
    /// ***********************
    function version() public view returns (uint256) {
        return _version.current();
    }

    function checkPass(address addr) public view returns (PassInfo[] memory) {
        uint256[] memory ownedTokens = OwnedChannels[addr];
        PassInfo[] memory passInfos = new PassInfo[](ownedTokens.length);
        for (uint256 i = 0; i < ownedTokens.length; ++i) {
            uint256 tokenId = ownedTokens[i];
            ChannelBasic memory basic = ChannelInfo[tokenId];
            string memory tokenURI = ERC1155URIStorageUpgradeable.uri(tokenId);
            uint256 amount = ERC1155Upgradeable.balanceOf(addr, tokenId);
            uint256 max = MaxCount[tokenId];
            PassInfo memory info = PassInfo(
                tokenId,
                basic.owner,
                basic.passCount,
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

    function getPassInfo(address addr, uint256 tokenId)
        public
        view
        onlyChannelExist(tokenId)
        returns (PassInfo memory)
    {
        ChannelBasic memory basic = ChannelInfo[tokenId];
        string memory tokenURI = ERC1155URIStorageUpgradeable.uri(tokenId);
        uint256 amount = ERC1155Upgradeable.balanceOf(addr, tokenId);
        uint256 max = MaxCount[tokenId];
        PassInfo memory info = PassInfo(
            tokenId,
            basic.owner,
            basic.passCount,
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
    modifier onlyChannelExist(uint256 channelId) {
        ChannelBasic memory basic = ChannelInfo[channelId];
        require(basic.owner != address(0), "channel not exist");
        _;
    }

    modifier onlyChannelOwner(uint256 channelId) {
        ChannelBasic memory basic = ChannelInfo[channelId];
        require(msg.sender == basic.owner, "only channel owner");
        _;
    }
}
