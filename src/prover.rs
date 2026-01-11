use k256::{ProjectivePoint, Scalar, elliptic_curve::Field};
use rand::rngs::OsRng;

use crate::transcript::Transcript;

pub struct Witness {
    pub s: Scalar,
    pub r: Scalar,
}

#[allow(non_snake_case)]
pub struct Proof {
    pub commitment_T: ProjectivePoint,
    pub response_z: Scalar,
}

#[allow(non_snake_case)]
pub fn generate_proof(
    witness: &Witness,
    R: ProjectivePoint,
    Q: ProjectivePoint,
    msg_hash: Scalar,
) -> Proof {
    let alpha = Scalar::random(OsRng);
    let T = R * alpha;

    let mut hasher = Transcript::new(b"QmxpdHpMZWFyaW5nWktQ");
    hasher.append_point(&T);
    hasher.append_point(&R);
    hasher.append_point(&Q);
    hasher.append_scalar(&msg_hash);

    let c = hasher.retrive_challenge();

    let response_z = alpha + c * witness.s;

    Proof {
        commitment_T: T,
        response_z,
    }
}
