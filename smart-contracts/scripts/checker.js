const {ethers, upgrades} = require('hardhat')
const env = require("hardhat");

async function main() {
    const [owner, Alice, Bob] = await ethers.getSigners();
    const cuckooAddress = process.env.CUCKOO || "0x5FC8d32690cc91D4c39d9d3abcBD16989F875707";
    console.log("cuckoo address", cuckooAddress);
    const cuckooSol = await ethers.getContractFactory('Cuckoo');
    const CuckooContract = cuckooSol.attach(cuckooAddress);
    const tokenId = ethers.BigNumber.from(0);
    const tokenAmount = await CuckooContract.connect(Bob).balanceOf(Bob.address, tokenId.toHexString());
    console.log("Bob channel 0 token amount", tokenAmount);
}

main()
    .then(() => process.exit(0))
    .catch((error) => {
        console.error(error);
        process.exit(1);
    });