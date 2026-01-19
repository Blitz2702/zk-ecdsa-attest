use k256::{ProjectivePoint, Scalar, elliptic_curve::Field};
use rand::rngs::OsRng;

use crate::{generate_h_point, transcript::Transcript};

pub struct Witness {
    pub s: Scalar,
    pub rho: Scalar,
    pub r: Scalar,
}

#[allow(non_snake_case)]
pub struct Proof {
    pub commitment_T: ProjectivePoint,
    pub response_z_1: Scalar,
    pub response_z_2: Scalar,
}

#[allow(non_snake_case)]
pub fn generate_proof(
    witness: &Witness,
    R: ProjectivePoint,
    C_Q: ProjectivePoint,
    msg_hash: Scalar,
) -> Proof {
    let alpha_1 = Scalar::random(OsRng);
    let alpha_2 = Scalar::random(OsRng);
    let H = generate_h_point();
    let T = R * alpha_1 + (*H * alpha_2);

    let mut hasher = Transcript::new(b"QmxpdHpMZWFyaW5nWktQ");
    hasher.append_point(&T);
    hasher.append_point(&R);
    hasher.append_point(&C_Q);
    hasher.append_scalar(&msg_hash);

    let c = hasher.retrive_challenge();

    let response_z_1 = alpha_1 + c * witness.s;
    let response_z_2 = alpha_2 + c * witness.r * witness.rho;

    Proof {
        commitment_T: T,
        response_z_1,
        response_z_2,
    }
}
