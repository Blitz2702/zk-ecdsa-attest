use k256::{
    ProjectivePoint, Scalar, U256,
    elliptic_curve::{ops::Reduce, point::AffineCoordinates},
};

use crate::{generate_h_point, prover::Proof, transcript::Transcript};

#[allow(non_snake_case)]
pub fn verify_proof(
    proof: &Proof,
    msg_hash: Scalar,
    R: ProjectivePoint,
    C_Q: ProjectivePoint,
) -> bool {
    let mut hasher = Transcript::new(b"QmxpdHpMZWFyaW5nWktQ");
    hasher.append_point(&proof.commitment_T);
    hasher.append_point(&R);
    hasher.append_point(&C_Q);
    hasher.append_scalar(&msg_hash);

    let c = hasher.retrive_challenge();

    let H = generate_h_point();

    let r = <Scalar as Reduce<U256>>::reduce_bytes(&R.to_affine().x());
    let public_target = (ProjectivePoint::GENERATOR * msg_hash) + (C_Q * r);

    let lhs = R * proof.response_z_1 + *H * proof.response_z_2;
    let rhs = proof.commitment_T + (public_target * c);

    lhs == rhs
}
