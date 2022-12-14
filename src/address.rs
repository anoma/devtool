use namada::types::address::{Address, EstablishedAddressGen};
use rand::Rng;

pub fn generate_random() -> Address {
    let mut rng = rand::thread_rng();
    let seed: [u8; 32] = rng.gen();
    let seed = String::from_utf8_lossy(&seed);

    let mut generator = EstablishedAddressGen::new(&seed);

    let entropy: [u8; 32] = rng.gen();
    generator.generate_address(entropy)
}
