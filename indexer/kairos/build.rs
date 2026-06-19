use cainome::rs::Abigen;
use std::collections::HashMap;

fn main() {
    // Aliases added from the ABI
    let mut aliases = HashMap::new();

    let kairos_abigen =
        Abigen::new("kairos", "./abi/kairos_contract.abi.json").with_types_aliases(aliases).with_derives(vec!["serde::Serialize".to_string(), "serde::Deserialize".to_string()]);

        kairos_abigen
            .generate()
            .expect("Fail to generate bindings")
            .write_to_file("./src/abi/kairos_contract.rs")
            .unwrap();
}