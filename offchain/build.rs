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
    Abigen::new("InnerTournamentFactory", "src/contract/artifacts/InnerTournamentFactory.json")?
        .generate()?
        .write_to_file("src/contract/inner_tournament_factory.rs")?;

    Abigen::new("RootTournamentFactory", "src/contract/artifacts/RootTournamentFactory.json")?
        .generate()?
        .write_to_file("src/contract/root_tournament_factory.rs")?;

    Abigen::new("RootTournament", "src/contract/artifacts/RootTournament.json")?
        .generate()?
        .write_to_file("src/contract/root_tournament.rs")?;

    Ok(())
}