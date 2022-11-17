const { assert } = require('console')
const { Contract } = require('ethers')
const { ethers, upgrades } = require('hardhat')
const { address } = require("hardhat/internal/core/config/config-validation");

const decimal = ethers.utils.parseUnits('1.0', 16);
const price = decimal.mul(1);

const contract_address = process.env.CONTRACT_ADDRESS;

// //update Thoughts
async function update() {

    const [deployer] = await ethers.getSigners();

    console.log("Deploying contracts with the account:", deployer.address);

    console.log("Account balance:", (await deployer.getBalance()).toString());

    const thoughtSol = await ethers.getContractFactory('Thoughts');
    const ThoughtContract = await upgrades.upgradeProxy(contract_address, thoughtSol);
    await ThoughtContract.deployed();
    console.log("Thoughts address", ThoughtContract.address);
}

update()
    .then(() => process.exit(0))
    .catch((error) => {
        console.error(error);
        process.exit(1);
    });