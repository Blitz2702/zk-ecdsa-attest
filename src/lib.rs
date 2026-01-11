use k256::{ProjectivePoint, Scalar, elliptic_curve::Field};
use rand::rngs::OsRng;

pub mod prover;
pub mod transcript;
pub mod verifier;

pub fn generate_keys() -> (Scalar, ProjectivePoint) {
    let sk = Scalar::random(OsRng);
    let pk = ProjectivePoint::GENERATOR * sk;

    (sk, pk)
}
