use std::sync::OnceLock;

use k256::{
    EncodedPoint, ProjectivePoint, Scalar,
    elliptic_curve::{Field, sec1::FromEncodedPoint},
};
use rand::rngs::OsRng;
use sha2::{Digest, Sha256};

pub mod commitments;
pub mod prover;
pub mod transcript;
pub mod verifier;

pub fn generate_keys() -> (Scalar, ProjectivePoint) {
    let sk = Scalar::random(OsRng);
    let pk = ProjectivePoint::GENERATOR * sk;
    (sk, pk)
}

static H_POINT_STORAGE: OnceLock<ProjectivePoint> = OnceLock::new();

pub fn generate_h_point() -> &'static ProjectivePoint {
    H_POINT_STORAGE.get_or_init(|| {
        println!("Initializing Generation of H_Point (using hash_to_curve implementation)");
        generate_second_generator()
    })
}

fn generate_second_generator() -> ProjectivePoint {
    let seed_string = "Pederson_Commitment_H_";
    let mut counter = 0u32;

    loop {
        let mut hasher = Sha256::new();
        hasher.update(seed_string.as_bytes());
        hasher.update(counter.to_be_bytes());
        let hash_result = hasher.finalize();

        let mut compressed_bytes = [0u8; 33];
        compressed_bytes[0] = 0x02;
        compressed_bytes[1..33].copy_from_slice(&hash_result);

        let encoded_point = match EncodedPoint::from_bytes(compressed_bytes) {
            Ok(pt) => pt,
            Err(_) => {
                counter += 1;
                continue;
            }
        };

        let projective_point = ProjectivePoint::from_encoded_point(&encoded_point);

        if projective_point.is_some().into() {
            return projective_point.unwrap();
        }

        counter += 1;

        if counter > 1_000_000 {
            panic!("Critical Error: Failed to find a valid curve point after 1M attempts.");
        }
    }
}
