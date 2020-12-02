# Descartes

## Getting Started

### Requirements

- Docker
- docker-compose
- node 12.x
- yarn
- jinja2

### Cloning

Make sure to include the submodules:
```
git clone --recurse-submodules ssh://github.com/cartesi/descartes.git
```
or using the http address:
```
git clone --recurse-submodules https://github.com/cartesi/descartes.git
```

### Running

To run execute:
```
% docker build . -t cartesi/descartes
% yarn
% rm deploy_done
% jinja2 -D num_players=2 docker-compose-template.yml | docker-compose -f - up --build
```

To shutdown:
```
% jinja2 -D num_players=2 docker-compose-template.yml | docker-compose -f - down -v
```

You can follow the output of a docker instance with:
```
% docker logs -f [name of the instance]
```
The instance could be retrieved by the command:
```
% docker ps --format {{.Names}}
```

This will run an environment connected to a private net (ganache or geth), with no descartes deployed.
To deploy a new descartes you need to run the `instantiate_descartes.ts` script. To do this you need to have `hardhat` installed, and run:

```
% npx hardhat run scripts/instantiate_descartes.ts --network localhost
```

To compile the smart contracts only:
```
% npx hardhat compile
```

## Contributing

Thank you for your interest in Cartesi! Head over to our [Contributing Guidelines](CONTRIBUTING.md) for instructions on how to sign our Contributors Agreement and get started with Cartesi!

Please note we have a [Code of Conduct](CODE_OF_CONDUCT.md), please follow it in all your interactions with the project.

## License

Note: This component currently has dependencies that are licensed under the GNU GPL, version 3, and so you should treat this component as a whole as being under the GPL version 3. But all Cartesi-written code in this component is licensed under the Apache License, version 2, or a compatible permissive license, and can be used independently under the Apache v2 license. After this component is rewritten, the entire component will be released under the Apache v2 license.
The arbitration d-lib repository and all contributions are licensed under
[GPL 3](https://www.gnu.org/licenses/gpl-3.0.en.html). Please review our [COPYING](COPYING) file.
