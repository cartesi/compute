use std::{
    path::Path,
    fs::File,
    error::Error,
};

use ethers::{
    core::abi::Abi,
    types::Bytes,
};

use serde_json;
use hex;

pub fn parse_artifact(path: &Path) -> Result<(Abi, Bytes), Box<dyn Error>> {
    let file = File::open(path)?;
    let artifact: serde_json::Value = serde_json::from_reader(file)?;
    let artifact = artifact.as_object().unwrap();
    
    let abi = artifact.get("abi").expect("no abi in artifact file");
    let abi = Abi::load(abi.to_string().as_bytes())?;
        
    let bytecode = artifact
        .get("bytecode").expect("no bytecode in artifact file")
        .get("object").expect("not bytecode.object in artifact file");
    let bytecode = Bytes::from(eth_hex_to_bytes(bytecode.as_str().unwrap())?);

    Ok((abi, bytecode))
}

pub fn eth_hex_to_bytes(eth_hex: &str) -> Result<Vec<u8>, Box< dyn Error>> {
    let hex = &eth_hex[2..eth_hex.len()];
    let bytes = hex::decode(hex)?;
    Ok(bytes)
}