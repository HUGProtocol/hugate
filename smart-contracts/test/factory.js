const {expect, use} = require('chai')
const {assert} = require('console')
const {Contract} = require('ethers')
const {ethers, upgrades} = require('hardhat')
const {reverts} = require('./helpers/error')

describe('Public factory template versions', () => {
    let CoreDeployed
    beforeEach(async () => {
        const CoreFactory = await ethers.getContractFactory('CoreFactory');
        const [unlockOwner] = await ethers.getSigners();
        CoreDeployed = await upgrades.deployProxy(CoreFactory);
        await CoreDeployed.deployed();
        console.log("core factory address:", CoreDeployed.address)
        const NftContract = await ethers.getContractFactory('TestNft');
        const publicNft = await NftContract.deploy();
        await publicNft.deployed();
        const [signer] = await ethers.getSigners();
        const new_version = 1;
        await CoreDeployed.connect(signer).addConTemplate(publicNft.address, new_version);

    })
    it('Should set owner to signer', async () => {
        const [signer] = await ethers.getSigners();
        const ownerret = await CoreDeployed.connect(signer).owner();
        console.log("ownerret", ownerret);
        console.log("signer", signer.address);
        expect(await CoreDeployed.connect(signer).owner()).to.equal(signer.address);
    })

    it('Add new implementation', async () => {
        //deploy nft implementation
        const NftContract = await ethers.getContractFactory('TestNft');
        const [unlockOwner] = await ethers.getSigners();
        const publicNft = await NftContract.deploy();
        await publicNft.deployed();
        const [signer] = await ethers.getSigners();
        const new_version = 2;
        await CoreDeployed.connect(signer).addConTemplate(publicNft.address, new_version);
        expect(await CoreDeployed.connect(signer).LastestVersion()).to.equal(new_version);
    })

    it('Init new nft from latest version', async () => {
        const [, , , user] = await ethers.getSigners();
        console.log("user address", user.address)
        const tx = await CoreDeployed.connect(user).createCon();
        const {events: createEvents, gasUsed: gas_used} = await tx.wait();
        console.log("create nft gas used", gas_used)
        const {
            args: {conOwner: newNftAddress, newConAddress: newNftCreator},
        } = createEvents.find(({event}) => event === 'NewCon');
        expect(newNftCreator).to.equal(user.address);
        const nftadd = await CoreDeployed.connect(user).conCreater(user.address);
        expect(nftadd).to.equal(newNftAddress);
        console.log("new nft contract address", newNftAddress);
        const NftContract = await ethers.getContractFactory('TestNft');
        const newNft = NftContract.attach(newNftAddress);
        const newNftOwner = await newNft.connect(user).owner();
        expect(newNftOwner).to.equal(user.address);
        const version = await newNft.connect(user).version();
        console.log("new nft contract version", version);
    })

    it('update nft implementation', async () => {
        //create version 1 contract
        const [, , , user] = await ethers.getSigners();
        console.log("user address", user.address)
        const tx = await CoreDeployed.connect(user).createCon();
        const {events: createEvents, gasUsed: gas_used} = await tx.wait();
        console.log("create nft gas used", gas_used)
        const {
            args: {conOwner: newNftAddress, newConAddress: newNftCreator},
        } = createEvents.find(({event}) => event === 'NewCon');
        expect(newNftCreator).to.equal(user.address);
        const nftadd = await CoreDeployed.connect(user).conCreater(user.address);
        expect(nftadd).to.equal(newNftAddress);
        console.log("new nft contract address", newNftAddress);
        const NftContract = await ethers.getContractFactory('TestNft');
        const newNft = NftContract.attach(newNftAddress);
        const newNftOwner = await newNft.connect(user).owner();
        expect(newNftOwner).to.equal(user.address);
        const version = await newNft.connect(user).version();
        console.log("new nft contract version", version);

        // add version 2 implementation
        const implVer2 = await ethers.getContractFactory('TestNft2');
        const publicNft = await implVer2.deploy();
        await publicNft.deployed();
        const [signer] = await ethers.getSigners();
        await CoreDeployed.connect(signer).addConTemplate(publicNft.address, 2);
        const latestVersion = await CoreDeployed.connect(signer).LastestVersion();
        console.log("latest version implementation", latestVersion);

        //update nft contract to version2
        const tx2 = await CoreDeployed.connect(user).upgradeCon(newNftAddress, 2);
        const {events: upgrade_events, gasUsed: upgrade_gas_used} = await tx2.wait();
        const {
            args: {conAddress: ver2_ConAddress, admin: ver2_admin, newlevel: ver2_level},
        } = upgrade_events.find(({event}) => event === 'ConUpgrade');
        console.log(
            "upgrade contract event contract address %s admin %s version %s",
            ver2_ConAddress, ver2_admin, ver2_level
        );

        const publicimplVer2 = implVer2.attach(ver2_ConAddress);
        const version_ver2 = await publicimplVer2.connect(user).version();
        console.log("updated to version %s", version_ver2);
    })

})