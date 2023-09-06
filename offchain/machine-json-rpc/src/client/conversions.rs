// Copyright (C) 2021 Cartesi Pte. Ltd.
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use
// this file except in compliance with the License. You may obtain a copy of the
// License at http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software distributed
// under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
// CONDITIONS OF ANY KIND, either express or implied. See the License for the
// specific language governing permissions and limitations under the License.

use crate::interfaces;
use crate::client::*;

impl From<&MachineRuntimeConfig> for interfaces::MachineRuntimeConfig {
    fn from(rc: &MachineRuntimeConfig) -> Self {
        interfaces::MachineRuntimeConfig {
            concurrency: Some(interfaces::ConcurrencyConfig {
                update_merkle_tree: Some(rc.concurrency.update_merkle_tree),
            }),
            htif: Some(interfaces::HTIFRuntimeConfig { no_console_putchar: rc.htif.no_console_putchar }),
            skip_root_hash_check: Some(rc.skip_root_hash_check),
            skip_version_check: Some(rc.skip_version_check)
        }
    }
}

impl From<&MerkleTreeProof> for interfaces::Proof {
    fn from(proof: &MerkleTreeProof) -> Self {
        interfaces::Proof {
            target_address: proof.target_address,
            log_2_target_size: proof.log2_target_size as u64,
            log_2_root_size: proof.log2_root_size as u64,
            target_hash: proof.target_hash.clone(),
            root_hash: proof.root_hash.clone(),
            sibling_hashes: proof.sibling_hashes.clone(),
        }
    }
}

impl From<&Access> for interfaces::Access {
    fn from(access: &Access) -> Self {
        let mut read = base64::encode(access.read_data.clone());
        let mut written = base64::encode(access.written_data.clone());

        if read.ends_with("=") {
            read.push('\n');
        }
        if written.ends_with("=") {
            written.push('\n');
        }

        interfaces::Access {
            r#type: match access.r#type {
                AccessType::Read => serde_json::json!("read"),
                AccessType::Write => serde_json::json!("write"),
            },
            read: read,
            written: Some(written),
            proof: Some(interfaces::Proof::from(&access.proof)),
            address: access.address,
            log_2_size: access.log2_size as u64,
        }
    }
}

impl From<&BracketNote> for interfaces::Bracket {
    fn from(bracket_note: &BracketNote) -> Self {
        interfaces::Bracket {
            r#type: match bracket_note.r#type {
                BracketType::Begin => serde_json::json!("begin"),
                BracketType::End => serde_json::json!("end"),
            },
            r#where: bracket_note.r#where,
            text: bracket_note.text.clone(),
        }
    }
}

impl From<&AccessLogType> for interfaces::AccessLogType {
    fn from(log_type: &AccessLogType) -> Self {
        interfaces::AccessLogType {
            has_proofs: log_type.proofs,
            has_annotations: log_type.annotations,
        }
    }
}

impl From<&AccessLog> for interfaces::AccessLog {
    fn from(log: &AccessLog) -> Self {
        let log_type = interfaces::AccessLogType {
            has_proofs: log.log_type.proofs,
            has_annotations: log.log_type.annotations,
        };
        interfaces::AccessLog {
            log_type,
            accesses: log.accesses.iter().map(|e| interfaces::Access::from(e)).collect(),
            brackets: Some(log.brackets.iter().map(|e| interfaces::Bracket::from(e)).collect()),
            notes: Some(log.notes.clone()),
        }
    }
}

pub fn convert_x_csr_field(config: &interfaces::ProcessorConfig) -> [u64; 32usize] {
    let mut result: [u64; 32usize] = [0; 32usize];
    result[0] = convert_csr_field(Some(config.x.clone().unwrap()[0]));
    result[1] = convert_csr_field(Some(config.x.clone().unwrap()[1]));
    result[2] = convert_csr_field(Some(config.x.clone().unwrap()[2]));
    result[3] = convert_csr_field(Some(config.x.clone().unwrap()[3]));
    result[4] = convert_csr_field(Some(config.x.clone().unwrap()[4]));
    result[5] = convert_csr_field(Some(config.x.clone().unwrap()[5]));
    result[6] = convert_csr_field(Some(config.x.clone().unwrap()[6]));
    result[7] = convert_csr_field(Some(config.x.clone().unwrap()[7]));
    result[8] = convert_csr_field(Some(config.x.clone().unwrap()[8]));
    result[9] = convert_csr_field(Some(config.x.clone().unwrap()[9]));
    result[10] = convert_csr_field(Some(config.x.clone().unwrap()[10]));
    result[11] = convert_csr_field(Some(config.x.clone().unwrap()[11]));
    result[12] = convert_csr_field(Some(config.x.clone().unwrap()[12]));
    result[13] = convert_csr_field(Some(config.x.clone().unwrap()[13]));
    result[14] = convert_csr_field(Some(config.x.clone().unwrap()[14]));
    result[15] = convert_csr_field(Some(config.x.clone().unwrap()[15]));
    result[16] = convert_csr_field(Some(config.x.clone().unwrap()[16]));
    result[17] = convert_csr_field(Some(config.x.clone().unwrap()[17]));
    result[18] = convert_csr_field(Some(config.x.clone().unwrap()[18]));
    result[19] = convert_csr_field(Some(config.x.clone().unwrap()[19]));
    result[20] = convert_csr_field(Some(config.x.clone().unwrap()[20]));
    result[21] = convert_csr_field(Some(config.x.clone().unwrap()[21]));
    result[22] = convert_csr_field(Some(config.x.clone().unwrap()[22]));
    result[23] = convert_csr_field(Some(config.x.clone().unwrap()[23]));
    result[24] = convert_csr_field(Some(config.x.clone().unwrap()[24]));
    result[25] = convert_csr_field(Some(config.x.clone().unwrap()[25]));
    result[26] = convert_csr_field(Some(config.x.clone().unwrap()[26]));
    result[27] = convert_csr_field(Some(config.x.clone().unwrap()[27]));
    result[28] = convert_csr_field(Some(config.x.clone().unwrap()[28]));
    result[29] = convert_csr_field(Some(config.x.clone().unwrap()[29]));
    result[30] = convert_csr_field(Some(config.x.clone().unwrap()[30]));
    result
}

pub fn convert_f_csr_field(config: &interfaces::ProcessorConfig) -> [u64; 32usize] {
    let mut result: [u64; 32usize] = [0; 32usize];
    result[0] = convert_csr_field(Some(config.f.clone().unwrap()[0]));
    result[1] = convert_csr_field(Some(config.f.clone().unwrap()[1]));
    result[2] = convert_csr_field(Some(config.f.clone().unwrap()[2]));
    result[3] = convert_csr_field(Some(config.f.clone().unwrap()[3]));
    result[4] = convert_csr_field(Some(config.f.clone().unwrap()[4]));
    result[5] = convert_csr_field(Some(config.f.clone().unwrap()[5]));
    result[6] = convert_csr_field(Some(config.f.clone().unwrap()[6]));
    result[7] = convert_csr_field(Some(config.f.clone().unwrap()[7]));
    result[8] = convert_csr_field(Some(config.f.clone().unwrap()[8]));
    result[9] = convert_csr_field(Some(config.f.clone().unwrap()[9]));
    result[10] = convert_csr_field(Some(config.f.clone().unwrap()[10]));
    result[11] = convert_csr_field(Some(config.f.clone().unwrap()[11]));
    result[12] = convert_csr_field(Some(config.f.clone().unwrap()[12]));
    result[13] = convert_csr_field(Some(config.f.clone().unwrap()[13]));
    result[14] = convert_csr_field(Some(config.f.clone().unwrap()[14]));
    result[15] = convert_csr_field(Some(config.f.clone().unwrap()[15]));
    result[16] = convert_csr_field(Some(config.f.clone().unwrap()[16]));
    result[17] = convert_csr_field(Some(config.f.clone().unwrap()[17]));
    result[18] = convert_csr_field(Some(config.f.clone().unwrap()[18]));
    result[19] = convert_csr_field(Some(config.f.clone().unwrap()[19]));
    result[20] = convert_csr_field(Some(config.f.clone().unwrap()[20]));
    result[21] = convert_csr_field(Some(config.f.clone().unwrap()[21]));
    result[22] = convert_csr_field(Some(config.f.clone().unwrap()[22]));
    result[23] = convert_csr_field(Some(config.f.clone().unwrap()[23]));
    result[24] = convert_csr_field(Some(config.f.clone().unwrap()[24]));
    result[25] = convert_csr_field(Some(config.f.clone().unwrap()[25]));
    result[26] = convert_csr_field(Some(config.f.clone().unwrap()[26]));
    result[27] = convert_csr_field(Some(config.f.clone().unwrap()[27]));
    result[28] = convert_csr_field(Some(config.f.clone().unwrap()[28]));
    result[29] = convert_csr_field(Some(config.f.clone().unwrap()[29]));
    result[30] = convert_csr_field(Some(config.f.clone().unwrap()[30]));
    result
}

pub fn convert_csr_field(field: ::core::option::Option<u64>) -> u64
where
{
    match field {
        Some(x) => u64::from(x),
        None => 0,
    }
}

impl From<&ProcessorConfig> for interfaces::ProcessorConfig {
    fn from(config: &ProcessorConfig) -> Self {
        interfaces::ProcessorConfig {
            x: Some(config.x.to_vec()),
            f: Some(config.f.to_vec()),
            pc: Some(config.pc),
            mvendorid: Some(config.mvendorid),
            marchid: Some(config.marchid),
            mimpid: Some(config.mimpid),
            mcycle: Some(config.mcycle),
            icycleinstret: Some(config.icycleinstret),
            mstatus: Some(config.mstatus),
            mtvec: Some(config.mtvec),
            mscratch: Some(config.mscratch),
            mepc: Some(config.mepc),
            mcause: Some(config.mcause),
            mtval: Some(config.mtval),
            misa: Some(config.misa),
            mie: Some(config.mie),
            mip: Some(config.mip),
            medeleg: Some(config.medeleg),
            mideleg: Some(config.mideleg),
            mcounteren: Some(config.mcounteren),
            stvec: Some(config.stvec),
            sscratch: Some(config.sscratch),
            sepc: Some(config.sepc),
            scause: Some(config.scause),
            stval: Some(config.stval),
            satp: Some(config.satp),
            scounteren: Some(config.scounteren),
            ilrsc: Some(config.ilrsc),
            iflags: Some(config.iflags),
            menvcfg: Some(config.menvcfg),
            senvcfg: Some(config.senvcfg),
            fcsr: Some(config.fcsr),
        }
    }
}

impl From<&RamConfig> for interfaces::RAMConfig {
    fn from(config: &RamConfig) -> Self {
        interfaces::RAMConfig {
            length: config.length,
            image_filename: Some(config.image_filename.clone()),
        }
    }
}

impl From<&RomConfig> for interfaces::ROMConfig {
    fn from(config: &RomConfig) -> Self {
        interfaces::ROMConfig {
            bootargs: Some(config.bootargs.clone()),
            image_filename: Some(config.image_filename.clone()),
        }
    }
}

impl From<&TlbConfig> for interfaces::TLBConfig {
    fn from(config: &TlbConfig) -> Self {
        interfaces::TLBConfig {
            image_filename: Some(config.image_filename.clone()),
        }
    }
}

impl From<&UarchConfig> for interfaces::UarchConfig {
    fn from(config: &UarchConfig) -> Self {
        interfaces::UarchConfig {
            processor: config.processor.clone(),
            ram: config.ram.clone(),
        }
    }
}

impl From<&MemoryRangeConfig> for interfaces::MemoryRangeConfig {
    fn from(config: &MemoryRangeConfig) -> Self {
        interfaces::MemoryRangeConfig {
            start: Some(config.start),
            length: Some(config.length),
            image_filename: Some(config.image_filename.clone()),
            shared: Some(config.shared),
        }
    }
}

impl From<&RollupConfig> for interfaces::RollupConfig {
    fn from(config: &RollupConfig) -> Self {
        interfaces::RollupConfig {
            input_metadata: match &config.input_metadata {
                Some(config) => Some(interfaces::MemoryRangeConfig::from(config)),
                None => None,
            },
            tx_buffer: match &config.tx_buffer {
                Some(config) => Some(interfaces::MemoryRangeConfig::from(config)),
                None => None,
            },
            voucher_hashes: match &config.voucher_hashes {
                Some(config) => Some(interfaces::MemoryRangeConfig::from(config)),
                None => None,
            },
            rx_buffer: match &config.rx_buffer {
                Some(config) => Some(interfaces::MemoryRangeConfig::from(config)),
                None => None,
            },
            notice_hashes: match &config.notice_hashes {
                Some(config) => Some(interfaces::MemoryRangeConfig::from(config)),
                None => None,
            },
        }
    }
}

impl From<&MachineConfig> for interfaces::MachineConfig {
    fn from(config: &MachineConfig) -> Self {
        interfaces::MachineConfig {
            processor: Some(interfaces::ProcessorConfig::from(&config.processor)),
            ram: Some(interfaces::RAMConfig::from(&config.ram)),
            rom: Some(interfaces::ROMConfig::from(&config.rom)),
            tlb: Some(interfaces::TLBConfig::from(&config.tlb)),
            uarch: Some(interfaces::UarchConfig::from(&config.uarch)),
            flash_drive: Some(
                config
                    .flash_drives
                    .iter()
                    .map(|e| interfaces::MemoryRangeConfig::from(e))
                    .collect(),
            ),
            clint: Some(interfaces::CLINTConfig::from(config.clint.clone())),
            htif: Some(interfaces::HTIFConfig::from(config.htif.clone())),
            rollup: Some(interfaces::RollupConfig::from(&config.rollup)),
        }
    }
}
