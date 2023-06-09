# Cartesi Compute

## Getting Started

### Requirements

- Docker
- docker-compose
- node 14.x
- yarn
- jinja2

### Cloning

Make sure to include the submodules:
```
git clone --recurse-submodules ssh://github.com/cartesi/compute.git
```
or using the http address:
```
git clone --recurse-submodules https://github.com/cartesi/compute.git
```

### Running

To run execute:
```
% docker build . -t cartesi/compute:local
% yarn
% jinja2 -D num_players=2 docker-compose-template.yml | docker-compose -f - up --build
```

To shutdown:
```
% jinja2 -D num_players=2 docker-compose-template.yml | docker-compose -f - down -v
```

To run using one of the [supported networks](https://docs.cartesi.io/compute/supported-networks/), you should:
- Define a `MNEMONIC` environment variable
- If using Infura, define a `PROJECT_ID` environment variable
- Specify the argument `"-D network=<name>"`, where `name` should be one of the following supported networks: `goerli`, `matic_testnet`, `bsc_testnet` or `avax_testnet`

For instance, for using the Goerli testnet using Infura, run:
```
% export MNEMONIC=<your_mnemonic>
% export PROJECT_ID=<your_infura_project_id>
% jinja2 -D num_players=2 -D network=goerli docker-compose-template.yml | docker-compose -f - up --build
```

You can follow the output of a docker instance with:
```
% docker logs -f [name of the instance]
```
The instance could be retrieved by the command:
```
% docker ps --format {{.Names}}
```

This will run an environment connected to a private net (ganache or geth), with cartesi compute already deployed.

There are a number of sample computations available within the `scripts` directory. To execute one of these computations on the environment, you need to first store the corresponding machine template in the `machines` directory (as configured in the docker-compose template). As such, for the `helloworld` application, execute the following commands:
```
% cd scripts
% ./download-images
% ./helloworld/build-cartesi-machine.sh ../images ../machines
```

Then, instantiate the computation using `hardhat`:
```
% npx hardhat run --network localhost --no-compile helloworld/instantiate.ts
```

After that, it will possible to query the computation result running the `getResult.ts` script:
```
% npx hardhat run --network localhost --no-compile getResult.ts
```
### IPFS Example
  
Among the sample computations, there is an example of usage of drive distribution through IPFS. Inside the folder there is a file `run.sh` and that is the only script you need to execute after starting the environment.

```
% ./scripts/ipfs/run.sh
```

## Mainnet usage (WARNING)

Given that the deployment of the Cartesi Compute contracts is deterministic, anyone can potentially deploy functional contracts with bytecode and correct addresses that matches the correct compilation of the Cartesi Compute contracts on Ethereum Mainnet or Arbitrum Mainnet.

Please note, however, these contracts and deployments haven't undergone a security audit. We view them as 'Mainnet Alpha', meaning they're in an early development stage and not recommended for your usage.

If you still choose to utilize these contracts despite the potential risks, we advise you to include a mechanism to suspend your usage of the Cartesi Compute contracts deployed on the mainnet within your smart contract. Furthermore, you should also have the ability to modify the Cartesi Compute contract addresses you're using.

Should you decide to proceed under these conditions, here are the steps to access the deterministically generated deployment files:


Mainnet:

- Get an infura.io account and create an Ethereum mainnet project and get project ID from it, henceforth <MY_PROJECT_ID>
- Clone the compute repository and check out the branch release/v1.3.x

```
% yarn
% MNEMONIC="test test test test test test test test test test test junk" PROJECT_ID=<MY_PROJECT_ID> npx hardhat deploy --network mainnet
```

Arbitrum One:

- Clone the compute repository and check out the branch release/v1.3.x

```
% yarn
% MNEMONIC="test test test test test test test test test test test junk" npx hardhat deploy --network arbitrum_one 
```

You'll now see a set of smart contract deployment files in deployments/mainnet and deployments/arbitrum_one which you can add inside your Compute SDK or link within your smart contract build process.

## Contributing

Thank you for your interest in Cartesi! Head over to our [Contributing Guidelines](CONTRIBUTING.md) for instructions on how to sign our Contributors Agreement and get started with Cartesi!

Please note we have a [Code of Conduct](CODE_OF_CONDUCT.md), please follow it in all your interactions with the project.


## License

Note: This component currently has dependencies that are licensed under the GNU GPL, version 3, and so you should treat this component as a whole as being under the GPL version 3. But all Cartesi-written code in this component is licensed under the Apache License, version 2, or a compatible permissive license, and can be used independently under the Apache v2 license. After this component is rewritten, the entire component will be released under the Apache v2 license.
The arbitration d-lib repository and all contributions are licensed under
[GPL 3](https://www.gnu.org/licenses/gpl-3.0.en.html). Please review our [COPYING](COPYING) file.
