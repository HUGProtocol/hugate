const { expect, use } = require('chai')
const { assert } = require('console')
const { Contract } = require('ethers')
const { ethers, upgrades } = require('hardhat')
const { reverts } = require('./helpers/error')
const { anyValue } = require("@nomicfoundation/hardhat-chai-matchers/withArgs");
const { address } = require("hardhat/internal/core/config/config-validation");

describe('ERC20 Basics', () => {
    let ERC20Contract
    let CuckooContract
    const decimal = ethers.utils.parseUnits('1.0', 18);
    before(async () => {
        const [signer] = await ethers.getSigners();
        const ERC20Sol = await ethers.getContractFactory('ERC20Tmpl');
        const initialSupply = decimal.mul(10000);
        ERC20Contract = await ERC20Sol.deploy(initialSupply.toHexString());
        await ERC20Contract.deployed();
        const value = await ERC20Contract.connect(signer).balanceOf(signer.address);
        expect(value).to.equal(initialSupply);

        const cuckooSol = await ethers.getContractFactory('Cuckoo');
        CuckooContract = await upgrades.deployProxy(cuckooSol);
        await CuckooContract.deployed();

        //ERC20 transfer
        const [owner, Alice, Bob] = await ethers.getSigners();
        //transfer 100 TMP to Alice
        const amount = decimal.mul(100);
        const tx = await ERC20Contract.connect(owner).transfer(Alice.address, amount.toHexString());
        await tx.wait();
        const BobBalance = await ERC20Contract.connect(Alice).balanceOf(Alice.address);
        const OwnerBalance = await ERC20Contract.connect(owner).balanceOf(owner.address);
        const ownerExpect = decimal.mul(9900);
        const bobExpect = decimal.mul(100);
        expect(ownerExpect).to.equal(OwnerBalance);
        expect(bobExpect).to.equal(BobBalance);

        //transfer 100 TMP to Bob
        const txBob = await ERC20Contract.connect(owner).transfer(Bob.address, amount.toHexString());
        await txBob.wait();
    })

    it('Cuckoo version', async () => {
        const [signer] = await ethers.getSigners();
        const version = await CuckooContract.connect(signer).version();
        const versionExpect = ethers.BigNumber.from(1);
        expect(versionExpect).to.equal(version);
        //update version
        const tx = await CuckooContract.connect(signer).updateVersion();
        await tx.wait();
        const newVersion = await CuckooContract.connect(signer).version();
        const newVersionExpect = ethers.BigNumber.from(2);
        expect(newVersionExpect).to.equal(newVersion);
    })

    it('should not mint not exist channel pass', async () => {
        const [owner, Alice, Bob] = await ethers.getSigners();
        const approveAmount = decimal.mul(100);
        const tx1 = await ERC20Contract.connect(Bob).approve(CuckooContract.address, approveAmount.toHexString());
        await tx1.wait();
        const tx2 = CuckooContract.connect(Bob).subscribeChannel(ethers.BigNumber.from(0).toHexString(), Bob.address, ethers.BigNumber.from(1).toHexString());
        await expect(tx2).to.be.revertedWith('channel not exist')
    })

    describe('Channel Management', () => {
        let Alice;
        let Bob
        let owner;
        let ChannelProxyAddress;
        const tokenUri = "ipfs://QmeSjSinHpPnmXmspMjwiXyN6zS4E9zccariGR3jxcaWtq/6476"
        const price = decimal.mul(10);
        const pass_amount = 10;
        before(async () => {
            [owner, Alice, Bob] = await ethers.getSigners();
        })

        it('Should publisher new channel', async () => {
            const txNewChan2 = await CuckooContract.connect(Alice).newChannel(tokenUri, price.toHexString(), ERC20Contract.address, ethers.BigNumber.from(pass_amount).toHexString());
            await txNewChan2.wait();
            // await expect(txNewChan2).not.to.be.reverted;
            const tokenId = ethers.BigNumber.from(0);
            const channelInfo = await CuckooContract.connect(Alice).ChannelInfo(tokenId);
            expect(channelInfo.owner).to.equal(Alice.address);
            expect(channelInfo.price).to.equal(price);
            expect(channelInfo.passCount).to.equal(ethers.BigNumber.from(1));
            expect(channelInfo.token).to.equal(ERC20Contract.address);
        })

        it('should publisher edit channel info', async function () {
            await expect(CuckooContract.connect(Bob).updateChannelBasic(ethers.BigNumber.from(0).toHexString(), price, ERC20Contract.address)).to.be.reverted;
            const basic = {
                owner: Alice.address,
                price: price,
                passCount: ethers.BigNumber.from(1),
                token: ERC20Contract.address
            };
            await expect(CuckooContract.connect(Alice).updateChannelBasic(ethers.BigNumber.from(0).toHexString(), price, ERC20Contract.address))
                .to.emit(CuckooContract, "UpdateChannelEvent").withArgs(basic.owner, price, basic.passCount, basic.token);
        });

        it('mint pass erc20', async () => {
            const approveAmount = decimal.mul(100);
            const tx1 = await ERC20Contract.connect(Bob).approve(CuckooContract.address, approveAmount.toHexString());
            await tx1.wait();

            const tx2 = await CuckooContract.connect(Bob).subscribeChannel(ethers.BigNumber.from(0).toHexString(), Bob.address, ethers.BigNumber.from(1).toHexString());
            await tx2.wait();

            //should have token 1
            const tokenId = ethers.BigNumber.from(0);
            const tokenAmount = await CuckooContract.connect(Bob).balanceOf(Bob.address, tokenId.toHexString());
            expect(tokenAmount).to.equal(ethers.BigNumber.from(1));

            //should channelInfo pass count added to 2
            const channelInfo = await CuckooContract.connect(Alice).ChannelInfo(tokenId);
            expect(channelInfo.passCount).to.equal(ethers.BigNumber.from(1 + 1));

            //should spend 10 TMP fot Alice channel pass
            const tmpBalance = await ERC20Contract.connect(Bob).balanceOf(Bob.address);
            expect(tmpBalance).to.be.equal(decimal.mul(90));
        });

        it('Should publisher new channel price eth', async () => {
            const txNewChan2 = await CuckooContract.connect(Alice).newChannel(tokenUri, price.toHexString(), "0x0000000000000000000000000000000000000000", ethers.BigNumber.from(pass_amount).toHexString());
            await txNewChan2.wait();
            const tokenId = ethers.BigNumber.from(1);
            const channelInfo = await CuckooContract.connect(Alice).ChannelInfo(tokenId);
            expect(channelInfo.owner).to.equal(Alice.address);
            expect(channelInfo.price).to.equal(price);
            expect(channelInfo.passCount).to.equal(ethers.BigNumber.from(1));
            expect(channelInfo.token).to.equal("0x0000000000000000000000000000000000000000");
        })


        it('mint pass eth', async () => {
            const AliceBalance = await Alice.getBalance();
            const bobBalance = await Bob.getBalance();
            const tokenId = ethers.BigNumber.from(1);
            const tx2 = await CuckooContract.connect(Bob).subscribeChannel(
                tokenId.toHexString(),
                Bob.address,
                ethers.BigNumber.from(1).toHexString(),
                { value: price });
            const receipt = await tx2.wait();
            const gasCostForTxn = receipt.gasUsed.mul(receipt.effectiveGasPrice)
            //should have token 1
            const tokenAmount = await CuckooContract.connect(Bob).balanceOf(Bob.address, tokenId.toHexString());
            expect(tokenAmount).to.equal(ethers.BigNumber.from(1));

            //should channelInfo pass count added to 2
            const channelInfo = await CuckooContract.connect(Alice).ChannelInfo(tokenId);
            expect(channelInfo.passCount).to.equal(ethers.BigNumber.from(1 + 1));

            //should spend 10 TMP fot Alice channel pass
            const bobBalanceNew = await Bob.getBalance();
            const AliceBalanceNew = await Alice.getBalance();
            expect(AliceBalanceNew).to.equal(price.add(AliceBalance));
            expect(bobBalance).to.equal(price.add(bobBalanceNew).add(gasCostForTxn));
        });

        it('should batch send pass', async () => {
            const tokenId = ethers.BigNumber.from(0);
            const info = await CuckooContract.connect(Alice).checkPass(Alice.address);
            console.log("alice before", info)
            const tx = await CuckooContract.connect(Alice).batchSend(tokenId, [owner.address, Bob.address]);
            await tx.wait();
            const bobAmount = await CuckooContract.connect(Bob).balanceOf(Bob.address, tokenId.toHexString());
            expect(bobAmount).to.equal(ethers.BigNumber.from(2));
            const ownerAmount = await CuckooContract.connect(owner).balanceOf(owner.address, tokenId.toHexString());
            expect(ownerAmount).to.equal(ethers.BigNumber.from(1));
            const info1 = await CuckooContract.connect(Alice).checkPass(Alice.address);
            console.log("alice after", info1)
            const info2 = await CuckooContract.connect(Bob).checkPass(Bob.address);
            console.log("bob info", info2)
        });

        it('should get pass info', async () => {
            const info = await CuckooContract.connect(Alice).checkPass(Alice.address);
            console.log(info);
        });
    })
})