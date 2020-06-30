/**
 * Truffle script to create new tornaments.
 * 
 * Usage:
 * truffle exec <script> --network <network>
 */

const contract = require("@truffle/contract");
const program = require("commander");

const Logger = contract(require("@cartesi/logger/build/contracts/Logger.json"));
const VGInstantiator = contract(require("@cartesi/arbitration/build/contracts/VGInstantiator.json"));
const Step = contract(require("@cartesi/machine-solidity-step/build/contracts/Step.json"));
const Descartes = contract(require("./build/contracts/Descartes.json"));

program
    .option('-n, --network <network>', 'Specify the network to use, using artifacts specific to that network. Network name must exist in the configuration')
    .option('-c, --compile', 'Compile contracts before executing the script')
    .option('--claimer <account>', 'Claimer address')
    .option('--challenger <account>', 'Challenger address')
    .option('--round-duration <duration>', 'Duration in seconds of round phase', 45);

module.exports = async (callback) => {

    program.parse(process.argv);
    console.log(`Creating descartes instance`);

    try {
        const networkId = await web3.eth.net.getId();

        let claimer = undefined;
        let challenger = undefined;
        if (program.claimer) {
            claimer = program.claimer;
        } else {
            const accounts = await web3.eth.personal.getAccounts();
            claimer = accounts[0];
        }
        if (program.challenger) {
            challenger = program.challenger;
        } else {
            const accounts = await web3.eth.personal.getAccounts();
            challenger = accounts[1];
        }
        fromAddress = claimer

        const contracts = [
            Logger,
            VGInstantiator,
            Step,
            Descartes
        ];
        contracts.forEach(contract => {
            contract.setNetwork(networkId);
            contract.setProvider(web3.currentProvider);
            console.log(`${contract.contract_name} => ${contract.address}`);
        });

        const roundDuration = program.roundDuration;
        const outputPosition = 0x2700;
        const finalTime = 1e13;

        const driveHash = "0x375fb938dcff562818779bc0dc4689a713a61d89659c8a9274a53551c7bc464c";
        const directDriveValue = "0x375fb938dcff562818779bc0dc4689a713a61d89659c8a9274a53551c7bc464c";
        const drive = [
            driveHash,
            0,
            5,
            directDriveValue,
            "",
            claimer,
            false,
            false
        ];

        const drives = [drive];

        // TODO: update this with actual machine hash
        const templateHash = "0x375fb938dcff562818779bc0dc4689a713a61d89659c8a9274a53551c7bc464c";

        const descartes = await Descartes.deployed();
        const transaction = await descartes.instantiate(
            finalTime,
            templateHash,
            outputPosition,
            roundDuration,
            claimer,
            challenger,
            drives,
            { from: fromAddress }
        );
        console.log(`Descartes instance created: ${transaction.tx}`);
        callback();

    } catch (e) {
        console.error(e);
        callback(e);
    }
};
