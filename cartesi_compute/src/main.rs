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

// error-chain recursion
#![recursion_limit = "1024"]
#![warn(unused_extern_crates)]

extern crate cartesi_compute;
extern crate dispatcher;
extern crate env_logger;
extern crate utils;

use cartesi_compute::CartesiCompute;
use dispatcher::Dispatcher;
use utils::print_error;

fn main() {
    env_logger::init();

    let dispatcher = match Dispatcher::new() {
        Ok(d) => d,
        Err(ref e) => {
            print_error(e);
            return;
        }
    };

    dispatcher.run::<CartesiCompute>();
}
