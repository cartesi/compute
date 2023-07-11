// Copyright (C) 2020 Cartesi Pte. Ltd.

// This program is free software: you can redistribute it and/or modify it under
// the terms of the GNU General Public License as published by the Free Software
// Foundation, either version 3 of the License, or (at your option) any later
// version.

// This program is distributed in the hope that it will be useful, but WITHOUT
// ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
// FOR A PARTICULAR PURPOSE. See the GNU General Public License for more
// details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Note: This component currently has dependencies that are licensed under the
// GNU GPL, version 3, and so you should treat this component as a whole as
// being under the GPL version 3. But all Cartesi-written code in this component
// is licensed under the Apache License, version 2, or a compatible permissive
// license, and can be used independently under the Apache v2 license. After
// this component is rewritten, the entire component will be released under the
// Apache v2 license.

#![warn(unused_extern_crates)]
pub mod cartesi_compute;

extern crate error;
extern crate grpc;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;
extern crate compute;
extern crate configuration;
extern crate dispatcher;
extern crate ethabi;
extern crate ethereum_types;
extern crate hex;
extern crate transaction;

pub use cartesi_compute::{CartesiCompute, CartesiComputeCtx, CartesiComputeCtxParsed};

use ethereum_types::{Address, H256, U256};

#[derive(Debug)]
enum Role {
    Claimer,
    Challenger,
    Other,
}
pub fn build_machine_id(
    cartesi_compute_index: U256,
    player_address: &Address,
) -> String {
    return format!("{:x}:{}", player_address, cartesi_compute_index);
}