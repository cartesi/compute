use std::path::Path;

use ethers_contract_abigen::Abigen;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    generate_contract_bidings()?;
    Ok(())
}

fn generate_contract_bidings() -> Result<(), Box<dyn std::error::Error>> {
    Abigen::new("NonRootTournament", "artifacts/NonRootTournament.json")?
        .generate()?
        .write_to_file("src/contract/tournament/non_root_tournament.rs")?;

    Abigen::new("RootTournament", "artifacts/RootTournament.json")?
        .generate()?
        .write_to_file("src/contract/tournament/root_tournament.rs")?;

    Abigen::new("NonLeafTournament", "artifacts/NonLeafTournament.json")?
        .generate()?
        .write_to_file("src/contract/tournament/non_leaf_tournament.rs")?;

    Abigen::new("LeafTournament", "artifacts/LeafTournament.json")?
        .generate()?
        .write_to_file("src/contract/tournament/leaf_tournament.rs")?;

    Abigen::new("Tournament", "artifacts/Tournament.json")?
        .generate()?
        .write_to_file("src/contract/tournament/tournament.rs")?;

    Abigen::new("TournamentFactory", "artifacts/TournamentFactory.json")?
        .generate()?
        .write_to_file("src/contract/factory/tournament_factory.rs")?;

    Ok(())
}