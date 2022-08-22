// SPDX-License-Identifier: MIT
pragma solidity ^0.8.7;


import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/utils/introspection/ERC165StorageUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/token/ERC721/extensions/ERC721URIStorageUpgradeable.sol";
import "@openzeppelin/contracts/utils/Counters.sol";
import "./utils/SolOwnable.sol";

contract Profile is ERC721URIStorageUpgradeable, SolOwnable {
    struct ProfileBasic {
        address owner;
        bool publisher;
        string image;
    }

    address private cuckoo;
    mapping(address => uint256) private UserId;
    mapping(uint256 => ProfileBasic) private Profiles;

    using Counters for Counters.Counter;
    uint16 public version;

    function setVersion(uint16 newVersion) public onlyOwner {
        version = newVersion;
    }

    function updateCuckoo(address new_cuckoo) public onlyOwner {
        cuckoo = new_cuckoo;
    }

    Counters.Counter private _tokenIds;

    function initialize() public initializer {
        version = 1;
        _tokenIds.increment();
        SolOwnable.__Ownable_init(msg.sender);
    }

    function newProfile(string memory tokenUrl) public userNotExist(msg.sender) {
        uint256 newId = _tokenIds.current();
        ERC721Upgradeable._safeMint(msg.sender, newId);
        _tokenIds.increment();
        ERC721URIStorageUpgradeable._setTokenURI(newId, tokenUrl);
        ProfileBasic memory basic = ProfileBasic(msg.sender, false, "");
        Profiles[newId] = basic;
        UserId[msg.sender] = newId;
    }

    function updateImage(string memory image) public userExist(msg.sender) {
        uint256 tokenId = UserId[msg.sender];
        ProfileBasic storage basic = Profiles[tokenId];
        basic.image = image;
    }

    function updateUrl(string memory tokenUrl) public userExist(msg.sender) {
        uint256 tokenId = UserId[msg.sender];
        ERC721URIStorageUpgradeable._setTokenURI(tokenId, tokenUrl);
    }

    function toPublisher(address user) external userExist(user) onlyCuckoo {
        uint256 tokenId = UserId[user];
        ProfileBasic storage basic = Profiles[tokenId];
        basic.publisher = true;
    }

    modifier onlyProfileOwner(uint256 profileId){
        ProfileBasic memory basic = Profiles[profileId];
        require(msg.sender == basic.owner, "only profile owner");
        _;
    }

    modifier userExist(address user){
        require(UserId[user] != 0, "user not exist");
        _;
    }

    modifier userNotExist(address user){
        require(UserId[user] == 0, "user exist");
        _;
    }

    modifier onlyCuckoo(){
        require(msg.sender == cuckoo, "only cuckoo");
        _;
    }

}