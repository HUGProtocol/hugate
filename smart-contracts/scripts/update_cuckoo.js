const { assert } = require('console')
const { Contract } = require('ethers')
const { ethers, upgrades } = require('hardhat')
const { address } = require("hardhat/internal/core/config/config-validation");

const contract_address = process.env.CONTRACT_ADDRESS;

//update Cuckoo
async function update() {

    const [deployer] = await ethers.getSigners();

    console.log("Deploying contracts with the account:", deployer.address);

    console.log("Account balance:", (await deployer.getBalance()).toString());

    const cuckooSol = await ethers.getContractFactory('Cuckoo');
    const CuckooContract = await upgrades.upgradeProxy(contract_address, cuckooSol);
    await CuckooContract.deployed();
    console.log("Cuckoo address", CuckooContract.address);
}

update()
    .then(() => process.exit(0))
    .catch((error) => {
        console.error(error);
        process.exit(1);
    });