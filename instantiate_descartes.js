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
        const outputPosition = "0xa000000000000000";
        const finalTime = 1e13;

        // bytes of print(math.sin(1))
        const directDriveValue = "0x7072696e74286d6174682e73696e28312929";
        // hash of print(math.sin(1))
        const loggerRootHash = "0xa87f79b5149218496af4d722798f46cdb1a15b12928ad05618892e5b3f999062"
        const drive = [
            "0x9000000000000000",
            5,
            directDriveValue,
            loggerRootHash,
            claimer,
            false,
            false
        ];

        const drives = [drive];

        const templateHash = "0x1b185aeffc778b7d2fdce1835be50261397975a7e17745ab8b97ca75d42becc6";

        const descartes = await Descartes.deployed();
        const transaction = await descartes.instantiate(
            finalTime,
            templateHash,
            outputPosition,
            5,
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
