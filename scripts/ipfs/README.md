# IPFS Sample Computation

IPFS can be used to share larger data drives among users without relying on the ethereum blockchain resulting on costs savings. 

The run.sh script executes a number of steps in order to run a test using IPFS drives, first preparing the data then adequately instantiating the Cartesi Compute computation.

First we create a machine that can execute generic scripts.

```sh
. ./src/build-cartesi-machine.sh $CARTESI_COMPUTE_DIR/machines
```

Then we create the script, truncate it to the size we specified on the machine creation (4096) and calculate the root hash for this drive. 

```sh
. ./src/build-flash-drive.sh $CARTESI_COMPUTE_DIR
```

Finally, we add to the user's IPFS node the file we want to make available to all the users involved in this computation. Here we have preselected the first user of the `docker-compose-template.yml`. 


```sh
. ./src/ipfs-add-drive.sh
```


After those steps, everything is ready from the drive's perspective. Now we only need to start the computation by instantiating it on the deployed Cartesi Compute contract. You may have noticed that we sourced and exported the relevant variables to be now used on the next script: 

```sh
npx hardhat run instantiate.ts --no-compile --network localhost
```