const { assert } = require('console')
const { Contract } = require('ethers')
const { ethers, upgrades } = require('hardhat')
const { address } = require("hardhat/internal/core/config/config-validation");

const decimal = ethers.utils.parseUnits('1.0', 16);
const price = decimal.mul(1);

// //update Thoughts
async function update() {

    const [deployer] = await ethers.getSigners();

    console.log("Deploying contracts with the account:", deployer.address);

    console.log("Account balance:", (await deployer.getBalance()).toString());

    const thoughtSol = await ethers.getContractFactory('Thoughts');
    const ThoughtContract = await upgrades.upgradeProxy("0x1c433a917314Ad73E1b9878De71fccE629cCB198", thoughtSol);
    await ThoughtContract.deployed();
    console.log("Thoughts address", ThoughtContract.address);
}

// deploy Thoughts
async function deploy() {
    const [deployer] = await ethers.getSigners();

    console.log("Deploying contracts with the account:", deployer.address);

    console.log("Account balance:", (await deployer.getBalance()).toString());

    //deploy cuckoo
    const thoughtSol = await ethers.getContractFactory('Thoughts');
    const ThoughtContract = await upgrades.deployProxy(thoughtSol);
    await ThoughtContract.deployed();
    console.log("Thoughts address", ThoughtContract.address);
}


// new collection
async function newCollection() {
    const [deployer] = await ethers.getSigners();

    console.log("Calling contracts with the account:", deployer.address);

    console.log("Account balance:", (await deployer.getBalance()).toString());

    const thoughtSol = await ethers.getContractFactory('Thoughts');
    const ThoughtContract = await thoughtSol.attach("0x1c433a917314Ad73E1b9878De71fccE629cCB198");

    const tokenUri = "http://18.118.160.68:6137/ipfs/QmVUF8AGz4vLL1u5J269g9xjMxgpCYJgr5oUa2MwnPgoGt";

    const txNewCollection = await ThoughtContract.connect(deployer).newCollection(tokenUri, price.toHexString(),
        "0x0000000000000000000000000000000000000000", price.toHexString());
    await txNewCollection.wait();

    const myInfo = await ThoughtContract.connect(deployer).checkUserCollections(deployer.address);
    console.log("my collections", myInfo);
}


//buy collection 
async function buy() {
    const [deployer] = await ethers.getSigners();

    console.log("Calling contracts with the account:", deployer.address);

    console.log("Account balance:", (await deployer.getBalance()).toString());

    const thoughtSol = await ethers.getContractFactory('Thoughts');
    const ThoughtContract = await thoughtSol.attach("0x1c433a917314Ad73E1b9878De71fccE629cCB198");

    const tokenId = ethers.BigNumber.from(0);

    const txNewCollection = await ThoughtContract.connect(deployer).collectThoughts(
        tokenId.toHexString(),
        ethers.BigNumber.from(10).toHexString(),
        { value: price.mul(10) }
    );
    await txNewCollection.wait();

    const myInfo = await ThoughtContract.connect(deployer).checkUserCollections(deployer.address);
    console.log("my collections", myInfo);

    const collectionInfo = await ThoughtContract.connect(deployer).getCollectionInfo(deployer.address, tokenId.toHexString());
    console.log("token 0", collectionInfo);
}

update()
    .then(() => process.exit(0))
    .catch((error) => {
        console.error(error);
        process.exit(1);
    });