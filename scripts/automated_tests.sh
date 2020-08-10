#!/bin/sh

echo "Starting BuidlerVM on the background"
nohup npx buidler node --no-deploy > /dev/null 2>&1 &
buidlervmPID=$!
echo "Waiting for the Node to start"
until $(curl --output /dev/null --silent --head --fail http://127.0.0.1:8545); do
    printf '.'
    sleep 5
done

cd $(realpath -s $0 )/..  

echo "Deploying all environmental artifacts $PWD"
sh ./deploy_development.sh

echo "Starting tests"

cd ..; npx buidler typechain

npx buidler test --network development

echo "Finishing automated test script and killing BuidlerVM"
kill $buidlervmPID