To generate programs

`cd simulator/data`

Start docker container with machine emulator

`docker run -it --user root -v $PWD:/mnt cartesi/machine-emulator:0.15.2 /bin/bash`

In container execute

```
cd /mnt
./gen_machine_linux.sh
./gen_machine_simple.sh
```

"Fix" permissions for generated files:

`sudo chmod -R 777 programs`