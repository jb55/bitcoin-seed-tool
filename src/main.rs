
mod descriptor;

use bip39::{Mnemonic, MnemonicType, Language, Seed};

use bitcoin_hashes::Hash;
use bitcoin_hashes::sha256::Hash as Sha256;

fn main() {

    // example diceroll entropy
    let entropy : Sha256 = Hash::hash(b"1234561234561346513412341436514356513246536543256421653");
    println!("entropy: {:x}", &entropy);

    let mnemonic = Mnemonic::from_entropy(&entropy, Language::English).unwrap();

    // get the phrase
    let phrase: &str = mnemonic.phrase();
    println!("phrase: {}", phrase);

    // get the HD wallet seed
    let seed = Seed::new(&mnemonic, "");

    // get the HD wallet seed as raw bytes
    // let seed_bytes: &[u8] = seed.as_bytes();

    // print the HD wallet seed as a hex string
    println!("{:X}", seed);
}
