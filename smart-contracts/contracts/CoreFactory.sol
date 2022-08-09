// SPDX-License-Identifier: MIT
pragma solidity ^0.8.7;

import "hardhat/console.sol";
import "@openzeppelin/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";
import "@openzeppelin/contracts/proxy/transparent/ProxyAdmin.sol";
import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/token/ERC1155/IERC1155Upgradeable.sol";

contract CoreFactory is Initializable, OwnableUpgradeable {
    ProxyAdmin private proxyAdmin;
    mapping(uint16 => address) public conVersionImplement;
    mapping(address => uint16) private publicImplVersion;
    uint16 public LastestVersion;
    address public ProxyAdminAddress;

    event NewCon(address indexed conOwner, address indexed newConAddress);

    event ConTemplateAdded(address indexed addImpl, uint16 level);

    event ConUpgrade(
        address indexed conAddress,
        address indexed admin,
        uint16 newLevel
    );

    function __FactoryInitialize() internal onlyInitializing {
        LastestVersion = 0;
        OwnableUpgradeable.__Ownable_init();
        _deployProxyAdmin();
    }

    function _createCon() internal returns (address) {
        bytes memory data = abi.encodeWithSignature(
            "initialize(address)",
            msg.sender
        );
        console.log("createUpgradeCon");
        return _createUpgradeCon(data);
    }

    //create and register new proxy
    function _createUpgradeCon(bytes memory data) private returns (address) {
        uint16 version = LastestVersion;
        require(ProxyAdminAddress != address(0), "proxy admin address empty");
        address verImpl = conVersionImplement[version];
        console.log("version %s address %s", version, verImpl);
        require(verImpl != address(0), "con version implementation empty");
        TransparentUpgradeableProxy proxy = new TransparentUpgradeableProxy(
            verImpl,
            ProxyAdminAddress,
            data
        );
        console.log("new proxy address %s", address(proxy));
        emit NewCon(address(proxy), msg.sender);
        return address(proxy);
    }

    function _deployProxyAdmin() private returns (address) {
        proxyAdmin = new ProxyAdmin();
        ProxyAdminAddress = address(proxyAdmin);
        return ProxyAdminAddress;
    }

    function addConTemplate(address impl, uint16 version) public onlyOwner {
        publicImplVersion[impl] = version;
        conVersionImplement[version] = impl;
        if (LastestVersion < version) LastestVersion = version;
        emit ConTemplateAdded(impl, version);
    }

    //can only upgrade to higher implement version
    function upgradeCon(address payable lockAddress, uint16 version)
    public
    returns (address)
    {
        require(lockAddress != address(0));
        TransparentUpgradeableProxy proxy = TransparentUpgradeableProxy(
            lockAddress
        );
        address newImpl = conVersionImplement[version];
        require(newImpl != address(0));
        proxyAdmin.upgrade(proxy, newImpl);
        //        proxy.upgradeTo(newImpl);
        address admin_owner = proxyAdmin.owner();
        console.log("proxy admin owner", admin_owner);
        emit ConUpgrade(lockAddress, msg.sender, version);
        return lockAddress;
    }
}
