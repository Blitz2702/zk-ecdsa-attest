use criterion::{Criterion, black_box, criterion_group, criterion_main};
use k256::{
    ProjectivePoint, Scalar, U256,
    elliptic_curve::{Field, ops::Reduce, point::AffineCoordinates},
};
use rand::rngs::OsRng;
use sha2::{Digest, Sha256};
use zk_ecdsa_attest::{commitments::commit_to_public_key, generate_keys, prover, verifier};

#[allow(non_snake_case)]
fn benchmark_proof_generation(c: &mut Criterion) {
    // Get Pre-requisites
    let (d, Q) = generate_keys();
    let msg = b"Benchmark Payload";
    let msg_hash = <Scalar as Reduce<U256>>::reduce_bytes(&Sha256::digest(msg));

    // Create ECDSA Signature
    let k = Scalar::random(OsRng);
    let R = ProjectivePoint::GENERATOR * k;
    let r = <Scalar as Reduce<U256>>::reduce_bytes(&R.to_affine().x());
    let s = k.invert().unwrap() * (msg_hash + r * d);

    // Commit to Q
    let pk_commitment = commit_to_public_key(Q);

    let witness = prover::Witness {
        s,
        rho: pk_commitment.rho,
        r,
    };

    // Measure the Proof Generation
    c.bench_function("ZK-ECDSA Prove", |b| {
        b.iter(|| {
            // black_box prevents compiler from optimizing this away
            prover::generate_proof(
                black_box(&witness),
                black_box(R),
                black_box(Q),
                black_box(msg_hash),
            )
        })
    });
}

#[allow(non_snake_case)]
fn benchmark_verification(c: &mut Criterion) {
    // Setup...
    let (d, Q) = generate_keys();
    let msg = b"Benchmark_Payload";
    let msg_hash = <Scalar as Reduce<U256>>::reduce_bytes(&Sha256::digest(msg));
    let k = Scalar::random(OsRng);
    let R = ProjectivePoint::GENERATOR * k;
    let r = <Scalar as Reduce<U256>>::reduce_bytes(&R.to_affine().x());
    let s = k.invert().unwrap() * (msg_hash + r * d);
    let pk_commitment = commit_to_public_key(Q);
    let witness = prover::Witness {
        s,
        rho: pk_commitment.rho,
        r,
    };

    let proof = prover::generate_proof(&witness, R, pk_commitment.C, msg_hash);

    // Measure Verification
    c.bench_function("ZK-ECDSA Verify", |b| {
        b.iter(|| {
            verifier::verify_proof(
                black_box(&proof),
                black_box(msg_hash),
                black_box(R),
                black_box(Q),
            )
        })
    });
}

criterion_group!(benches, benchmark_proof_generation, benchmark_verification);
criterion_main!(benches);
