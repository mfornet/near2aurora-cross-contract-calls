require("@nomicfoundation/hardhat-toolbox");
const { keccak256 } = require("ethers/lib/utils");
const { task } = require("hardhat/config");

const statusMessageAddress = '';

task("deploy", "Deploy contract").addParam("nearProxyAccountId").setAction(async (taskArgs) => {
  const address = '0x' + keccak256(Buffer.from(taskArgs.nearProxyAccountId)).slice(26);
  console.log({ nearProxyAddress: address });
  const statusMessageFactory = await ethers.getContractFactory("StatusMessage");
  const statusMessage = await statusMessageFactory.deploy(address);
  await statusMessage.deployed();
  console.log("Deployed contract to:", statusMessage.address);
});

task("evm-address", "EVM Address for specific NEAR account id").addParam("nearAccountId").setAction(async (taskArgs) => {
  const address = '0x' + keccak256(Buffer.from(taskArgs.nearAccountId)).slice(26);
  console.log({ nearAccountId: taskArgs.nearAccountId, nearProxyAddress: address });
});

task("list-messages", "List status messages").setAction(async () => {
  const statusMessageFactory = await ethers.getContractFactory("StatusMessage");
  const statusMessage = statusMessageFactory.attach(statusMessageAddress);

  const resultBN = await statusMessage.totalKeys();
  const total = resultBN.toNumber();

  console.log(`Found ${total} status`);

  for (let i = 0; i < total; i++) {
    const key = await statusMessage.getKey(i);
    const status = await statusMessage.getStatus(key);
    console.log(`${key}: ${status}`);
  }
});

task("add-status", "Add status message").addParam("statusMessage").setAction(async (taskArgs) => {
  const statusMessageFactory = await ethers.getContractFactory("StatusMessage");
  const statusMessage = statusMessageFactory.attach(statusMessageAddress);
  const result = await statusMessage.setAuroraAddressStatus(taskArgs.statusMessage);
  console.log({ result });
});

/** @type import('hardhat/config').HardhatUserConfig */
module.exports = {
  solidity: "0.8.13",
  defaultNetwork: 'auroraTestnet',
  networks: {
    aurora: {
      url: "https://mainnet.aurora.dev/",
      accounts: ['0x1311143f626d916df89a3f8829146fe1c606b6efced2901448eff84e4853ca56']
    },
    auroraTestnet: {
      url: "https://testnet.aurora.dev",
      accounts: ['0x1311143f626d916df89a3f8829146fe1c606b6efced2901448eff84e4853ca56']
    }
  }
};
