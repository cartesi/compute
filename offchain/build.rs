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
}