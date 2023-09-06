use std::path::Path;

use ethers_contract_abigen::Abigen;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    generate_grpc_stubs()?;
    generate_contract_bidings()?;
    Ok(())
}

fn generate_grpc_stubs() -> Result<(), Box<dyn std::error::Error>> {
    let proto_files = ["proto/compute.proto"];
    let proto_dirs = ["proto/"];
    let stub_dirs = Path::new("src/grpc");

    tonic_build::configure()
        .build_server(true)
        .out_dir(stub_dirs)
        .compile(&proto_files, &proto_dirs)
        .unwrap_or_else(|e| panic!("failed to build grpc stubs: {}", e));
    
    Ok(())
}

fn generate_contract_bidings() -> Result<(), Box<dyn std::error::Error>> {
    Abigen::new("NonRootTournament", "src/contract/tournament/artifact/NonRootTournament.json")?
        .generate()?
        .write_to_file("src/contract/tournament/non_root_tournament.rs")?;

    Abigen::new("RootTournament", "src/contract/tournament/artifact/RootTournament.json")?
        .generate()?
        .write_to_file("src/contract/tournament/root_tournament.rs")?;

    Abigen::new("NonLeafTournament", "src/contract/tournament/artifact/NonLeafTournament.json")?
        .generate()?
        .write_to_file("src/contract/tournament/non_leaf_tournament.rs")?;

    Abigen::new("LeafTournament", "src/contract/tournament/artifact/LeafTournament.json")?
        .generate()?
        .write_to_file("src/contract/tournament/leaf_tournament.rs")?;

    Abigen::new("Tournament", "src/contract/tournament/artifact/Tournament.json")?
        .generate()?
        .write_to_file("src/contract/tournament/tournament.rs")?;

    Abigen::new("TournamentFactory", "src/contract/factory/artifact/TournamentFactory.json")?
        .generate()?
        .write_to_file("src/contract/factory/tournament_factory.rs")?;

    Ok(())

    // In case state-fold is used to generate bindings.
    /*
    generate_contract_binding(
        "RootTournament",
        "src/contract/tournament/artifact/RootTournament.json",
        "src/contract/tournament/root_tournament.rs",
    )?;

    generate_contract_binding(
        "NonLeafTournament",
        "src/contract/tournament/artifact/NonLeafTournament.json",
        "src/contract/tournament/non_leaf_tournament.rs",
    )?;

    generate_contract_binding(
        "LeafTournament",
        "src/contract/tournament/artifact/LeafTournament.json",
        "src/contract/tournament/leaf_tournament.rs",
    )?;

    generate_contract_binding(
        "Tournament",
        "src/contract/tournament/artifact/Tournament.json",
        "src/contract/tournament/tournament.rs",
    )?;

    generate_contract_binding(
        "TournamentFactory",
        "src/contract/factory/artifact/TournamentFactory.json",
        "src/contract/factory/tournament_factory.rs"
    )?;
    */
}

// In case state-fold is used to generate bindings.
/*
fn generate_contract_binding(contract_name: &str, atrifact_path: &str, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let artifact = fs::canonicalize(atrifact_path)?;
    let abi = read_contract_abi_from_artifact(&artifact)?;

    let output = fs::canonicalize(output_path)?;
    let output = fs::File::create(Path::new(&output))?;

    contract::write(contract_name, abi.as_slice(), output)?;

    Ok(())
}

fn read_contract_abi_from_artifact(artifact_path: &Path) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let file = fs::File::open(artifact_path)?;
    let artifact: serde_json::Value = serde_json::from_reader(file)?;
    let artifact = artifact.as_object().unwrap();

    let abi = if let Some(abi) = artifact.get("abi") {
       abi
    } else {
        panic!("artifact file does not cotain contract abi");
    };

    Ok(abi.to_string().as_bytes().to_vec())
}
*/