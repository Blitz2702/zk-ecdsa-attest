use k256::{
    ProjectivePoint, Scalar, U256,
    elliptic_curve::{Field, ops::Reduce, point::AffineCoordinates},
};
use rand::rngs::OsRng;
use sha2::{Digest, Sha256};
use zk_ecdsa_attest::{
    prover::{self, Witness},
    verifier,
};

#[allow(non_snake_case)]
fn main() {
    println!("--- ZK-ECDSA ATTESTATION PROTOCOL ---");

    let (d, Q) = zk_ecdsa_attest::generate_keys();
    let msg = "Zero Knowledge ECDSA Implmenetation";
    let msg_h = Sha256::digest(msg);
    let msg_hash = <Scalar as Reduce<U256>>::reduce_bytes(&msg_h);

    // Generate a ECDSA Signature
    println!("> Prover Signing Message ...");
    let k = Scalar::random(OsRng);
    let R = ProjectivePoint::GENERATOR * k;
    let r = <Scalar as Reduce<U256>>::reduce_bytes(&R.to_affine().x());
    let k_inv = k.invert().unwrap();
    let s = k_inv * (msg_hash + (r * d));

    // Create a witness
    let witness = Witness { s, r };

    // Generate a Proof
    println!("> Generating Proof ...");
    let proof = prover::generate_proof(&witness, R, Q, msg_hash);

    // Verify the Proof
    println!("> Verifying Proof...");
    let valid = verifier::verifiy_proof(&proof, msg_hash, R, Q);
    if valid {
        println!("✅ SUCCESS: Signature verified without revealing 's'!");
    } else {
        println!("❌ FAIL: Verification failed.");
    }
}
