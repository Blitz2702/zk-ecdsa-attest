use k256::{
    ProjectivePoint, Scalar, U256,
    elliptic_curve::{Field, ops::Reduce, point::AffineCoordinates, rand_core::OsRng},
};
use zk_ecdsa_attest::{commitments::commit_to_public_key, generate_keys, prover, verifier};

#[allow(non_snake_case)]
#[test]
fn test_end_to_end_flow() {
    // 1. Setup
    let (d, Q) = generate_keys();
    let z = Scalar::from(12345u64);

    // 2. Sign
    let k = Scalar::random(OsRng);
    let R = ProjectivePoint::GENERATOR * k;
    let r = <Scalar as Reduce<U256>>::reduce_bytes(&R.to_affine().x());
    let s = k.invert().unwrap() * (z + r * d);

    // 3. Hide Q
    let pk_commitment = commit_to_public_key(Q);

    // 3. Prove
    let witness = prover::Witness {
        s,
        rho: pk_commitment.rho,
    };
    let proof = prover::generate_proof(&witness, R, pk_commitment.C, z);

    // 4. Verify
    let valid = verifier::verify_proof(&proof, z, R, pk_commitment.C);
    assert!(valid);
}
