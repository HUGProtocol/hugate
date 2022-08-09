const {assert} = require('console')
const {Contract} = require('ethers')
const {ethers, upgrades} = require('hardhat')
const {address} = require("hardhat/internal/core/config/config-validation");

async function main() {
    //deploy ERC20 TMP
    const decimal = ethers.utils.parseUnits('1.0', 18);
    const [owner, Alice, Bob] = await ethers.getSigners();
    console.log("owner address", owner.address);
    console.log("Alice address", Alice.address);
    console.log("Bob address", Bob.address);
    const ERC20Sol = await ethers.getContractFactory('ERC20Tmpl');
    const initialSupply = decimal.mul(10000);
    const ERC20Contract = await ERC20Sol.deploy(initialSupply.toHexString());
    await ERC20Contract.deployed();
    console.log("ERC20 TMP address", ERC20Contract.address);
    await ERC20Contract.connect(owner).balanceOf(owner.address);

    //transfer 100 TMP to Alice
    const amount = decimal.mul(100);
    const txTransferAlice = await ERC20Contract.connect(owner).transfer(Alice.address, amount.toHexString());
    await txTransferAlice.wait();

    //transfer 100 TMP to Bob
    const txBob = await ERC20Contract.connect(owner).transfer(Bob.address, amount.toHexString());
    await txBob.wait();

    //deploy Cuckoo
    const cuckooSol = await ethers.getContractFactory('Cuckoo');
    const CuckooContract = await upgrades.deployProxy(cuckooSol);
    await CuckooContract.deployed();
    console.log("Cuckoo address", CuckooContract.address);

    //deploy post
    const PostsSol = await ethers.getContractFactory('Posts');
    const postContract = await PostsSol.deploy();
    await postContract.deployed();

    //add post template to cuckoo
    const new_version = 1;
    const txAddTMP = await CuckooContract.connect(owner).addConTemplate(postContract.address, new_version);
    await txAddTMP.wait();

    //set Alice Publisher
    const txAddPublisher = await CuckooContract.connect(owner).addPublisher(Alice.address);
    await txAddPublisher.wait();

    //Alice new channel
    const channelURI = "ipfs://QmeSjSinHpPnmXmspMjwiXyN6zS4E9zccariGR3jxcaWtq/6476";
    const price = decimal.mul(10);
    await CuckooContract.connect(Alice).newChannel(channelURI, price.toHexString(), ERC20Contract.address);

    //Bob approve and mint Alice channel pass
    const approveAmount = decimal.mul(1000);
    const txApprove = await ERC20Contract.connect(Bob).approve(CuckooContract.address, approveAmount.toHexString());
    await txApprove.wait();

    const txSubscribe = await CuckooContract.connect(Bob).subscribeChannel(ethers.BigNumber.from(0).toHexString());
    await txSubscribe.wait();

    //check Bob tokenamount
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