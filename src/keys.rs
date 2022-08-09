use namada::types::key::common::SecretKey;
use namada::types::key::{common, RefTo};
use rand::prelude::ThreadRng;

#[allow(dead_code)]
pub fn random_keypair() -> (SecretKey, common::PublicKey) {
    let mut rng: ThreadRng = rand::thread_rng();
    let sk: SecretKey = {
        use namada::types::key::{ed25519, SecretKey, SigScheme};
        ed25519::SigScheme::generate(&mut rng).try_to_sk().unwrap()
    };
    let sk_clone = sk.clone();
    (sk, sk_clone.ref_to())
}
