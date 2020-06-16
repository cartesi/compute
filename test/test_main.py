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


import os
import json
from enum import Enum
from web3.auto import w3

class DescartesState(Enum):
    WaitingProviders = 0
    ProviderMissedDeadline = 1
    ClaimerMissedDeadline = 2
    WaitingClaim = 3
    WaitingConfirmation = 4
    WaitingChallenge = 5
    ChallengerWon = 6
    ClaimerWon = 7
    ConsensusResult = 8

class Drive:
    def create_drive_tuple(position, drive_log2_size, value, provider, needs_provider, needs_logger):
        drive_hash = bytes(32)
        return (drive_hash, position, drive_log2_size, value, provider, needs_provider, needs_logger)

class BaseTest:

    def __init__(self):

        if (w3.isConnected()):
            print("Connected to node\n")
        else:
            print("Couldn't connect to node, exiting")
            sys.exit(1)

        self.endpoint = "http://127.0.0.1:8545"
        self.w3 = w3
        networkId = w3.net.version

        #loading deployed contract address and json file path

        with open('../build/contracts/Descartes.json') as json_file:
            descartes_data = json.load(json_file)
            self.descartes = w3.eth.contract(address=descartes_data['networks'][networkId]['address'], abi=descartes_data['abi'])

