module.exports = function(callback) {
    web3.eth.net.getId((error, network_id) => {
        if (error) {
            callback(error);
        } else {
            if (network_id === 15) {
                web3.eth.personal.getAccounts((error, accounts) => {
                    if (error) {
                        callback(error);
                    } else {
                        const account = accounts[0];
                        console.log(`Unlocking account ${account} for local geth use`);
                        web3.eth.personal.unlockAccount(account, "private_network", 15000, callback);
                    }
                });
            } else {
                console.log(`No need to unlock account, network_id = ${network_id}`)
                callback();
            }
        }
    });
}
