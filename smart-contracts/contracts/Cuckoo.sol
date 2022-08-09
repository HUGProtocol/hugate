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

contract Cuckoo is ERC1155URIStorageUpgradeable, OwnableUpgradeable, ERC165StorageUpgradeable, CoreFactory {
    using SafeERC20Upgradeable for IERC20Upgradeable;
    using Counters for Counters.Counter;

    struct ChannelBasic {
        address owner;
        uint256 price;
        uint256 passCount;
        address token;
    }

    Counters.Counter private _tokenIds;
    Counters.Counter private _version;
    mapping(uint256 => ChannelBasic) public ChannelInfo;
    mapping(address => uint256[]) public OwnedChannels;
    mapping(address => bool) private _publisher;
    mapping(uint256 => address) public channelProxy;

    event UpdateChannelEvent(
        address indexed owner,
        uint256 price,
        uint256 passCount,
        address indexed token
    );

    event MintEvent(
        address indexed owner,
        uint256 tokenId,
        uint256 amount
    );

    /// ***********************
    //Core
    /// ***********************
    function initialize() public initializer {
        ERC165StorageUpgradeable.__ERC165Storage_init();
        ERC1155URIStorageUpgradeable.__ERC1155URIStorage_init();
        OwnableUpgradeable.__Ownable_init();
        CoreFactory.__FactoryInitialize();
        _version.increment();
    }

    function _beforeMint(uint256 tokenId, uint256 amount) internal virtual {
        ChannelBasic storage basic = ChannelInfo[tokenId];
        basic.passCount += amount;
    }

    function _mint(
        address to,
        uint256 id,
        uint256 amount,
        bytes memory data) internal override {
        _beforeMint(id, amount);
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

    function addPublisher(address user) public onlyOwner {
        _publisher[user] = true;
    }

    function revokePublisher(address user) public onlyOwner {
        _publisher[user] = false;
    }

    /// ***********************
    //Channel Owner
    /// ***********************
    //set token url
    function setTokenURI(uint256 tokenId, string memory tokenURI) public onlyChannelOwner(tokenId) {
        ERC1155URIStorageUpgradeable._setURI(tokenId, tokenURI);
    }

    //init channel
    function newChannel(string memory tokenURI, uint256 price, address payment) public onlyPublisher {
        //new channel token
        require(payment != address(0));
        uint256 newId = _tokenIds.current();

        ChannelBasic memory basic = ChannelBasic(msg.sender, price, 0, payment);
        ChannelInfo[newId] = basic;

        uint256[] storage list = OwnedChannels[msg.sender];
        list.push(newId);

        setTokenURI(newId, tokenURI);

        _mint(msg.sender, newId, 1, "");
        _tokenIds.increment();

        //new channel contract
        address channelAddress = CoreFactory._createCon();
        channelProxy[newId] = channelAddress;
    }

    //update channel basic info
    function updateChannelBasic(uint256 tokenId, uint256 price, address token) public onlyChannelOwner(tokenId) {
        require(token != address(0));
        ChannelBasic storage basic = ChannelInfo[tokenId];
        basic.price = price;
        basic.token = token;
        emit UpdateChannelEvent(msg.sender, price, basic.passCount, basic.token);
    }

    function updateChannelOwner(uint256 tokenId, address newOwner) public onlyChannelOwner(tokenId) {
        require(newOwner != address(0));
        ChannelBasic storage basic = ChannelInfo[tokenId];
        basic.owner = newOwner;
        emit UpdateChannelEvent(msg.sender, basic.price, basic.passCount, basic.token);
    }

    /// ***********************
    //Channel Subscriber
    /// ***********************
    function subscribeChannel(uint256 tokenId) public onlyChannelExist(tokenId) {
        ChannelBasic memory basic = ChannelInfo[tokenId];
        IERC20Upgradeable paymentToken = IERC20Upgradeable(basic.token);
        paymentToken.safeTransferFrom(msg.sender, basic.owner, basic.price);
        _mint(msg.sender, tokenId, 1, "");
    }

    /// ***********************
    //View Functions
    /// ***********************
    function version() public view returns (uint256){
        return _version.current();
    }

    function ifPublisher() public view returns (bool){
        return _publisher[msg.sender];
    }

    /// ***********************
    /// Modifier
    /// ***********************
    modifier onlyChannelExist(uint256 channelId) {
        ChannelBasic memory basic = ChannelInfo[channelId];
        require(basic.owner != address(0), "channel not exist");
        _;
    }

    modifier onlyPublisher(){
        require(_publisher[msg.sender] == true, "only publisher");
        _;
    }

    modifier onlyChannelOwner(uint256 channelId){
        ChannelBasic memory basic = ChannelInfo[channelId];
        require(msg.sender == basic.owner, "only channel owner");
        _;
    }

}
