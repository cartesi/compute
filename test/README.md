# Quick Start #

run yarn install
install python 3.7 and pip  
run pip install -r requirements.txt
run ganache-cli in a separate terminal
run prepare_python_tests.sh  
run run_python_tests.sh  

## Note ##

***DON'T run `prepare_python_tests_coverage.sh` and `run_python_tests_coverage.sh` manually, they are supposed to be called by `solidity-coverage` only.***

# Test Example #

```python
def test_getters():
    # use BaseTest to get all contracts' address and abi
    base_test = BaseTest()
    claimer = Web3.toChecksumAddress(base_test.w3.eth.accounts[0])
    challenger = Web3.toChecksumAddress(base_test.w3.eth.accounts[1])
    fake_li = Web3.toChecksumAddress("0000000000000000000000000000000000000001")
    fake_vg = Web3.toChecksumAddress("0000000000000000000000000000000000000002")
    fake_machine = Web3.toChecksumAddress("0000000000000000000000000000000000000003")
    template_hash = bytes("templateHash", 'utf-8')
    final_time = 3000
    round_duration = 300
    output_position = 50000
    drives = [Drive.create_drive_tuple(0, 5, bytes(32), claimer, DriveType.DirectWithValue.value)]

    # call instantiate function via transaction
    tx_hash = base_test.descartes.functions.instantiate(
            final_time,
            template_hash,
            output_position,
            round_duration,
            claimer,
            challenger,
            fake_li,
            fake_vg,
            fake_machine,
            drives).transact({'from': claimer})
    # wait for the transaction to be mined
    tx_receipt = base_test.w3.eth.waitForTransactionReceipt(tx_hash)
    # get the returned index via the event filter
    descartes_logs = base_test.descartes.events.DescartesCreated().processReceipt(tx_receipt)
    index = descartes_logs[0]['args']['_index']

    # (ret_hashes, ret_final_time, ret_time_of_last_move, ret_state, ret_drives)
    ret = base_test.descartes.functions.getState(index, claimer).call({'from': claimer})

    error_msg = "template_hash should match"
    assert ret[0][0][:len(template_hash)] == template_hash, error_msg

```

# Pytest Fixture #

```python

@pytest.fixture(autouse=True)
def run_between_tests():
    base_test = BaseTest()
    # Code that will run before your test, for example:
    headers = {'content-type': 'application/json'}
    payload = {"method": "evm_snapshot", "params": [], "jsonrpc": "2.0", "id": 0}
    response = requests.post(base_test.endpoint, data=json.dumps(payload), headers=headers).json()
    snapshot_id = response['result']
    # A test function will be run at this point
    yield
    # Code that will run after your test, for example:
    payload = {"method": "evm_revert", "params": [snapshot_id], "jsonrpc": "2.0", "id": 0}
    response = requests.post(base_test.endpoint, data=json.dumps(payload), headers=headers).json()

```
