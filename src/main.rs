use k256::{
    ProjectivePoint, Scalar, U256,
    elliptic_curve::{Field, ops::Reduce, point::AffineCoordinates},
};
use rand::rngs::OsRng;
use sha2::{Digest, Sha256};
use zk_ecdsa_attest::{
    commitments::commit_to_public_key,
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

    // Generate Commitment to Public Key 'Q'
    let pk_commitment = commit_to_public_key(Q);

    // Create a witness
    let witness = Witness {
        s,
        rho: pk_commitment.rho,
    };

    // Generate a Proof
    println!("> Generating Proof ...");
    let proof = prover::generate_proof(&witness, R, pk_commitment.C, msg_hash);

    // Verify the Proof
    println!("> Verifying Proof...");
    let valid = verifier::verify_proof(&proof, msg_hash, R, pk_commitment.C);
    if valid {
        println!("✅ SUCCESS: Signature verified without revealing 's'!");
    } else {
        println!("❌ FAIL: Verification failed.");
    }
}
