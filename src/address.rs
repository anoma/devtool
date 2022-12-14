use namada::types::address::{Address, EstablishedAddressGen};
use rand::Rng;

use crate::keys::random_keypair;

pub fn generate_established() -> Address {
    let mut rng = rand::thread_rng();
    let seed: [u8; 32] = rng.gen();
    let seed = String::from_utf8_lossy(&seed);

    let mut generator = EstablishedAddressGen::new(&seed);

    let entropy: [u8; 32] = rng.gen();
    generator.generate_address(entropy)
}

pub fn generate_implicit() -> Address {
    let (_secret_key, public_key) = random_keypair();
    (&public_key).into()
}
