# Copyright (C) 2019 Cartesi Pte. Ltd.

# This program is free software: you can redistribute it and/or modify it under
# the terms of the GNU General Public License as published by the Free Software
# Foundation, either version 3 of the License, or (at your option) any later
# version.

# This program is distributed in the hope that it will be useful, but WITHOUT ANY
# WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A
# PARTICULAR PURPOSE. See the GNU General Public License for more details.

# You should have received a copy of the GNU General Public License
# along with this program.  If not, see <https://www.gnu.org/licenses/>.

# Note: This component currently has dependencies that are licensed under the GNU
# GPL, version 3, and so you should treat this component as a whole as being under
# the GPL version 3. But all Cartesi-written code in this component is licensed
# under the Apache License, version 2, or a compatible permissive license, and can
# be used independently under the Apache v2 license. After this component is
# rewritten, the entire component will be released under the Apache v2 license.


import pytest
import ast
import requests
import json
from web3 import Web3
from test_main import BaseTest, DescartesState, Drive

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

def test_throws():
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
    drives = [Drive.create_drive_tuple(0, 5, bytes(32), claimer, False, False)]

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
    tx_receipt = base_test.w3.eth.waitForTransactionReceipt(tx_hash)
    descartes_logs = base_test.descartes.events.DescartesCreated().processReceipt(tx_receipt)
    index = descartes_logs[0]['args']['_index']

    error_msg = "abortByDeadline Transaction should fail, deadline is not over yet"
    try:
        base_test.descartes.functions.abortByDeadline(index).transact({'from': claimer})
    except ValueError as e:
        error_dict = ast.literal_eval(str(e))
        assert error_dict['message'][50:] == "Deadline is not over for this specific state", error_msg
    else:
        raise Exception(error_msg)

    error_msg = "challenge Transaction should fail, caller should be challenger"
    try:
        base_test.descartes.functions.challenge(index).transact({'from': claimer})
    except ValueError as e:
        error_dict = ast.literal_eval(str(e))
        assert error_dict['message'][50:] == "Cannot be called by user", error_msg
    else:
        raise Exception(error_msg)

    error_msg = "challenge Transaction should fail, currentState is not WaitingConfirmation"
    try:
        base_test.descartes.functions.challenge(index).transact({'from': challenger})
    except ValueError as e:
        error_dict = ast.literal_eval(str(e))
        assert error_dict['message'][50:] == "State should be WaitingConfirmation", error_msg
    else:
        raise Exception(error_msg)

    error_msg = "confirm Transaction should fail, caller should be challenger"
    try:
        base_test.descartes.functions.confirm(index).transact({'from': claimer})
    except ValueError as e:
        error_dict = ast.literal_eval(str(e))
        assert error_dict['message'][50:] == "Cannot be called by user", error_msg
    else:
        raise Exception(error_msg)

    error_msg = "confirm Transaction should fail, currentState is not WaitingConfirmation"
    try:
        base_test.descartes.functions.confirm(index).transact({'from': challenger})
    except ValueError as e:
        error_dict = ast.literal_eval(str(e))
        assert error_dict['message'][50:] == "State should be WaitingConfirmation", error_msg
    else:
        raise Exception(error_msg)

    timeout = 341
    
    headers = {'content-type': 'application/json'}
    payload = {"method": "evm_increaseTime", "params": [timeout], "jsonrpc": "2.0", "id": 0}
    response = requests.post(base_test.endpoint, data=json.dumps(payload), headers=headers).json()

    base_test.descartes.functions.abortByDeadline(index).transact({'from': claimer})
    
    error_msg = "abortByDeadline Transaction should fail, currentState is not allowed"
    try:
        base_test.descartes.functions.abortByDeadline(index).transact({'from': claimer})
    except ValueError as e:
        error_dict = ast.literal_eval(str(e))
        assert error_dict['message'][50:] == "Cannot abort current state", error_msg
    else:
        raise Exception(error_msg)
