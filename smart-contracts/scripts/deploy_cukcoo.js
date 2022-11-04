const { assert } = require('console')
const { Contract } = require('ethers')
const { ethers, upgrades } = require('hardhat')
const { address } = require("hardhat/internal/core/config/config-validation");

//update Cuckoo
async function update() {

    const [deployer] = await ethers.getSigners();

    console.log("Deploying contracts with the account:", deployer.address);

    console.log("Account balance:", (await deployer.getBalance()).toString());

    const cuckooSol = await ethers.getContractFactory('Cuckoo');
    const CuckooContract = await upgrades.upgradeProxy("0xd29C5baBfb1E382Cc1e0a7E575A4a45a1bAaA64F", cuckooSol);
    await CuckooContract.deployed();
    console.log("Cuckoo address", CuckooContract.address);
}

//deploy Cuckoo
async function deploy() {
    const [deployer] = await ethers.getSigners();

    console.log("Deploying contracts with the account:", deployer.address);

    console.log("Account balance:", (await deployer.getBalance()).toString());

    //deploy cuckoo
    const cuckooSol = await ethers.getContractFactory('Cuckoo');
    const CuckooContract = await upgrades.deployProxy(cuckooSol);
    await CuckooContract.deployed();
    console.log("Cuckoo address", CuckooContract.address);
}

// newchannel
async function new_channel() {
    const [deployer] = await ethers.getSigners();

    console.log("Calling contracts with the account:", deployer.address);

    console.log("Account balance:", (await deployer.getBalance()).toString());

    const cuckooSol = await ethers.getContractFactory('Cuckoo');
    const CuckooContract = await cuckooSol.attach("0xd29C5baBfb1E382Cc1e0a7E575A4a45a1bAaA64F");

    const tokenUri = "http://18.118.160.68:6137/ipfs/QmVUF8AGz4vLL1u5J269g9xjMxgpCYJgr5oUa2MwnPgoGt";
    const decimal = ethers.utils.parseUnits('1.0', 16);
    const price = decimal.mul(10);
    const txNewChan2 = await CuckooContract.connect(deployer).newChannel(tokenUri, price.toHexString(), 
    "0x509Ee0d083DdF8AC028f2a56731412edD63223B9", ethers.BigNumber.from(100).toHexString());
    await txNewChan2.wait();

    const channelInfo = await CuckooContract.connect(deployer).checkPass(deployer.address);
    console.log(channelInfo);
}

//test checkPass
async function checkPass() {
    //deploy Cuckoo
    const [deployer] = await ethers.getSigners();

    console.log("Deploying contracts with the account:", deployer.address);

    console.log("Account balance:", (await deployer.getBalance()).toString());

    const cuckooSol = await ethers.getContractFactory('Cuckoo');
    const CuckooContract = await cuckooSol.attach("0xd29C5baBfb1E382Cc1e0a7E575A4a45a1bAaA64F");
    const newVersion = await CuckooContract.connect(deployer).version();
    console.log(newVersion);
    console.log(CuckooContract.address);
    const passInfo = await CuckooContract.connect(deployer).checkPass(deployer.address);
    console.log(passInfo);
    const singlePassInfo = await CuckooContract.connect(deployer).getPassInfo(deployer.address,ethers.BigNumber.from(0).toHexString());
    console.log(singlePassInfo);
}

checkPass()
    .then(() => process.exit(0))
    .catch((error) => {
        console.error(error);
        process.exit(1);
    });