const { assert } = require('console')
const { Contract } = require('ethers')
const { ethers, upgrades } = require('hardhat')
const { address } = require("hardhat/internal/core/config/config-validation");

// async function main() {
//     //deploy Cuckoo
//     const [deployer] = await ethers.getSigners();

//     console.log("Deploying contracts with the account:", deployer.address);

//     console.log("Account balance:", (await deployer.getBalance()).toString());

//     const cuckooSol = await ethers.getContractFactory('Cuckoo');
//     const CuckooContract = await upgrades.deployProxy(cuckooSol);
//     await CuckooContract.deployed();
//     console.log("Cuckoo address", CuckooContract.address);
// }

// async function main() {
//     //deploy Cuckoo
//     const [deployer] = await ethers.getSigners();

//     console.log("Deploying contracts with the account:", deployer.address);

//     console.log("Account balance:", (await deployer.getBalance()).toString());

//     const cuckooSol = await ethers.getContractFactory('Cuckoo');
//     const CuckooContract = await cuckooSol.attach("0x1c433a917314Ad73E1b9878De71fccE629cCB198");
//     const newVersion = await CuckooContract.connect(deployer).version();
//     console.log(newVersion);
//     console.log(CuckooContract.address);

//     //deploy post
//     const PostsSol = await ethers.getContractFactory('Posts');
//     const postContract = await PostsSol.deploy();
//     await postContract.deployed();
//     console.log("post address", postContract.address);

//     //add post template to cuckoo
//     const new_version = 1;
//     const txAddTMP = await CuckooContract.connect(deployer).addConTemplate(postContract.address, new_version);
//     await txAddTMP.wait();
//     const postAddress = await CuckooContract.connect(deployer).conVersionImplement(new_version);
//     console.log("cuckoo post address",postAddress);
// }


async function main() {
    //deploy Cuckoo
    const [deployer] = await ethers.getSigners();

    console.log("Deploying contracts with the account:", deployer.address);

    console.log("Account balance:", (await deployer.getBalance()).toString());

    const cuckooSol = await ethers.getContractFactory('Cuckoo');
    const CuckooContract = await cuckooSol.attach("0x1c433a917314Ad73E1b9878De71fccE629cCB198");
    const newVersion = await CuckooContract.connect(deployer).version();
    console.log(newVersion);
    console.log(CuckooContract.address);
    
    // const tokenUri = "https://www.baidu.com";
    // const decimal = ethers.utils.parseUnits('1.0', 18);
    // const price = decimal.mul(10);
    // const txNewChan2 = await CuckooContract.connect(deployer).newChannel(tokenUri, price.toHexString(), "0x509Ee0d083DdF8AC028f2a56731412edD63223B9", ethers.BigNumber.from(1).toHexString());
    // await txNewChan2.wait();
    // await expect(txNewChan2).not.to.be.reverted;
    const tokenId = ethers.BigNumber.from(0);
    const channelInfo = await CuckooContract.connect(deployer).ChannelInfo(tokenId);
    console.log(channelInfo);
}

main()
    .then(() => process.exit(0))
    .catch((error) => {
        console.error(error);
        process.exit(1);
    });