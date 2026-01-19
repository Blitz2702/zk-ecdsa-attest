use k256::{ProjectivePoint, Scalar, elliptic_curve::Field};
use rand::rngs::OsRng;

use crate::generate_h_point;

#[allow(non_snake_case)]
pub struct PointCommitment {
    pub C: ProjectivePoint,
    pub rho: Scalar,
}

#[allow(non_snake_case)]
pub fn commit_to_public_key(pk: ProjectivePoint) -> PointCommitment {
    let rho = Scalar::random(OsRng);
    let H = generate_h_point();
    let C = pk + (*H * rho);

    PointCommitment { C, rho }
}
