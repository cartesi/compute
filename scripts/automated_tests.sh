#!/bin/sh

echo "Starting BuidlerVM on the background"
nohup npx buidler node  > /dev/null 2>&1 &
buidlervmPID=$!
cd $(realpath -s $0 )/..

echo "Deploying all environmental artifacts $PWD"
sh ./deploy_development.sh

echo "Starting tests"

npx buidler test --network development

echo "Finishing automated test script and killing BuidlerVM"
kill $buidlervmPID