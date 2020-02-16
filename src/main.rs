
mod descriptor;

use bip39::{Mnemonic, MnemonicType, Language, Seed};

use bitcoin_hashes::sha256::Hash as sha256;

fn main() {

    // create a new randomly generated mnemonic phrase
    let mnemonic = Mnemonic::from_entropy(b"1234", Language::English).unwrap();

    // get the phrase
    let phrase: &str = mnemonic.phrase();
    println!("phrase: {}", phrase);

    // get the HD wallet seed
    let seed = Seed::new(&mnemonic, "");

    // get the HD wallet seed as raw bytes
    let seed_bytes: &[u8] = seed.as_bytes();

    // print the HD wallet seed as a hex string
    println!("{:X}", seed);
}
