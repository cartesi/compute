# RISC-V programs

## Generate programs

Run container with cartesi machine:

`cd data`
`docker run -it --user root -v $PWD:/mnt cartesi/machine-emulator:0.15.2 /bin/bash`

Inside container:

`cd /mnt`
`./gen_machine_linux.sh`
`./gen_machine_simple.sh`

"Fix permissions" of generated directories:

`sudo chmod -R 777 programs`