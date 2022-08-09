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
    const proxy = await CuckooContract.connect(Alice).channelProxy(tokenId.toHexString());
    console.log("channel proxy address", proxy);
    const postsSol = await ethers.getContractFactory('Posts');
    const postsContract = postsSol.attach(proxy);

    const postURI = "127.0.0.1:8080/ipfs/QmT8Kq56RPj5qLDVbXGdrFg1spXpTPitrQyjJAWAhXzL6B?filename=post_test.json";
    const postTx = await postsContract.connect(Alice).post(postURI);
    await postTx.wait();
    const getPostURI = await postsContract.connect(Bob).tokenURI(ethers.BigNumber.from(1).toHexString());
    console.log("post uri", getPostURI);
}

main()
    .then(() => process.exit(0))
    .catch((error) => {
        console.error(error);
        process.exit(1);
    });