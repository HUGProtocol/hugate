require("@nomiclabs/hardhat-waffle");
require('@openzeppelin/hardhat-upgrades');
require('hardhat-contract-sizer');
require('hardhat-abi-exporter');
require('dotenv').config();
/**
 * @type import('hardhat/config').HardhatUserConfig
 */
const PRIVATE_KEY = process.env.PRIVATE_KEY;
module.exports = {
    solidity: {
        version: "0.8.7",
        settings: {
            optimizer: {
                enabled: true,
                runs: 200,
            }
        }
    },
    contractSizer: {
        alphaSort: true,
        disambiguatePaths: false,
        runOnCompile: false,
        strict: true,
    },
    abiExporter: {
        path: './data/abi',
        runOnCompile: false,
        clear: true,
        flat: false,
        spacing: 2,
        format: 'json',
        // pretty: true,
    },
    // defaultNetwork: "localhost",
    networks: {
        localhost: {
            url: "http://127.0.0.1:8545",
            accounts:
                [
                    "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80",
                    "0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d",
                    "0x5de4111afa1a4b94908f83103eb1f1706367c2e68ca870fc3fb9a804cdab365a"
                ]
        },
        goerli: {
            url: "https://goerli.infura.io/v3/ce421f619bc34c37a0fb86075d41226f",
            // url: "https://goerli.infura.io/v3/9aa3d95b3bc440fa88ea12eaa4456161",
            accounts:
                [
                    PRIVATE_KEY,
                ]
        },
        fuji: {
            url: "https://avalanche-fuji.infura.io/v3/ce421f619bc34c37a0fb86075d41226f",
            accounts:
                [
                    PRIVATE_KEY,
                ]
        },
    }
};
