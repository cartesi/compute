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

### Building

To build the Docker image, use the following command

```shell
docker build . -t cartesi/descartes
```

#### Building for ARM

To build the Docker image for ARM, use the following argument to specify an ARM build

```shell
docker build --build-arg ARCH=ARM . -t cartesi/descartes
```



### Running

To run execute:
```
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
To deploy a new descartes you need to run the `instantiate_descartes.js` truffle script. To do this you need to have `truffle` installed, and run:

```
% truffle exec instantiate_descartes.js --network development
```
This will print something like this:

```
Using network 'development'.

Creating descartes instance
Logger => 0x1c18eADf263d0d76A8944D225F702C662596B595
VGInstantiator => 0x90015115745a6B12429608543cf1E1E69AF56C63
Step => 0x84e9c63e38D7b2950f8Fc991789b00210d2538F1
Descartes => 0x7E8066A9dc6ed98CC2e3D508B1b807E57f10d355
Descartes instance created: 0x88a7e7e5d1ad19a1b441adda72958fc0ad6761be3d1e3346bc17e6d0afb5ac3c
```

## Contributing

Thank you for your interest in Cartesi! Head over to our [Contributing Guidelines](CONTRIBUTING.md) for instructions on how to sign our Contributors Agreement and get started with Cartesi!

Please note we have a [Code of Conduct](CODE_OF_CONDUCT.md), please follow it in all your interactions with the project.

## License

Note: This component currently has dependencies that are licensed under the GNU GPL, version 3, and so you should treat this component as a whole as being under the GPL version 3. But all Cartesi-written code in this component is licensed under the Apache License, version 2, or a compatible permissive license, and can be used independently under the Apache v2 license. After this component is rewritten, the entire component will be released under the Apache v2 license.
The arbitration d-lib repository and all contributions are licensed under
[GPL 3](https://www.gnu.org/licenses/gpl-3.0.en.html). Please review our [COPYING](COPYING) file.
