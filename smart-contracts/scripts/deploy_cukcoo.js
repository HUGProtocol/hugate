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
    const CuckooContract = await upgrades.upgradeProxy("0x0A14Db069d2b76b7a49EFd4A1bbEedcfe3b49Ab4", cuckooSol);
    await CuckooContract.deployed();
    console.log("Cuckoo address", CuckooContract.address);
}

//deploy Cuckoo
// async function main() {
//     const [deployer] = await ethers.getSigners();

//     console.log("Deploying contracts with the account:", deployer.address);

//     console.log("Account balance:", (await deployer.getBalance()).toString());

//     //deploy post
//     const PostsSol = await ethers.getContractFactory('Posts');
//     const postContract = await PostsSol.deploy();
//     await postContract.deployed();
//     console.log("post address", postContract.address);

//     //deploy cuckoo
//     const cuckooSol = await ethers.getContractFactory('Cuckoo');
//     const CuckooContract = await upgrades.deployProxy(cuckooSol);
//     await CuckooContract.deployed();
//     console.log("Cuckoo address", CuckooContract.address);

//     //add post template to cuckoo
//     const new_version = 1;
//     const txAddTMP = await CuckooContract.connect(deployer).addConTemplate(postContract.address, new_version);
//     await txAddTMP.wait();
//     const postAddress = await CuckooContract.connect(deployer).conVersionImplement(new_version);
//     console.log("cuckoo post address",postAddress);
// }

// newchannel
async function main() {
    const [deployer] = await ethers.getSigners();

    console.log("Calling contracts with the account:", deployer.address);

    console.log("Account balance:", (await deployer.getBalance()).toString());

    const cuckooSol = await ethers.getContractFactory('Cuckoo');
    const CuckooContract = await cuckooSol.attach("0x0A14Db069d2b76b7a49EFd4A1bbEedcfe3b49Ab4");

    const tokenUri = "http://18.118.160.68:6137/ipfs/QmVUF8AGz4vLL1u5J269g9xjMxgpCYJgr5oUa2MwnPgoGt";
    const decimal = ethers.utils.parseUnits('1.0', 18);
    const price = decimal.mul(10);
    const txNewChan2 = await CuckooContract.connect(deployer).newChannel(tokenUri, price.toHexString(), 
    "0x509Ee0d083DdF8AC028f2a56731412edD63223B9", ethers.BigNumber.from(1).toHexString());
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
    const CuckooContract = await cuckooSol.attach("0x0A14Db069d2b76b7a49EFd4A1bbEedcfe3b49Ab4");
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