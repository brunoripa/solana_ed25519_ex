extern crate rand;
extern crate ed25519_dalek;

use rand::rngs::OsRng;
use ed25519_dalek::Keypair;
// use rustler::NifReturnable;

#[rustler::nif]
fn generate() -> Keypair {
    let mut csprng: OsRng = OsRng::new().unwrap();
    // the trait `Encoder` is not implemented for `Keypair`
    // Maybe this must be somehow serialized to be returned ?
    // Or myabe wrapped in `NifReturnable` ?
    return Keypair::generate(&mut csprng);
}

rustler::init!("Elixir.SolanaED", [generate]);