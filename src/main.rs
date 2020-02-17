
mod descriptor;

use bip39::{Mnemonic, MnemonicType, Language, Seed};

use bitcoin::hashes::hex::ToHex;
use bitcoin::hashes::Hash;
use bitcoin::hashes::sha256::Hash as Sha256;
use bitcoin::util::bip32::ExtendedPrivKey;
use bitcoin::network::constants::Network;

use clap::{Arg, App};
use crate::descriptor::descriptor_checksum;

enum Entropy {
    Dice(String),
    Raw(Vec<u8>)
}

fn get_entropy(args: &clap::ArgMatches) -> Option<Vec<u8>> {
    // TODO: entropy arg
    args.value_of("dice").as_ref().map(|x| {
        let rolls = x.as_bytes();
        let hashed : Sha256 = Hash::hash(rolls);
        let hashed_bytes : &[u8] = &hashed;
        println!("dice\t{}", x);
        hashed_bytes.to_vec()
    })
}

fn main() {
    let matches = App::new("bitcoin-seed-tool")
        .version("0.1.0")
        .author("William Casarin <jb55@jb55.com>")
        .about("Various bip39 utilities")
        .arg(Arg::with_name("dice")
             .short("d")
             .long("dice")
             .takes_value(true)
             .help("diceroll entropy, eg: 123456123. Equivalent to --entropy sha256(<diceroll string>)"))
        .arg(Arg::with_name("entropy")
             .short("e")
             .long("entropy")
             .takes_value(true)
             .help("Explicit entropy as a hexstring"))
        .get_matches();

    let entropy : &[u8] = &get_entropy(&matches).expect("expected --dice or --entropy");
    println!("entropy\t{}", entropy.to_hex());

    let mnemonic = Mnemonic::from_entropy(entropy, Language::English).unwrap();

    // get the phrase
    let phrase: &str = mnemonic.phrase();
    println!("phrase\t{}", phrase);

    // get the HD wallet seed
    let seed = Seed::new(&mnemonic, "");

    // print the HD wallet seed as a hex string
    println!("seed\t{:x}", seed);

    // TODO: testnet, liquid?
    let xprv = ExtendedPrivKey::new_master(Network::Bitcoin, seed.as_bytes()).unwrap();
    println!("xprv\t{}", xprv);

    let descriptor = format!("combo({}/0/*)", xprv);
    let checksum = descriptor_checksum(&descriptor).expect("unexpected error in descriptor_checksum");
    println!("descriptor\t{}#{}", descriptor, checksum);
}
